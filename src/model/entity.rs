use super::{Prototype, DescriptionTrait};

pub trait EntityTrait<'e, T: EntityTemplateTrait<'e>> {
    fn id(&self) -> u64;
    fn template(&self) -> &Option<T>;
    fn permeability(&self) -> &Prototype<Permeability>;
    fn description(&self) -> &Prototype<EntityDescription>;
    fn components(&self) -> &Prototype<T::ComponentModel>;
    fn contents(&self) -> &Prototype<Vec<Box<dyn EntityTrait<'e, T>>>>;
}

pub struct Entity<'e, T: EntityTemplateTrait<'e>> {
    pub(crate) id: u64,
    pub(crate) template: Option<T>,
    pub(crate) permeability: Prototype<Permeability>,
    pub(crate) description: Prototype<EntityDescription<'e>>,
    pub(crate) components: Prototype<T::ComponentModel>,
    pub(crate) contents: Prototype<Vec<Box<dyn EntityTrait<'e,T>>>>
}

impl<'e, T: EntityTemplateTrait<'e>> EntityTrait<'e, T> for Entity<'e, T> {
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

    fn components(&self) -> &Prototype<T::ComponentModel> {
        &self.components
    }

    fn contents(&self) -> &Prototype<Vec<Box<dyn EntityTrait<'e, T>>>> {
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

pub trait EntityTemplateTrait<'e> {
    type ComponentModel;

    fn component_model(&self) -> &Self::ComponentModel;
    //fn permeability(&self) -> &Permeability;
    //fn description(&self) -> &EntityDescription;
}