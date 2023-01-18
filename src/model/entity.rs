use super::{Prototype, DescriptionTrait};

pub trait EntityTrait<'e, T: EntityTemplateTrait<'e>> {
    fn id(&self) -> u64;
    fn template(&self) -> Option<T>;
    fn permeability(&self) -> Option<Permeability>;
    fn description(&self) -> Prototype<EntityDescription>;
    fn components(&self) -> Prototype<T>;
    fn contents(&self) -> Prototype<Vec<Box<dyn EntityTrait<'e, T>>>>;
}

pub struct Entity<'e, T: EntityTemplateTrait<'e>> {
    id: u64,
    template: Option<T>,
    permeability: Option<Permeability>,
    description: Prototype<EntityDescription<'e>>,
    components: Prototype<T>,
    contains: Prototype<Vec<Box<dyn EntityTrait<'e,T>>>>
}

pub struct EntityDescription<'e> {
    name: &'e str
}

impl<'e, T: EntityTemplateTrait> DescriptionTrait<'e> for Entity<'e, T> {
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
    type ComponentTemplate;

    fn components(&self) -> Box<dyn Self::ComponentTemplate<'e>>;
}

pub struct EntityTemplate<'e> {
    components: ComponentModel<'e>
}

impl EntityTemplateTrait for EntityTemplate {
    fn model(&self) -> ComponentModel<'e> {
        self.model
    }
}

pub enum ComponentType<'e> {
    Entity (EntityComponent<'e, M: ComponentModel>),
    Logical (LogicalComponent<'e>)
}

pub enum NoComponents {}

pub struct NoComponentModel {

}

pub struct EntityComponent<'e, M: ComponentModel<'e>> {
    entity: Entity<'e, M>,
    subcomponents: Option<Box<Component>>
}

pub struct LogicalComponent<'e> {

}

pub trait ComponentModel<'e> {
    type Components;

    fn get(&self, component: Self::Components) -> Option<Box<&dyn EntityTrait<'e>>>;
}

impl<'e, M: ComponentModel<'e>> EntityTrait<'e> for Entity<'e, M> {
    fn name(&self) -> &'e str {
        self.name
    }
}

impl<'e> ComponentModel<'e> for NoComponentModel {
    type Components = NoComponents;

    fn get(&self, _component: Self::Components) -> Option<Box<&dyn EntityTrait<'e>>> {
        None
    }
}

pub trait Component<'e> {
    fn components(&self) -> Option<Box<dyn Component<'e>>>;
    fn get(&self) -> ComponentType<'e>;
}

impl<'e, M: ComponentModel<'e>> Component for EntityComponent<'e, M> {
}

impl<'e, M: ComponentModel<'e>> EntityComponent<'e, M> {
    pub fn entity(&self) -> &Entity<'e, M> {
        &self.entity
    }
}

