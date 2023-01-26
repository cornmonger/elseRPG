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

impl HumanoidComponentAlias {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Head => "head",
            Self::Back => "back"
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


impl<'e> HumanoidEntityTemplate {
    pub fn new_backpack(zone: &mut Zone) -> Entity<'e, EmptyEntityTemplate> {
        Entity {
            id: zone.generate_id(),
            template: Some(EmptyEntityTemplate {}),
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

