use super::{entity::{Entity, EntityTemplateTrait}};

trait CharacterTrait {

}

pub struct Character<'e:'i,'i, T: EntityTemplateTrait<'e,'i, Composite = M>, M> {
    pub entity: Entity<'e,'i, T>,
}

impl<'e:'i,'i, T: EntityTemplateTrait<'e,'i, Composite = M>, M> Character<'e,'i, T, M> {
    pub fn entity(&'i self) -> &'e Entity<'e,'i,T> {
        &self.entity
    }
}

pub struct NPC<'e:'i,'i, T: EntityTemplateTrait<'e,'i, Composite = M>, M> {
    pub character: Character<'e,'i, T, M>
}

pub struct Player<'e:'i,'i, T: EntityTemplateTrait<'e,'i, Composite = M>, M> {
    pub character: Character<'e,'i, T, M>
}

impl<'e:'i,'i, T: EntityTemplateTrait<'e,'i, Composite = M>, M> Player<'e,'i, T, M> {
    pub fn character(&'i self) -> &'e Character<'e,'i,T,M> {
        &self.character
    }
}

