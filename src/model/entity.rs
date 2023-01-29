use std::collections::HashMap;

use super::{DescriptionTrait};

pub trait EntityTrait {
    fn id(&self) -> u64;
    fn permeability(&self) -> Option<&Permeability>;
    fn description(&self) -> Option<&EntityDescription>;
    fn components(&self) -> Option<&HashMap<isize, EntityComponent>>;
    fn component(&self, key: isize) -> Option<&EntityComponent>;
    fn attachments(&self) -> Option<&HashMap<isize, EntityComponent>>;
    fn attachment(&self, key: isize) -> Option<&EntityComponent>;
    fn contents(&self) -> Option<&Vec<Entity>>;
    fn componentz(&self) -> Option<&Box<dyn EntityCompositionTrait>>;
    fn compozent(&self, key: isize) -> Result<&EntityComponent, ()>;
}

pub struct Entity {
    pub(crate) id: u64,
    pub(crate) permeability: Option<Permeability>,
    pub(crate) description: Option<EntityDescription>,
    pub(crate) components: Option<HashMap<isize, EntityComponent>>,
    pub(crate) attachments: Option<HashMap<isize, EntityComponent>>,
    pub(crate) contents: Option<Vec<Entity>>,
    pub(crate) componentz: Option<Box<dyn EntityCompositionTrait>>
}

impl EntityTrait for Entity {
    fn id(&self) -> u64 {
        self.id
    }

    fn permeability(&self) -> Option<&Permeability> {
        self.permeability.as_ref()
    }

    fn description(&self) -> Option<&EntityDescription> {
        self.description.as_ref()
    }

    fn components(&self) -> Option<&HashMap<isize, EntityComponent>> {
        self.components.as_ref()
    }

    fn component(&self, key: isize) -> Option<&EntityComponent> {
        if let Some(components) = &self.components {
            components.get(&key)
        } else {
            None
        }
    }

    fn attachments(&self) -> Option<&HashMap<isize, EntityComponent>> {
        self.attachments.as_ref()
    }

    fn attachment(&self, key: isize) -> Option<&EntityComponent> {
        if let Some(attachments) = &self.attachments {
            attachments.get(&key)
        } else {
            None
        }
    }

    fn contents(&self) -> Option<&Vec<Entity>> {
        self.contents.as_ref()
    }

    fn componentz(&self) -> Option<&Box<dyn EntityCompositionTrait>> {
        self.componentz.as_ref()
    }

    fn compozent(&self, key: isize) -> Result<&EntityComponent, ()> {
        if let Some(componentz) = &self.componentz {
            componentz.get(key)
        } else {
            Err(())
        }
    }
}

pub struct EntityDescription {
    pub(crate) name: String
}

impl DescriptionTrait for EntityDescription {
    fn name(&self) -> &str {
        self.name.as_str()
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

pub trait EntityComponentTrait {
    fn entity(&self) -> Option<&Entity>;
    fn components(&self) -> Option<&HashMap<isize, EntityComponent>>;
    fn component(&self, key: isize) -> Option<&EntityComponent>;
    fn attachments(&self) -> Option<&HashMap<isize, EntityComponent>>;
    fn attachment(&self, key: isize) -> Option<&EntityComponent>;
    fn contents(&self) -> Option<&Vec<Entity>>;
 }

pub struct EntityComponent {
    pub(crate) key: isize,
    pub(crate) entity: Option<Entity>
}

impl EntityComponentTrait for EntityComponent {
    fn entity(&self) -> Option<&Entity> {
        self.entity.as_ref()
    }

    fn components(&self) -> Option<&HashMap<isize, EntityComponent>> {
        if let Some(entity) = self.entity.as_ref() {
            entity.components()
        } else {
            None
        }
    }

    fn component(&self, key: isize) -> Option<&EntityComponent> {
        if let Some(components) = self.components() {
            components.get(&key)
        } else {
            None
        }
    }

    fn attachments(&self) -> Option<&HashMap<isize, EntityComponent>> {
        if let Some(entity) = self.entity.as_ref() {
            entity.attachments()
        } else {
            None
        }
    }

    fn attachment(&self, key: isize) -> Option<&EntityComponent> {
        if let Some(attachments) = self.attachments() {
            attachments.get(&key)
        } else {
            None
        }
    }

    fn contents(&self) -> Option<&Vec<Entity>> {
        if let Some(entity) = self.entity() {
            entity.contents()
        } else {
            None
        }
    }
}

pub trait EntityCompositionTrait {
    fn get(&self, key: isize) -> Result<&EntityComponent, ()>;
}