use super::{entity::{Entity, EntityTemplateTrait}};

pub struct Character<'e, M, T: EntityTemplateTrait<'e, Composite = M>> {
    pub entity: Entity<'e, T>,
}

pub struct NPC<'e, M, T: EntityTemplateTrait<'e, Composite = M>> {
    pub character: Character<'e, M, T>
}

pub struct Player<'e, M, T: EntityTemplateTrait<'e, Composite = M>> {
    pub character: Character<'e, M, T>
}