pub use super::entity::Entity;
pub use super::entity::{EntityTemplateTrait, EntityTrait, Permeability, EntityDescription};
use super::component::{Component, ComponentModelTrait};
use super::{Prototype, zone::{Zone, ZoneTrait}, character::{Player, Character, NPC}};

pub struct HumanoidEntityTemplate {

}

impl<'e> EntityTemplateTrait<'e> for HumanoidEntityTemplate {
    type ComponentModel = HumanoidComponentModel<'e>;
}

pub enum HumanoidComponentAlias {
    Head,
    Back,
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
            Some(&component);
        }

        None
    }
}


pub struct EmptyEntityTemplate {

}

impl<'e> EntityTemplateTrait<'e> for EmptyEntityTemplate {
    type ComponentModel = EmptyComponentModel;

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

/* impl<'e> Attachable<'e, HumanoidModel<'e>, HumanoidComponents> for Player<'e, HumanoidModel<'e>>  {
    fn attached(&self, component: HumanoidComponents) -> Option<Box<&dyn EntityTrait<'e>>> {
        self.character.entity.components.as_ref().unwrap().get(component)
    }
} */



/* pub struct HumanoidModel<'e> {
    head: Option<Entity<'e, NoComponentModel>>,
    back: Option<Entity<'e, NoComponentModel>>
}

pub enum HumanoidComponents {
    Head,
    Back 
}


impl<'e> ComponentModel<'e> for HumanoidModel<'e> {
    type Components = HumanoidComponents;

    fn get(&self, component: Self::Components) -> Option<Box<&dyn EntityTrait<'e>>> {
        match component {
            HumanoidComponents::Head => Some(Box::new(self.head.as_ref().unwrap())),
            HumanoidComponents::Back => Some(Box::new(self.back.as_ref().unwrap()))
        }
    }
}impl HumanoidComponents {
    pub fn name(&self) -> &'static str {
        match self {
            HumanoidComponents::Head => "head",
            HumanoidComponents::Back => "back"
        }
    }
} */


impl<'e> HumanoidEntityTemplate {
    pub fn new_player(zone: &mut Zone) -> Player<'e, HumanoidComponentModel<'e>, Self> {
        Player {
            character: Character {
                entity: Entity {
                    id: zone.generate_id(),
                    template: Some(Self {}),
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
                    components: Prototype::None
                }
            }
        }
    }

    pub fn new_npc(zone: &mut Zone) -> NPC<'e, HumanoidComponentModel<'e>, Self> {
        NPC {
            character: Character {
                entity: Entity {
                    id: zone.generate_id(),
                    template: Some(Self {}),
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

