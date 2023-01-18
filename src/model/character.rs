pub struct Character<'e, M: ComponentModel<'e>> {
    entity: Entity<'e, M>,
}

pub struct NPC<'e, M: ComponentModel<'e>> {
    character: Character<'e, M>
}

pub struct Player<'e, M: ComponentModel<'e>> {
    character: Character<'e, M>
}

impl<'e, M: ComponentModel<'e>> Player<'e, M> {
    pub fn name(&self) -> &'e str {
        self.character.entity.name
    }
}

impl<'e, M: ComponentModel<'e>> NPC<'e, M> {
    pub fn name(&self) -> &'e str {
        self.character.entity.name
    }
}