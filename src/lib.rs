#![allow(dead_code)]

pub struct Area<'a> {
    id: u64,
    name: &'a str
}

pub trait ComponentModel {

}
pub enum HumanoidComponents {
    Head,
    Back 
}

pub struct HumanoidModel<'e> {
    head: Option<Entity<'e, Componentless>>,
    back: Option<Entity<'e, Componentless>>
}

pub struct Componentless {

}

impl ComponentModel for Componentless {
}

impl<'e> ComponentModel for HumanoidModel<'e> {
}

impl<'e> HumanoidModel<'e> {
    fn get(&self, component: HumanoidComponents) -> Option<Box<&dyn EntityTrait<'e>>> {
        match component {
            HumanoidComponents::Head => Some(Box::new(self.head.as_ref().unwrap())),
            HumanoidComponents::Back => Some(Box::new(self.back.as_ref().unwrap()))
        }
    }
}

pub trait EntityTrait<'e> {
    fn name(&self) -> &'e str;
}


pub struct Entity<'e, M: ComponentModel> {
    id: u64,
    name: &'e str,
    max_health: u16,
    health: u16,
    max_armor: u16,
    armor: u16,
    max_ability: u16,
    ability: u16,
    components: Option<M>,
    contains: Option<Vec<Box<dyn EntityTrait<'e>>>>
}

impl<'e, M: ComponentModel> EntityTrait<'e> for Entity<'e, M> {
    fn name(&self) -> &'e str {
        self.name
    }
}

pub struct Character<'e, M: ComponentModel> {
    entity: Entity<'e, M>,
}

pub struct NPC<'e, M: ComponentModel> {
    character: Character<'e, M>
}

pub struct Player<'e, M: ComponentModel> {
    character: Character<'e, M>
}

pub struct Zone<'z> {
    id: u64,
    name: &'z str,
    serial_id: u64,
}

impl<'e> HumanoidModel<'e> {
    pub fn new(zone: &mut Zone) -> HumanoidModel<'e> {
        HumanoidModel::<'e> {
            head: None,
            back: Some(
                    Entity::<'e, Componentless> {
                        id: zone.next_id(),
                        name: "Backpack",
                        max_health: 100,
                        health: 100,
                        max_armor: 0,
                        armor: 0,
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
                        max_armor: 100,
                        armor: 0,
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
                        max_armor: 0,
                        armor: 0,
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

pub struct Component {
    name: &'static str,
}

impl<'e, M: ComponentModel> Player<'e, M> {
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

impl<'e, M: ComponentModel> NPC<'e, M> {
    pub fn name(&self) -> &'e str {
        self.character.entity.name
    }
}