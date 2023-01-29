use std::collections::HashMap;

pub use super::entity::Entity;
use super::entity::{EntityComponent, EntityCompositionTrait, UnrestrainedEntityComposition};
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
                componentz: None,
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
                componentz: None,
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
            componentz: None,
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
            componentz: None
        }
    }

    fn new_composition(zone: &mut Zone) -> HashMap<isize, EntityComponent> {
        let mut map = HashMap::<isize, EntityComponent>::new();
        map.insert(HumanoidPart::Head as isize, EntityComponent { key: HumanoidPart::Head as isize, entity: Some( Entity {
            id: zone.generate_id(),
            description: Some(EntityDescription { name: "Head".to_owned() }),
            permeability: None,
            components: None,
            attachments: None,
            contents: None,
            componentz: None,
        })});

        map.insert(HumanoidPart::Back as isize, EntityComponent { key: HumanoidPart::Back as isize, entity: Some( Entity {
            id: zone.generate_id(),
            description: Some(EntityDescription { name: "Back".to_owned() }),
            permeability: None,
            components: None,
            attachments: Some(Box::new(UnrestrainedEntityComposition::new([
                (HumanoidPart::Back as isize, EntityComponent { key: HumanoidPart::Back as isize, entity: Some(Self::new_backpack(zone)) })
            ].into_iter().collect()))),
            contents: None,
            componentz: None,
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
                    components: Some(Self::new_composition(zone)),
                    attachments: None,
                    contents: None,
                    componentz: Some(Box::new(HumanoidComposition::new(zone)))
                }
            }
        }
    }
}

