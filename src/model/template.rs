pub use super::entity::Entity;
pub use super::entity::{EntityTemplateTrait, EntityTrait, Permeability, EntityDescription};
use super::composition::{Component, CompositionTrait, CompositionIteratorTrait, ComponentTrait};
use super::{Prototype, zone::{Zone, ZoneTrait}, character::{Player, Character, NPC}};
use strum::{EnumIter, IntoEnumIterator};

pub struct EmptyEntityTemplate {
    component_model: EmptyComponentModel
}

impl<'e> EntityTemplateTrait<'e> for EmptyEntityTemplate {
    type Composite = EmptyComponentModel;

    fn component_model(&self) -> &Self::Composite {
        &self.component_model
    }

}

pub enum EmptyComponentAlias {}

pub enum EmptyComponentSlot {}

pub struct EmptyComponentModel {}

impl<'e> CompositionTrait<'e> for EmptyComponentModel {
    type Alias = EmptyComponentAlias;
    type Slot = EmptyComponentSlot;

    fn component(&self, _alias: Self::Alias) -> Option<&Self::Slot> {
        None
    }
}

impl<'i,'e> CompositionIteratorTrait<'i,'e> for EmptyComponentModel {
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

pub struct Humanoid<'e> {
    component_model: HumanoidComposition<'e> 
}

impl<'e> EntityTemplateTrait<'e> for Humanoid<'e> {
    type Composite = HumanoidComposition<'e>;

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

pub enum HumanoidComponentSlot <'e> {
    Head (Component<Entity<'e, EmptyEntityTemplate>>),
    Back (Component<Entity<'e, EmptyEntityTemplate>>),
}

impl<'e> HumanoidComponentSlot<'e> {
    pub fn component<T>(&self) -> Box<&dyn EntityTrait<'e, T>> {
        match self {
            Self::Head(component) => Box::new(&component as &EntityTrait<'e, T>),
            Self::Back(component) => component
        }
    }
}

pub struct HumanoidComposition<'e> {
    head: Option<HumanoidComponentSlot<'e>>,
    back: Option<HumanoidComponentSlot<'e>>,
}

impl<'e> CompositionTrait<'e> for HumanoidComposition<'e> {
    type Alias = HumanoidPart;
    type Slot = HumanoidComponentSlot<'e>; 

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

impl <'i,'e: 'i> CompositionIteratorTrait<'i,'e> for HumanoidComposition<'e> {
    type Iterator = HumanoidIterator<'i,'e>;

    fn components(&'i self) -> Self::Iterator {
        HumanoidIterator {
            composition: self,
            enum_iter: HumanoidPart::iter(),
        }
        //HumanoidIterator::new(&self)
    }

}

pub struct HumanoidIterator<'i, 'e> {
    composition: &'i HumanoidComposition<'e>,
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

impl<'i, 'e> Iterator for HumanoidIterator<'i, 'e> {
    type Item = Option<&'i HumanoidComponentSlot<'e>>;

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


impl<'e> Humanoid<'e> {
    pub fn new_backpack(zone: &mut Zone) -> Entity<'e, EmptyEntityTemplate> {
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
            components: Prototype::None
        }
    }

    pub fn new_player(zone: &mut Zone) -> Player<'e, Self, HumanoidComposition<'e>> {
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
                    components: Prototype::Local(HumanoidComposition {
                        head: None,
                        back: Some(HumanoidComponentSlot::Back(Component { object: Self::new_backpack(zone) }))
                    })
                }
            }
        }
    }

    pub fn new_npc(zone: &mut Zone) -> NPC<'e, Self, HumanoidComposition<'e>> {
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
                    components: Prototype::None
                }
            }
        }
    }
}

