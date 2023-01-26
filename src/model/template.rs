pub use super::entity::Entity;
pub use super::entity::{EntityTemplateTrait, EntityTrait, Permeability, EntityDescription};
use super::component::{Component, ComponentModelTrait};
use super::{Prototype, zone::{Zone, ZoneTrait}, character::{Player, Character, NPC}};
use strum::EnumIter;

pub struct HumanoidEntityTemplate<'e> {
    component_model: HumanoidComponentModel<'e> 
}

impl<'e> EntityTemplateTrait<'e> for HumanoidEntityTemplate<'e> {
    type ComponentModel = HumanoidComponentModel<'e>;

    fn component_model(&self) -> &Self::ComponentModel {
        &self.component_model
    }
}

#[derive(Clone, Copy, EnumIter)]
pub enum HumanoidComponentAlias {
    Head,
    Back,
}

impl HumanoidComponentAlias {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Head => "head",
            Self::Back => "back"
        }
    }

    pub fn by(name: &str) -> Result<HumanoidComponentAlias, ()> {
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

pub struct HumanoidComponentModel<'e> {
    head: Option<HumanoidComponentSlot<'e>>,
    back: Option<HumanoidComponentSlot<'e>>,
}

impl<'e> ComponentModelTrait for HumanoidComponentModel<'e> {
    type Alias = HumanoidComponentAlias;
    type Slot = HumanoidComponentSlot<'e>; 

    fn component(&self, alias: Self::Alias) -> Option<&Self::Slot> {
        let slot = { match alias {
            HumanoidComponentAlias::Head => &self.head,
            HumanoidComponentAlias::Back => &self.back
        } };

        if let Some(component) = slot {
            Some(&component)
        } else {
            None
        }
    }

    /*fn aliases(&self) -> &str {
        let it = Alias::iter();
    }*/
}


pub struct EmptyEntityTemplate {
    component_model: EmptyComponentModel
}

impl<'e> EntityTemplateTrait<'e> for EmptyEntityTemplate {
    type ComponentModel = EmptyComponentModel;

    fn component_model(&self) -> &Self::ComponentModel {
        &self.component_model
    }

}

pub enum EmptyComponentAlias {}

pub enum EmptyComponentSlot {}

pub struct EmptyComponentModel {

}

impl ComponentModelTrait for EmptyComponentModel {
    type Alias = EmptyComponentAlias;
    type Slot = EmptyComponentSlot;

    fn component(&self, _alias: Self::Alias) -> Option<&Self::Slot> {
        None
    }
}


impl<'e> HumanoidEntityTemplate<'e> {
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

    pub fn new_player(zone: &mut Zone) -> Player<'e, HumanoidComponentModel<'e>, Self> {
        Player {
            character: Character {
                entity: Entity {
                    id: zone.generate_id(),
                    template: Some(Self { component_model: HumanoidComponentModel { head: None, back: None } }),
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
                    components: Prototype::Local(HumanoidComponentModel {
                        head: None,
                        back: Some(HumanoidComponentSlot::Back(Component { object: Self::new_backpack(zone) }))
                    })
                }
            }
        }
    }

    pub fn new_npc(zone: &mut Zone) -> NPC<'e, HumanoidComponentModel<'e>, Self> {
        NPC {
            character: Character {
                entity: Entity {
                    id: zone.generate_id(),
                    template: Some(Self { component_model: HumanoidComponentModel { head: None, back: None } }),
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

