#![allow(dead_code)]

use std::{collections::HashMap};

pub struct Area<'a> {
    id: u64,
    name: &'a str
}

pub struct Entity<'a> {
    id: u64,
    name: &'a str,
    max_health: u16,
    health: u16,
    max_armor: u16,
    armor: u16,
    max_ability: u16,
    ability: u16,
    components: Option<HashMap<&'a str, Entity<'a>>>,
    contains: Option<Vec<Entity<'a>>>
}

pub struct Character<'a> {
    entity: Entity<'a>,
}

pub struct NPC<'a> {
    character: Character<'a>
}

pub struct Player<'a> {
    character: Character<'a>
}

pub struct Zone<'a> {
    id: u64,
    name: &'a str,
    serial_id: u64,
}

pub enum HumanoidComponents {
    Head,
    Back 
}

pub trait Describable<'a> {
    fn name (&self) -> &'a str;
}

impl<'a> Describable<'a> for Zone<'a> {
    fn name(&self) -> &'a str {
        self.name
    }
}

impl<'a> Describable<'a> for Area<'a> {
    fn name(&self) -> &'a str {
        self.name
    }
}

impl<'a> Describable<'a> for Entity<'a> {
    fn name(&self) -> &'a str {
        self.name
    }
}

impl<'a> Describable<'a> for NPC<'a> {
    fn name(&self) -> &'a str {
        self.character.entity.name
    }
}

impl<'a> Describable<'a> for Character<'a> {
    fn name(&self) -> &'a str {
        self.entity.name
    }
}

impl<'a> Describable<'a> for Player<'a> {
    fn name(&self) -> &'a str {
        self.character.entity.name
    }
}

impl<'a> Player<'a> {
    pub fn new(zone: &mut Zone<'a>) -> Player<'a> {
        Player {
                character: Character {
                    entity: Entity {
                        id: 2,
                        name: "Player",
                        max_health: 100,
                        health: 100,
                        max_armor: 100,
                        armor: 0,
                        max_ability: 100,
                        ability: 100,
                        contains: None,
                        components: Some({
                            let mut map = HashMap::<&str, Entity>::new();
                            map.insert(HumanoidComponents::Back.name(), Entity {
                                id: zone.next_id(),
                                name: "Klondike Backpack",
                                max_health: 1,
                                health: 1,
                                max_armor: 0,
                                armor: 0,
                                max_ability: 0,
                                ability: 0,
                                components: None,
                                contains: None
                            });

                            map
                        })
                    }
                }
            }
    }

    pub fn attached(&self, component_name: &'static str) -> &Entity<'a> {
        self.character.entity.components.as_ref().unwrap().get(component_name).unwrap()
    }
}

impl<'a> NPC<'a> {
    pub fn new(zone: &mut Zone<'a>) -> NPC<'a> {
        NPC {
            character: Character {
                entity: Entity {
                    id: zone.next_id(),
                    name: "Troll",
                    max_health: 50,
                    health: 50,
                    max_armor: 0,
                    armor: 0,
                    max_ability: 0,
                    ability: 0,
                    components: None,
                    contains: None
                }
            }
        } 
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

pub struct Component<'a> {
    name: &'static str,
    entity: Option<Entity<'a>>
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
}

impl<'a> Area<'a> {
    pub fn new(zone: &mut Zone<'a>) -> Area<'a> {
        Area {
            id: zone.next_id(),
            name: "Lobby"
        }
    }
}