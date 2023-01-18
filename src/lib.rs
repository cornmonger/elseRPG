#![allow(dead_code)]

pub struct Area<'a> {
    id: u64,
    name: &'a str
}

pub trait ComponentModel<'e> {
    type Components;

    fn get(&self, component: Self::Components) -> Option<Box<&dyn EntityTrait<'e>>>;
}

pub enum HumanoidComponents {
    Head,
    Back 
}

pub enum NoComponents {}

pub struct HumanoidModel<'e> {
    head: Option<Entity<'e, NoComponentModel>>,
    back: Option<Entity<'e, NoComponentModel>>
}

pub struct NoComponentModel {

}

impl<'e> ComponentModel<'e> for NoComponentModel {
    type Components = NoComponents;

    fn get(&self, _component: Self::Components) -> Option<Box<&dyn EntityTrait<'e>>> {
        None
    }
}

impl<'e> ComponentModel<'e> for HumanoidModel<'e> {
    type Components = HumanoidComponents;

    fn get(&self, component: Self::Components) -> Option<Box<&dyn EntityTrait<'e>>> {
        match component {
            HumanoidComponents::Head => Some(Box::new(self.head.as_ref().unwrap())),
            HumanoidComponents::Back => Some(Box::new(self.back.as_ref().unwrap()))
        }
    }
}

pub trait EntityTrait<'e> {
    fn name(&self) -> &'e str;
}


pub struct Entity<'e, M: ComponentModel<'e>> {
    id: u64,
    name: &'e str,
    max_health: u16,
    health: u16,
    max_resist: u16,
    resist: u16,
    max_ability: u16,
    ability: u16,
    components: Option<M>,
    contains: Option<Vec<Box<dyn EntityTrait<'e>>>>
}

impl<'e, M: ComponentModel<'e>> EntityTrait<'e> for Entity<'e, M> {
    fn name(&self) -> &'e str {
        self.name
    }
}

pub struct Character<'e, M: ComponentModel<'e>> {
    entity: Entity<'e, M>,
}

pub struct NPC<'e, M: ComponentModel<'e>> {
    character: Character<'e, M>
}

pub struct Player<'e, M: ComponentModel<'e>> {
    character: Character<'e, M>
}

pub struct Zone<'z> {
    id: u64,
    name: &'z str,
    serial_id: u64,
}

pub trait Component<'e> {
    fn components(&self) -> Option<Box<dyn Component<'e>>>;
    fn get(&self) -> ComponentType<'e>;
}

pub enum ComponentType<'e> {
    Entity (EntityComponent<'e, M: ComponentModel>),
    Logical (LogicalComponent<'e>)
}

pub struct EntityComponent<'e, M: ComponentModel<'e>> {
    entity: Entity<'e, M>,
    subcomponents: Option<Box<Component
}

pub struct LogicalComponent<'e> {

}

impl<'e, M: ComponentModel<'e>> Component for EntityComponent<'e, M> {
}

impl<'e, M: ComponentModel<'e>> EntityComponent<'e, M> {
    pub fn entity(&self) -> &Entity<'e, M> {
        &self.entity
    }
}

impl<'e> HumanoidModel<'e> {
    pub fn new(zone: &mut Zone) -> HumanoidModel<'e> {
        HumanoidModel::<'e> {
            head: None,
            back: Some(
                    Entity::<'e, NoComponentModel> {
                        id: zone.next_id(),
                        name: "Backpack",
                        max_health: 100,
                        health: 100,
                        max_resist: 0,
                        resist: 0,
                        max_ability: 0,
                        ability: 100,
                        contains: None,
                        components: None
                    }
            ) 
        }
    }

    pub fn new_player(zone: &mut Zone) -> Player<'e, HumanoidModel<'e>> {
        Player {
                character: Character {
                    entity: Entity {
                        id: zone.next_id(),
                        name: "Player",
                        max_health: 100,
                        health: 100,
                        max_resist: 100,
                        resist: 0,
                        max_ability: 100,
                        ability: 100,
                        contains: None,
                        components: Some(HumanoidModel::new(zone))
                    }
                }
            }
    }

    pub fn new_npc(zone: &mut Zone) -> NPC<'e, HumanoidModel<'e>> {
        NPC {
                character: Character {
                    entity: Entity {
                        id: zone.next_id(),
                        name: "Troll",
                        max_health: 40,
                        health: 40,
                        max_resist: 0,
                        resist: 0,
                        max_ability: 0,
                        ability: 0,
                        contains: None,
                        components: Some(HumanoidModel::new(zone))
                    }
                }
            }
    }

}

pub trait Attachable<'e, M, E> {
    fn attached(&self, component: E) -> Option<Box<&dyn EntityTrait<'e>>>;
}

impl<'e> Attachable<'e, HumanoidModel<'e>, HumanoidComponents> for Player<'e, HumanoidModel<'e>>  {
    fn attached(&self, component: HumanoidComponents) -> Option<Box<&dyn EntityTrait<'e>>> {
        self.character.entity.components.as_ref().unwrap().get(component)
    }
}

impl HumanoidComponents {
    pub fn name(&self) -> &'static str {
        match self {
            HumanoidComponents::Head => "head",
            HumanoidComponents::Back => "back"
        }
    }
}



impl<'e, M: ComponentModel<'e>> Player<'e, M> {
    pub fn name(&self) -> &'e str {
        self.character.entity.name
    }
}

impl<'a> Zone<'a> {
    pub fn new() -> Zone<'a> {
        Zone {
            id: 1,
            name: "World",
            serial_id: 1
        }
    }

    pub fn next_id(&mut self) -> u64 {
        self.serial_id += 1;
        self.serial_id
    }

    pub fn name(&self) -> &'a str {
        self.name
    }
}

impl<'a> Area<'a> {
    pub fn new(zone: &mut Zone<'a>) -> Area<'a> {
        Area {
            id: zone.next_id(),
            name: "Lobby"
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }
}

impl<'e, M: ComponentModel<'e>> NPC<'e, M> {
    pub fn name(&self) -> &'e str {
        self.character.entity.name
    }
}