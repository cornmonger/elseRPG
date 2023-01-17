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
    inventory: Option<Box<Container<'a>>>
}

struct Item<'a> {
    entity: Entity<'a>
}

struct Container<'a> {
    item: Item<'a>,
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
                inventory: None
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
                inventory: Some(Box::new(
                    Container {
                        item: Item {
                            entity: Entity {
                                id: zone.next_id(),
                                name: "Backpack",
                                max_health: 1,
                                health: 1,
                                max_armor: 0,
                                armor: 0,
                                max_ability: 0,
                                ability: 0,
                                inventory: None
                            }
                        }
                    }
                ))
            }
        }
    };

    println!("Welcome, {}.", player.character.entity.name);
    println!("You arrive in {}.", lobby.name);
    println!("You see a {}.", troll.character.entity.name);
    println!("You are carrying a {}.", player.character.entity.inventory.unwrap().item.entity.name);
}
