use super::entity::EntityRef;

pub struct Character {
    pub(crate) entity: EntityRef,
}

impl Character {
    pub fn entity(&self) -> &EntityRef {
        &self.entity
    }
}

pub struct Player {
    pub(crate) character: Character
}

impl Player {
    pub fn new (entity: EntityRef) -> Self {
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

