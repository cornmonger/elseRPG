use std::collections::HashMap;

pub use super::entity::Entity;
use super::entity::{EntityComponent, EntityCompositionTrait, UnrestrainedEntityComposition};
pub use super::entity::{EntityTrait, Permeability, EntityDescription};
use super::{zone::{Zone}, character::{Player, Character}};

pub struct Humanoid {
}

#[derive(Clone, Copy)]
pub enum HumanoidPart {
    Head = 1,
    Back = 2,
}

pub mod humanoid {
    pub enum Component {
        Head = 1,
        Torso = 2,
        Waist = 3,
        Legs = 4,
    }
    pub mod head {
        pub enum Attachment {
            WearOn = 1
        }
    }
    pub mod torso {
        pub enum Component {
            Back = 1,
            Chest = 2,
        }
        pub enum Attachment {
            WearOn = 1
        }
        pub mod back {
            pub enum Attachment {
                StrapTo = 1,
            }
        }
    }
    pub mod legs {
        pub enum Component {
            Left = 1,
            Right = 2
        }
        pub enum Attachment {
            WearOn = 1
        }
        pub mod leg {
            pub enum Component {
                Shin = 1,
                Foot = 2,
            }
        }
    }
}

pub struct HumanoidComposition {
    head: EntityComponent,
    back: EntityComponent
}

impl EntityCompositionTrait for HumanoidComposition {
    fn get(&self, key: isize) -> Result<&EntityComponent, ()> {
        match key {
            1 => Ok(&self.head),
            2 => Ok(&self.back),
            _ => Err(())
        }
    }

    fn iter(&self) -> std::vec::IntoIter<&EntityComponent> {
        vec![&self.head, &self.back].into_iter()
    }
}
    
impl HumanoidComposition {
    pub fn new(zone: &mut Zone) -> HumanoidComposition {
        HumanoidComposition {
            head: EntityComponent { key: HumanoidPart::Head as isize, entity: Some( Entity {
                id: zone.generate_id(),
                description: Some(EntityDescription { name: "Head".to_owned() }),
                permeability: None,
                components: None,
                attachments: None,
                contents: None,
            })},
            back: EntityComponent { key: HumanoidPart::Back as isize, entity: Some( Entity {
                id: zone.generate_id(),
                description: Some(EntityDescription { name: "Back".to_owned() }),
                permeability: None,
                components: None,
                attachments: Some(Box::new(UnrestrainedEntityComposition::new([
                    (HumanoidPart::Back as isize, EntityComponent { key: HumanoidPart::Back as isize, entity: Some(Humanoid::new_backpack(zone)) })
                ].into_iter().collect()))),
                contents: None,
            })}
        }
    }

    pub fn by(&self, alias: HumanoidPart) -> &EntityComponent {
        match alias {
            HumanoidPart::Head => &self.head,
            HumanoidPart::Back => &self.back
        }
    }
}

impl Humanoid {
    pub fn new_backpack(zone: &mut Zone) -> Entity {
        Entity {
            id: zone.generate_id(),
            description: Some(EntityDescription { name: "Backpack".to_owned() }),
            permeability: None, 
            components: None,
            attachments: None,
            contents: Some(vec![Self::new_apple(zone)]),
        }
    }

    pub fn new_apple(zone: &mut Zone) -> Entity {
        Entity {
            id: zone.generate_id(),
            description: Some(EntityDescription { name: "Apple".to_owned() }),
            permeability: None, 
            components: None,
            attachments: None,
            contents: None,
        }
    }

    pub fn new_player(zone: &mut Zone) -> Player {
        Player {
            character: Character {
                entity: Entity {
                    id: zone.generate_id(),
                    description: Some(EntityDescription {
                        name: "Player".to_owned()
                    }),
                    permeability: Some(Permeability {
                        max_health: 100,
                        max_resist: 100,
                        max_ability: 100,
                        health: 100,
                        resist: 100,
                        ability: 100
                    }),
                    components: Some(Box::new(HumanoidComposition::new(zone))),
                    attachments: None,
                    contents: None,
                }
            }
        }
    }
}

