use std::collections::HashMap;

struct Area<'a> {
    id: u64,
    name: &'a str
}

struct Entity<'a> {
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

struct Character<'a> {
    entity: Entity<'a>,
}

struct NPC<'a> {
    character: Character<'a>
}

struct Player<'a> {
    character: Character<'a>
}

struct Zone<'a> {
    id: u64,
    name: &'a str,
    serial_id: u64,
}

struct Component<'a> {
    id: u64,
    name: &'a str
}

struct HumanoidComponents<'a> {
    head: Component<'a>,
    torso: Component<'a>,
    shoulders: Component<'a>,
    hands: Component<'a>,
    waist: Component<'a>,
    legs: Component<'a>,
    feet: Component<'a>,
}

impl Zone<'_> {
    pub fn next_id(&mut self) -> u64 {
        self.serial_id += 1;
        self.serial_id
    }
}

fn main() {
    let mut zone = Zone {
        id: 1,
        name: "World",
        serial_id: 1
    };

    let lobby = Area {
        id: zone.next_id(),
        name: "Lobby"
    };

    let troll = NPC {
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
    };

    let player = Player {
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
                    map.insert("back", Entity {
                        id: zone.next_id(),
                        name: "Backpack",
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
    };

    println!("Welcome, {}.", player.character.entity.name);
    println!("You arrive in {}.", lobby.name);
    println!("You see a {}.", troll.character.entity.name);
    println!("You are carrying a {}.", player.character.entity.components.unwrap().get("back").unwrap().name);
}
