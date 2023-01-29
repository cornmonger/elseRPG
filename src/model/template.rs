use std::collections::HashMap;

pub use super::entity::Entity;
use super::entity::EntityComponent;
pub use super::entity::{EntityTrait, Permeability, EntityDescription};
use super::{zone::{Zone}, character::{Player, Character}};
use strum::{EnumIter};

pub struct Humanoid {
}

#[derive(Clone, Copy, EnumIter)]
pub enum HumanoidPart {
    Head = 1,
    Back = 2,
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

impl Humanoid {
    pub fn new_backpack(zone: &mut Zone) -> Entity {
        Entity {
            id: zone.generate_id(),
            description: Some(EntityDescription { name: "Backpack".to_owned() }),
            permeability: None, 
            contents: Some(Vec::new()),
            components: None
        }
    }

    fn new_composition(zone: &mut Zone) -> HashMap<isize, EntityComponent> {
        let mut map = HashMap::<isize, EntityComponent>::new();
        map.insert(HumanoidPart::Back as isize, EntityComponent { entity: Some( Entity {
            id: zone.generate_id(),
            description: Some(EntityDescription { name: "Klondike Backpack".to_owned() }),
            permeability: None,
            contents: None,
            components: None
        })});

        map
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
                    contents: None,
                    components: Some(Self::new_composition(zone)),
                }
            }
        }
    }
}

