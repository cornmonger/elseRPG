use std::collections::HashMap;

pub use super::entity::Entity;
use super::entity::{EntityComponent, EntityCompositionTrait, UnrestrainedEntityComposition, EntityBuilder};
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
            head: EntityComponent::new(HumanoidPart::Head as isize, None, Some(
                EntityBuilder::new().id_zone(zone).description_name("Head").create()
            )),
            back: EntityComponent::new(HumanoidPart::Back as isize, None, Some(
                EntityBuilder::new().id_zone(zone).description_name("Back")
                    .attachments(Box::new(UnrestrainedEntityComposition::new([
                        ( HumanoidPart::Back as isize, EntityComponent::new(HumanoidPart::Back as isize, None, Some(
                            Humanoid::new_backpack(zone)) )) 
                    ].into_iter().collect())))
                    .create()
            ))
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
        EntityBuilder::new().id_zone(zone).description_name("Backpack")
            .contents(vec![Self::new_apple(zone)])
            .create()
    }

    pub fn new_apple(zone: &mut Zone) -> Entity {
        EntityBuilder::new().id_zone(zone).description_name("Apple").create()
    }

    pub fn new_player(zone: &mut Zone) -> Player {
        Player::new(EntityBuilder::new().id_zone(zone).description_name("Player")
            .permeability_max(100, 100, 100)
            .components(Box::new(HumanoidComposition::new(zone)))
            .create()
        )
    }
}

