use super::{entity::{Entity}};

pub struct Character {
    pub(crate) entity: Entity,
}

impl Character {
    pub fn entity(&self) -> &Entity {
        &self.entity
    }
}

pub struct Player {
    pub(crate) character: Character
}

impl Player {
    pub fn new (entity: Entity) -> Self {
        Player {
            character: Character {
                entity
            }
        }
    }

    pub fn character(&self) -> &Character {
        &self.character
    }
}

