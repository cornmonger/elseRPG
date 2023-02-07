use std::{collections::{HashMap}, rc::{Rc, Weak}, cell::{RefCell}};
use std::any::Any;

use super::{DescriptionTrait, zone::Zone};

pub type RelationRef = Rc<RefCell<Relation>>;
pub type WeakRelationRef = Weak<RefCell<Relation>>;
pub type RelationMapRef = Rc<RefCell<Box<dyn RelationMapTrait>>>;
pub type WeakRelationMapRef = Weak<RefCell<Box<dyn RelationMapTrait>>>;
pub type EntityRef = Rc<RefCell<Entity>>;
pub type WeakEntityRef = Weak<RefCell<Entity>>;

pub trait EntityTrait {
    fn id(&self) -> u64;
    fn permeability(&self) -> Option<&Permeability>;
    fn permeability_mut(&mut self) -> Option<&mut Permeability>;
    fn description(&self) -> Option<&EntityDescription>;
    fn description_mut(&mut self) -> Option<&mut EntityDescription>;
    fn components(&self) -> Option<&RelationMapRef>;
    fn component(&self, key: isize) -> Result<RelationRef, ()>;
    fn component_entity(&self, key: isize) -> Result<EntityRef, ()>;
    fn attachments(&self) -> Option<&RelationMapRef>;
    fn attachment(&self, key: isize) -> Result<RelationRef, ()>;
    fn attachment_entity(&self, key: isize) -> Result<EntityRef, ()>;
    fn contents(&self) -> Option<&Vec<EntityRef>>;
    fn contents_mut(&mut self) -> Option<&mut Vec<EntityRef>>;
    fn parent(&self) -> Option<RelationRef>;
}

pub struct Entity {
    pub(crate) id: u64,
    pub(crate) permeability: Option<Permeability>,
    pub(crate) description: Option<EntityDescription>,
    pub(crate) components: Option<RelationMapRef>,
    pub(crate) attachments: Option<RelationMapRef>,
    pub(crate) contents: Option<Vec<EntityRef>>,
    pub(crate) parent: Option<WeakRelationRef>
}

impl EntityTrait for Entity {
    fn id(&self) -> u64 {
        self.id
    }

    fn permeability(&self) -> Option<&Permeability> {
        self.permeability.as_ref()
    }

    fn permeability_mut(&mut self) -> Option<&mut Permeability> {
        self.permeability.as_mut()
    }

    fn description(&self) -> Option<&EntityDescription> {
        self.description.as_ref()
    }

    fn description_mut(&mut self) -> Option<&mut EntityDescription> {
        self.description.as_mut()
    }

    fn components(&self) -> Option<&RelationMapRef> {
        self.components.as_ref()
    }

    fn component(&self, key: isize) -> Result<RelationRef, ()> {
        match self.components {
            Some(ref components) => components.borrow().relation_ref(key),
            None => Err(())
        }
    }

    fn attachments(&self) -> Option<&RelationMapRef> {
        self.attachments.as_ref()
    }

    fn attachment(&self, key: isize) -> Result<RelationRef, ()> {
        match self.attachments {
            Some(ref attachments) => attachments.borrow().relation_ref(key),
            None => Err(())
        }
    }

    fn contents(&self) -> Option<&Vec<EntityRef>> {
        self.contents.as_ref()
    }

    fn contents_mut(&mut self) -> Option<&mut Vec<EntityRef>> {
        self.contents.as_mut()
    }

    fn component_entity(&self, key: isize) -> Result<EntityRef, ()> {
        match self.component(key) {
            Ok(ref component) => Ok(component.borrow().entity().unwrap().clone()),
            Err(_)=> Err(())
        }
    }

    fn attachment_entity(&self, key: isize) -> Result<EntityRef, ()> {
        match self.attachment(key) {
            Ok(ref attachment) => Ok(attachment.borrow().entity().unwrap().clone()),
            Err(_)=> Err(())
        }
    }

    fn parent(&self) -> Option<RelationRef> {
        match self.parent {
            Some(ref parent) => parent.upgrade(),
            None => None
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

    fn rename(&mut self, name: String) {
        self.name = name;
    }
}

pub struct EntityBuilder {
    id: u64,
    permeability: Option<Permeability>,
    description: Option<EntityDescription>,
    components: Option<RelationMapRef>,
    attachments: Option<RelationMapRef>,
    contents: Option<Vec<EntityRef>>,
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

    pub fn create(self) -> EntityRef {
        let entity = Rc::new(RefCell::new(Entity {
            id: self.id,
            permeability: self.permeability,
            description: self.description,
            components: self.components,
            attachments: self.attachments,
            contents: self.contents,
            parent: None
        }));

        let weak_entity = Rc::downgrade(&entity);

        if let Some(components) = &mut entity.borrow_mut().components {
            let weak_composition = Rc::downgrade(&components);
            components.borrow_mut().bind(weak_entity.clone(), weak_composition.clone());
        }

        if let Some(attachments) = &mut entity.borrow_mut().attachments{
            let weak_attachments = Rc::downgrade(&attachments);
            attachments.borrow_mut().bind(weak_entity.clone(), weak_attachments.clone());
        }

        entity 
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

    pub fn components(mut self, components: RelationMapRef) -> Self {
        self.components = Some(components);
        self
    }

    pub fn attachments(mut self, attachments: RelationMapRef) -> Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn contents(mut self, contents: Vec<EntityRef>) -> Self {
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
    fn set_health(&mut self, health: u16);
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

    fn set_health(&mut self, health: u16) {
        self.health = health;
    }
}

pub trait RelationTrait {
    fn parent(&self) -> RelationMapRef;
    fn entity(&self) -> Option<&EntityRef>;
    fn bind(&mut self, parent: WeakRelationMapRef);
 }

pub struct Relation {
    pub(crate) key: isize,
    pub(crate) parent: Option<WeakRelationMapRef>,
    pub(crate) entity: Option<EntityRef>,
    pub(crate) template: Option<&'static RelationTemplate>,
}

impl RelationTrait for Relation {
    fn parent(&self) -> RelationMapRef {
        match self.parent {
            Some(ref parent) => parent.upgrade().unwrap(),
            None => panic!("Relation parent doesn't exist!")
        }
    }

    fn entity(&self) -> Option<&EntityRef> {
        self.entity.as_ref()
    }

    fn bind(&mut self, parent: WeakRelationMapRef) {
        self.parent = Some(parent);
    }
}

impl Relation {
    pub fn new(key: isize, entity: Option<EntityRef>) -> RelationRef {
        let relation = Rc::new(RefCell::new(Relation { key, parent: None, template: None, entity}));
        
        let weak_relation = Rc::downgrade(&relation);

        if let Some(entity) = &relation.borrow_mut().entity {
            entity.borrow_mut().parent = Some(weak_relation);
        }

        relation
    }

    pub fn new_from(template: &'static RelationTemplate, entity: Option<EntityRef>) -> RelationRef {
        let relation = Rc::new(RefCell::new(Relation { key: template.key(), parent: None, template: Some(template), entity}));
        
        let weak_relation = Rc::downgrade(&relation);

        if let Some(entity) = &relation.borrow_mut().entity {
            entity.borrow_mut().parent = Some(weak_relation);
        }

        relation
    }

    pub fn entity(&self) -> Option<&EntityRef> {
        self.entity.as_ref()
    }
}

pub trait RelationMapTrait {
    fn bind(&mut self, entity: WeakEntityRef, weak_self: WeakRelationMapRef);
    fn entity(&self) -> EntityRef;
    fn relation(&self, key: isize) -> Result<&RelationRef, ()>;
    fn relation_ref(&self, key: isize) -> Result<RelationRef, ()>;
    fn iter(&self) -> std::vec::IntoIter<RelationRef>;
    fn as_any(&self) -> &dyn Any;
}

pub struct RelationHashMap {
    entity: Option<WeakEntityRef>,
    map: HashMap<isize, RelationRef>
}

impl RelationMapTrait for RelationHashMap {
    fn iter(&self) -> std::vec::IntoIter<RelationRef> {
        self.map.values().cloned().collect::<Vec<RelationRef>>().into_iter()
    }

    fn bind(&mut self, entity: WeakEntityRef, weak_self: WeakRelationMapRef) {
        self.entity = Some(entity);

        for r in self.map.iter_mut() {
            r.1.borrow_mut().bind(weak_self.clone());
        }
    }

    fn entity(&self) -> EntityRef {
        match self.entity {
            Some(ref entity) => entity.upgrade().unwrap(),
            None => panic!("Called entity before bind()!")
        }
    }

    fn relation(&self, key: isize) -> Result<&RelationRef, ()> {
        if let Some(value) = self.map.get(&key) {
            Ok(value)
        } else {
            Err(())
        }
    }

    fn relation_ref(&self, key: isize) -> Result<RelationRef, ()> {
        if let Some(value) = self.map.get(&key) {
            Ok(value.clone())
        } else {
            Err(())
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RelationHashMap {
    pub fn new(map: HashMap<isize, RelationRef>) -> Self {
        Self { entity: None, map }
    }
}

pub trait CompositionTemplateTrait {
    type Relationship;
    const NAMESPACE: &'static str;
}

pub struct RelationTemplate {
    pub key: isize,
    pub namepath: &'static str,
    pub name: &'static str,
}

impl RelationTemplate {
    pub fn key(&self) -> isize {
        self.key
    }

    pub fn namepath(&self) -> &'static str {
        self.namepath
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

