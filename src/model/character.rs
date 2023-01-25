use super::{component::ComponentModelTrait, entity::{Entity, EntityTemplateTrait}};

pub struct Character<'e, M, T: EntityTemplateTrait<'e, ComponentModel = M>> {
    entity: Entity<'e, T>,
}

pub struct NPC<'e, M, T: EntityTemplateTrait<'e, ComponentModel = M>> {
    character: Character<'e, M, T>
}

pub struct Player<'e, M, T: EntityTemplateTrait<'e, ComponentModel = M>> {
    character: Character<'e, M, T>
}