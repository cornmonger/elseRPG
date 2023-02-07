use std::any::Any;
use std::{rc::Rc, cell::RefCell};

use super::entity::{EntityRef, RelationRef, Relation, RelationMapTrait, RelationHashMap, EntityBuilder, WeakEntityRef, WeakRelationMapRef, RelationTrait, RelationMapRef, Entity, EntityTrait};
use super::{zone::{Zone}, character::{Player}};

pub struct Humanoid {
}

#[derive(Clone, Copy)]
pub enum HumanoidPart {
    Head = 1,
    Back = 2,
}

pub mod humanoid {
    use crate::model::entity::{RelationTemplate, CompositionTemplateTrait};

    pub enum Component {
        Head = 1,  // humanoid//Component::Head
        Torso = 2,
        Arms = 3,
        Legs = 4,
    }

    pub struct Composition {
        pub head: RelationTemplate,
        pub torso: RelationTemplate,
    }

    impl CompositionTemplateTrait for Composition {
        type Relationship = Component;
        const NAMESPACE: &'static str = "humanoid//Component";
    }

    pub const COMPOSITION: Composition = Composition {
        head: RelationTemplate {
            key: Component::Head as isize,
            namepath: "humanoid//Component::Head",
            name: "head"
        },
        torso: RelationTemplate {
            key: Component::Torso as isize,
            namepath: "humanoid//Component::Torso",
            name: "torso"
        },
    };

    pub mod head {
        pub enum Attachment {
            WearOn = 1  // humanoid/head//Attachment::WearOn
        }
    }
    pub mod torso {  // humanoid/torso 
        pub enum Component {  // humanoid/torso//Component
            Upper = 1,  // humanoid/torso//Component::Back
            Waist = 2,
        }
        pub enum Attachment {
            StrapToBack = 2,
        }

        pub mod upper {
            pub enum Attachment {
                WearOn = 1,
            }
        }
    }

    pub mod legs {
        pub enum Component {
            Left = 1,
            Right = 2
        }
        pub enum Attachment {
            WearOn = 1
        }
        pub mod leg {
            pub enum Component {
                Shin = 1,
                Foot = 2,
            }
        }
    }
}

pub struct HumanoidComposition {
    entity: Option<WeakEntityRef>,
    head: RelationRef,
    back: RelationRef
}

impl RelationMapTrait for HumanoidComposition {
    fn iter(&self) -> std::vec::IntoIter<RelationRef> {
        vec![self.head.clone(), self.back.clone()].into_iter()
    }

    fn bind(&mut self, entity: WeakEntityRef, weak_self: WeakRelationMapRef) {
        self.entity = Some(entity);

        self.head.borrow_mut().bind(weak_self.clone());
        self.back.borrow_mut().bind(weak_self.clone());
    }

    fn entity(&self) -> EntityRef {
        match self.entity {
            Some(ref entity) => entity.upgrade().unwrap(),
            None => panic!("Called entity before bind()!")
        }
    }

    fn relation(&self, key: isize) -> Result<&RelationRef, ()> {
        match key {
            1 => Ok(&self.head),
            2 => Ok(&self.back),
            _ => Err(())
        }
    }

    fn relation_ref(&self, key: isize) -> Result<RelationRef, ()> {
        match key {
            1 => Ok(self.head.clone()),
            2 => Ok(self.back.clone()),
            _ => Err(())
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl HumanoidComposition {
    pub fn new(zone: &mut Zone) -> HumanoidComposition {
        HumanoidComposition {
            entity: None,
            head: Relation::new_from(&humanoid::COMPOSITION.head, Some(
                EntityBuilder::new().id_zone(zone).description_name("Head").create()
            )),
            back: Relation::new_from(&humanoid::COMPOSITION.torso, Some(
                EntityBuilder::new().id_zone(zone).description_name("Back")
                    .attachments(Rc::new(RefCell::new(Box::new(
                        RelationHashMap::new(
                        [
                            (   HumanoidPart::Back as isize,
                                Relation::new(
                                    HumanoidPart::Back as isize,
                                    Some(Humanoid::new_backpack(zone))
                                )
                            )
                        ].into_iter().collect()
                        ))))
                    ) 
                    .create()
                )
            )
        }
    }

    pub fn from(boxed: &Box<dyn RelationMapTrait>) -> &Self {
        match boxed.as_any().downcast_ref::<Self>() {
            Some(c) => c,
            None => panic!("Not a HumanoidComposition"),
        }
    }

    pub fn head(&self) -> &RelationRef {
        &self.head
    }

    pub fn back(&self) -> &RelationRef {
        &self.back
    }
}

impl Humanoid {
    pub fn new_backpack(zone: &mut Zone) -> EntityRef {
        EntityBuilder::new().id_zone(zone).description_name("Backpack")
            .contents(vec![Self::new_apple(zone)])
            .create()
    }

    pub fn new_apple(zone: &mut Zone) -> EntityRef {
        EntityBuilder::new().id_zone(zone).description_name("Apple").create()
    }

    pub fn new_flute(zone: &mut Zone) -> EntityRef {
        EntityBuilder::new().id_zone(zone).description_name("Flute").create()
    }

    pub fn new_player(zone: &mut Zone) -> Player {
        Player::new(EntityBuilder::new().id_zone(zone).description_name("Player")
            .permeability_max(100, 100, 100)
            .components(
                Rc::new(RefCell::new(Box::new(HumanoidComposition::new(zone))))
            )
            .create()
        )
    }
}

