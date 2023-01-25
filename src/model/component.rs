use crate::model::Prototype;

pub trait ComponentTrait {
    type Type;

    fn get(&self) -> &Self::Type;
}

pub struct Component<T> {
    object: T
}

impl<T> ComponentTrait for Component<T> {
    type Type = T;

    fn get(&self) -> &Self::Type {
        &self.object
    }
}

pub trait ComponentModelTrait {
    type AliasEnum; 
    type Type;

    fn component(&self, alias: Self::AliasEnum) -> Prototype<&Component<Self::Type>>;
}

/*
pub enum ComponentType<'e> {
    Entity (EntityComponent<'e, M: ComponentTemplate>),
    Logical (LogicalComponent<'e>)
}

pub enum NoComponents {}

pub struct NoComponentModel {

}

pub struct EntityComponent<'e, M: ComponentTemplate<'e>> {
    entity: Entity<'e, M>,
    subcomponents: Option<Box<ComponentTrait>>
}

pub struct LogicalComponent<'e> {

}

pub trait ComponentTemplate<'e> {
    type Components;

    fn get(&self, component: Self::Components) -> Option<Box<&dyn EntityTrait<'e>>>;
}

impl<'e, M: ComponentTemplate<'e>> EntityTrait<'e> for Entity<'e, M> {
    fn name(&self) -> &'e str {
        self.name
    }
}

impl<'e> ComponentTemplate<'e> for NoComponentModel {
    type Components = NoComponents;

    fn get(&self, _component: Self::Components) -> Option<Box<&dyn EntityTrait<'e>>> {
        None
    }
}

pub trait ComponentTrait<'e> {
    fn components(&self) -> Option<Box<dyn ComponentTrait<'e>>>;
    fn get(&self) -> ComponentType<'e>;
}

impl<'e, M: ComponentTemplate<'e>> ComponentTrait for EntityComponent<'e, M> {
}

impl<'e, M: ComponentTemplate<'e>> EntityComponent<'e, M> {
    pub fn entity(&self) -> &Entity<'e, M> {
        &self.entity
    }
}
*/