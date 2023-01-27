use super::{entity::{Entity, EntityTemplateTrait}};

trait CharacterTrait {
    
}

pub struct Character<'e, T: EntityTemplateTrait<'e, Composite = M>, M> {
    pub entity: Entity<'e, T>,
}

pub struct NPC<'e, T: EntityTemplateTrait<'e, Composite = M>, M> {
    pub character: Character<'e, T, M>
}

pub struct Player<'e, T: EntityTemplateTrait<'e, Composite = M>, M> {
    pub character: Character<'e, T, M>
}