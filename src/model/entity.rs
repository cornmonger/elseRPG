use std::{collections::{HashMap}, rc::Rc, borrow::Borrow};

use super::{DescriptionTrait, zone::Zone};

pub trait EntityTrait {
    fn id(&self) -> u64;
    fn permeability(&self) -> Option<&Permeability>;
    fn description(&self) -> Option<&EntityDescription>;
    fn components(&self) -> Option<&Box<dyn EntityCompositionTrait>>;
    fn component(&self, key: isize) -> Result<&EntityComponent, ()>;
    fn attachments(&self) -> Option<&Box<dyn EntityCompositionTrait>>;
    fn attachment(&self, key: isize) -> Result<&EntityComponent, ()>;
    fn contents(&self) -> Option<&Vec<Entity>>;
}

pub struct Entity {
    pub(crate) id: u64,
    pub(crate) permeability: Option<Permeability>,
    pub(crate) description: Option<EntityDescription>,
    pub(crate) components: Option<Box<dyn EntityCompositionTrait>>,
    pub(crate) attachments: Option<Box<dyn EntityCompositionTrait>>,
    pub(crate) contents: Option<Vec<Entity>>,
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

    fn components(&self) -> Option<&Box<dyn EntityCompositionTrait>> {
        self.components.as_ref()
    }

    fn component(&self, key: isize) -> Result<&EntityComponent, ()> {
        if let Some(components) = &self.components {
            components.get(key)
        } else {
            Err(())
        }
    }

    fn attachments(&self) -> Option<&Box<dyn EntityCompositionTrait>> {
        self.attachments.as_ref()
    }

    fn attachment(&self, key: isize) -> Result<&EntityComponent, ()> {
        if let Some(attachments) = &self.attachments {
            attachments.get(key)
        } else {
            Err(())
        }
    }

    fn contents(&self) -> Option<&Vec<Entity>> {
        self.contents.as_ref()
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

pub struct EntityBuilder {
    id: u64,
    permeability: Option<Permeability>,
    description: Option<EntityDescription>,
    components: Option<Box<dyn EntityCompositionTrait>>,
    attachments: Option<Box<dyn EntityCompositionTrait>>,
    contents: Option<Vec<Entity>>,
}

impl EntityBuilder {
    pub fn new() -> Self {
        EntityBuilder {
            id: 0,
            permeability: None,
            description: None,
            components: None,
            attachments: None,
            contents: None 
        }
    }

    pub fn create(mut self) -> Entity {
        Entity {
            id: self.id,
            permeability: self.permeability,
            description: self.description,
            components: self.components,
            attachments: self.attachments,
            contents: self.contents
        }
    }

    pub fn id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub fn permeability(mut self, permeability: Permeability) -> Self {
        self.permeability = Some(permeability);
        self
    }

    pub fn description(mut self, description: EntityDescription) -> Self {
        self.description = Some(description);
        self
    }

    pub fn components(mut self, components: Box<dyn EntityCompositionTrait>) -> Self {
        self.components = Some(components);
        self
    }

    pub fn attachments(mut self, attachments: Box<dyn EntityCompositionTrait>) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn contents(mut self, contents: Vec<Entity>) -> Self {
        self.contents = Some(contents);
        self
    }

    pub fn id_zone(mut self, zone: &mut Zone) -> Self {
        self.id = zone.generate_id();
        self
    }

    pub fn description_name(mut self, name: &str) -> Self {
        self.description = Some(EntityDescription { name: name.to_owned() });
        self
    }

    pub fn permeability_max(mut self, max_health: u16, max_resist: u16, max_ability: u16) -> Self {
        self.permeability = Some(Permeability { max_health, max_resist, max_ability, health: max_health,
            resist: max_resist, ability: max_ability });
        self
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
    fn parent(&self) -> Option<&EntityComponent>;
    fn entity(&self) -> Option<&Entity>;
    fn components(&self) -> Option<&Box<dyn EntityCompositionTrait>>;
    fn component(&self, key: isize) -> Result<&EntityComponent, ()>;
    fn attachments(&self) -> Option<&Box<dyn EntityCompositionTrait>>;
    fn attachment(&self, key: isize) -> Result<&EntityComponent, ()>;
    fn contents(&self) -> Option<&Vec<Entity>>;
 }

pub struct EntityComponent {
    pub(crate) key: isize,
    pub(crate) parent: Option<Rc<EntityComponent>>,
    pub(crate) entity: Option<Entity>
}

impl EntityComponentTrait for EntityComponent {
    fn parent(&self) -> Option<&EntityComponent> {
        if let Some(parent) = self.parent.as_ref() {
            Some(parent.borrow())
        } else {
            None
        }
    }

    fn entity(&self) -> Option<&Entity> {
        self.entity.as_ref()
    }

    fn components(&self) -> Option<&Box<dyn EntityCompositionTrait>> {
        if let Some(entity) = self.entity.as_ref() {
            entity.components()
        } else {
            None
        }
    }

    fn component(&self, key: isize) -> Result<&EntityComponent, ()> {
        if let Some(components) = self.components() {
            components.get(key)
        } else {
            Err(())
        }
    }

    fn attachments(&self) -> Option<&Box<dyn EntityCompositionTrait>> {
        if let Some(entity) = self.entity.as_ref() {
            entity.attachments()
        } else {
            None
        }
    }

    fn attachment(&self, key: isize) -> Result<&EntityComponent, ()> {
        if let Some(attachments) = self.attachments() {
            attachments.get(key)
        } else {
            Err(())
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

impl EntityComponent {
    pub fn new(key: isize, parent: Option<Rc<EntityComponent>>, entity: Option<Entity>) -> Self {
        EntityComponent { key, parent, entity }
    }
}

pub trait EntityCompositionTrait {
    fn get(&self, key: isize) -> Result<&EntityComponent, ()>;
    fn iter(&self) -> std::vec::IntoIter<&EntityComponent>;
}

pub struct UnrestrainedEntityComposition {
    map: HashMap<isize, EntityComponent>
}

impl EntityCompositionTrait for UnrestrainedEntityComposition {
    fn get(&self, key: isize) -> Result<&EntityComponent, ()> {
        if let Some(value) = self.map.get(&key) {
            Ok(value)
        } else {
            Err(())
        }
    }

    fn iter(&self) -> std::vec::IntoIter<&EntityComponent> {
        let vec: Vec<&EntityComponent> = self.map.values().collect();
        vec.into_iter()
    }
}

impl UnrestrainedEntityComposition {
    pub fn new(map: HashMap<isize, EntityComponent>) -> Self {
        Self { map }
    }

    pub fn empty() -> Self {
        Self { map: HashMap::new() }
    }
}
