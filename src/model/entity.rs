use super::{Prototype, DescriptionTrait, composition::CompositionTrait};

pub trait EntityTrait<'e:'i,'i, T: EntityTemplateTrait<'e,'i>> {
    fn id(&self) -> u64;
    fn template(&self) -> &Option<T>;
    fn permeability(&self) -> &Prototype<Permeability>;
    fn description(&self) -> &Prototype<EntityDescription>;
    fn composition(&self) -> &Prototype<T::Composite>;
    fn contents(&self) -> &Prototype<Vec<Box<dyn EntityTrait<'e,'i, T>>>>;
}

pub struct Entity<'e:'i,'i, T: EntityTemplateTrait<'e,'i>> {
    pub(crate) id: u64,
    pub(crate) template: Option<T>,
    pub(crate) permeability: Prototype<Permeability>,
    pub(crate) description: Prototype<EntityDescription<'e>>,
    pub(crate) composition: Prototype<T::Composite>,
    pub(crate) contents: Prototype<Vec<Box<dyn EntityTrait<'e,'i, T>>>>
}

impl<'e, 'i, T: EntityTemplateTrait<'e,'i>> EntityTrait<'e,'i, T> for Entity<'e,'i, T> {
    fn id(&self) -> u64 {
        self.id
    }

    fn template(&self) -> &Option<T> {
        &self.template
    }

    fn permeability(&self) -> &Prototype<Permeability> {
        &self.permeability
    }

    fn description(&self) -> &Prototype<EntityDescription> {
        &self.description
    }

    fn composition(&self) -> &Prototype<T::Composite> {
        &self.composition
    }

    fn contents(&self) -> &Prototype<Vec<Box<dyn EntityTrait<'e,'i, T>>>> {
        &self.contents
    }
}

pub struct EntityDescription<'e> {
    pub(crate) name: &'e str
}

impl<'e> DescriptionTrait<'e> for EntityDescription<'e> {
    fn name(&self) -> &'e str {
        self.name
    }
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
    pub(crate) max_health: u16,
    pub(crate) health: u16,
    pub(crate) max_resist: u16,
    pub(crate) resist: u16,
    pub(crate) ability: u16,
    pub(crate) max_ability: u16,
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

pub trait EntityTemplateTrait<'e:'i,'i> {
    type Composite: CompositionTrait<'e, 'i>;

    fn component_model(&self) -> &Self::Composite;
    //fn permeability(&self) -> &Permeability;
    //fn description(&self) -> &EntityDescription;
}