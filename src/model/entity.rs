use super::{Prototype, DescriptionTrait};

pub trait EntityTrait<'e, T: EntityTemplateTrait<'e>> {
    fn id(&self) -> u64;
    fn template(&self) -> Option<T>;
    fn permeability(&self) -> Option<Permeability>;
    fn description(&self) -> Prototype<EntityDescription>;
    fn components(&self) -> Prototype<T::ComponentModel>;
    fn contents(&self) -> Prototype<Vec<Box<dyn EntityTrait<'e, T>>>>;
}

pub struct Entity<'e, T: EntityTemplateTrait<'e>> {
    id: u64,
    template: Option<T>,
    permeability: Option<Permeability>,
    description: Prototype<EntityDescription<'e>>,
    components: Prototype<T::ComponentModel>,
    contains: Prototype<Vec<Box<dyn EntityTrait<'e,T>>>>
}

pub struct EntityDescription<'e> {
    name: &'e str
}

pub trait PermeabilityTrait {
    fn max_health(&self) -> u16;
    fn max_resist(&self) -> u16;
    fn max_ability(&self) -> u16;
    fn health(&self) -> u16;
    fn resist(&self) -> u16;
    fn ability(&self) -> u16;
}

pub struct Permeability {
    max_health: u16,
    health: u16,
    max_resist: u16,
    resist: u16,
    ability: u16,
    max_ability: u16,
}

impl PermeabilityTrait for Permeability {
    fn max_health(&self) -> u16 {
        self.max_health
    }

    fn max_resist(&self) -> u16 {
        self.max_resist
    }

    fn max_ability(&self) -> u16 {
        self.max_ability
    }

    fn health(&self) -> u16 {
        self.health
    }

    fn resist(&self) -> u16 {
        self.resist
    }

    fn ability(&self) -> u16 {
        self.ability
    }
}

pub trait EntityTemplateTrait<'e> {
    type ComponentModel;

    //fn components(&self) -> Self::ComponentModel;
    //fn permeability(&self) -> &Permeability;
    //fn description(&self) -> &EntityDescription;
}