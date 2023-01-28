pub use super::entity::Entity;
pub use super::entity::{EntityTemplateTrait, EntityTrait, Permeability, EntityDescription};
use super::composition::{Component, CompositionTrait, CompositionIteratorTrait, ComponentTrait};
use super::{Prototype, zone::{Zone, ZoneTrait}, character::{Player, Character, NPC}};
use strum::{EnumIter, IntoEnumIterator};

pub struct EmptyEntityTemplate {
    component_model: EmptyComponentModel
}

impl<'e:'i,'i> EntityTemplateTrait<'e,'i> for EmptyEntityTemplate {
    type Composite = EmptyComponentModel;

    fn component_model(&self) -> &Self::Composite {
        &self.component_model
    }

}

pub enum EmptyComponentAlias {}

pub enum EmptyComponentSlot {}

pub struct EmptyComponentModel {}

impl<'e:'i,'i> CompositionTrait<'e,'i> for EmptyComponentModel {
    type Alias = EmptyComponentAlias;
    type Slot = EmptyComponentSlot;

    fn component(&self, _alias: Self::Alias) -> Option<&Self::Slot> {
        None
    }
}

impl<'e:'i, 'i> CompositionIteratorTrait<'e,'i> for EmptyComponentModel {
    type Iterator = EmptyCompositionIterator;

    fn components(&self) -> Self::Iterator {
        EmptyCompositionIterator {}
    }
}

pub struct EmptyCompositionIterator {}

impl Iterator for EmptyCompositionIterator {
    type Item = EmptyCompositionIterator;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub struct Humanoid<'e:'i,'i> {
    component_model: HumanoidComposition<'e,'i> 
}

impl<'e:'i, 'i> EntityTemplateTrait<'e,'i> for Humanoid<'e,'i> {
    type Composite = HumanoidComposition<'e,'i>;

    fn component_model(&self) -> &Self::Composite {
        &self.component_model
    }
}

#[derive(Clone, Copy, EnumIter)]
pub enum HumanoidPart {
    Head,
    Back,
}

impl HumanoidPart {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Head => "head",
            Self::Back => "back"
        }
    }

    pub fn by(name: &str) -> Result<HumanoidPart, ()> {
        match name {
            "head" => Ok(Self::Head),
            "back" => Ok(Self::Back),
            _ => Err(())
        }
    }
}

pub enum HumanoidComponentSlot <'e:'i,'i> {
    Head (Component<Entity<'e,'i, EmptyEntityTemplate>>),
    Back (Component<Entity<'e,'i, EmptyEntityTemplate>>),
}

impl<'e:'i,'i> HumanoidComponentSlot<'e,'i> {
    /* pub fn component<T>(&self) -> Box<&dyn EntityTrait<'e, T>> {
        /* match self {
            Self::Head(component) => Box::new(&component as &dyn EntityTrait<'e, T>),
            Self::Back(component) => component
        } */
    } */
}

pub struct HumanoidComposition<'e:'i,'i> {
    head: Option<HumanoidComponentSlot<'e,'i>>,
    back: Option<HumanoidComponentSlot<'e,'i>>,
}

impl<'e: 'i, 'i> CompositionTrait<'e,'i> for HumanoidComposition<'e,'i> {
    type Alias = HumanoidPart;
    type Slot = HumanoidComponentSlot<'e,'i>; 

    fn component(&self, alias: Self::Alias) -> Option<&Self::Slot> {
        let slot = { match alias {
            HumanoidPart::Head => &self.head,
            HumanoidPart::Back => &self.back
        } };

        if let Some(component) = slot {
            Some(&component)
        } else {
            None
        }
    }

}

impl <'e: 'i, 'i> CompositionIteratorTrait<'e, 'i> for HumanoidComposition<'e,'i> {
    type Iterator = HumanoidIterator<'e,'i>;

    fn components(&'i self) -> Self::Iterator {
        HumanoidIterator {
            composition: self,
            enum_iter: HumanoidPart::iter(),
        }
        //HumanoidIterator::new(&self)
    }

}

pub struct HumanoidIterator<'e:'i, 'i> {
    composition: &'i HumanoidComposition<'e,'i>,
    enum_iter: HumanoidPartIter,
}

/* impl<'e> HumanoidIterator<'e> {
    pub(crate) fn new(composition: &'e HumanoidComposition<'e>) -> HumanoidIterator<'e> {
        HumanoidIterator {
            composition,
            enum_iter: HumanoidPart::iter(),
        }
    }
} */

impl<'e:'i, 'i> Iterator for HumanoidIterator<'e,'i> {
    type Item = Option<&'i HumanoidComponentSlot<'e,'i>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_alias= self.enum_iter.next();
        if let Some(alias) = next_alias {
            println!("next {}", alias.name());
            Some(self.composition.component(alias))
        } else {
            None
        }
    }
}


impl<'e:'i,'i> Humanoid<'e,'i> {
    pub fn new_backpack(zone: &mut Zone) -> Entity<'e,'i, EmptyEntityTemplate> {
        Entity {
            id: zone.generate_id(),
            template: Some(EmptyEntityTemplate { component_model: EmptyComponentModel {} }),
            description: Prototype::Local(EntityDescription {
                name: "Backpack"
            }),
            permeability: Prototype::Local(Permeability {
                max_health: 10,
                max_resist: 10,
                max_ability: 10,
                health: 10,
                resist: 10,
                ability: 10
            }),
            contents: Prototype::None,
            composition: Prototype::None
        }
    }

    pub fn new_player(zone: &mut Zone) -> Player<'e,'i, Self, HumanoidComposition<'e,'i>> {
        Player {
            character: Character {
                entity: Entity {
                    id: zone.generate_id(),
                    template: Some(Self { component_model: HumanoidComposition { head: None, back: None } }),
                    description: Prototype::Local(EntityDescription {
                        name: "Player"
                    }),
                    permeability: Prototype::Local(Permeability {
                        max_health: 100,
                        max_resist: 100,
                        max_ability: 100,
                        health: 100,
                        resist: 100,
                        ability: 100
                    }),
                    contents: Prototype::None,
                    composition: Prototype::Local(HumanoidComposition {
                        head: None,
                        back: Some(HumanoidComponentSlot::Back(Component { object: Self::new_backpack(zone) }))
                    })
                }
            }
        }
    }

    pub fn new_npc(zone: &mut Zone) -> NPC<'e,'i, Self, HumanoidComposition<'e,'i>> {
        NPC {
            character: Character {
                entity: Entity {
                    id: zone.generate_id(),
                    template: Some(Self { component_model: HumanoidComposition { head: None, back: None } }),
                    description: Prototype::Local(EntityDescription {
                        name: "Troll"
                    }),
                    permeability: Prototype::Local(Permeability {
                        max_health: 50,
                        max_resist: 50,
                        max_ability: 50,
                        health: 50,
                        resist: 50,
                        ability: 50 
                    }),
                    contents: Prototype::None,
                    composition: Prototype::None
                }
            }
        }
    }
}

