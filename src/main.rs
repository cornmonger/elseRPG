struct Area<'a> {
    id: u64,
    name: &'a str
}

struct Entity<'a> {
    id: u64,
    name: &'a str
}

struct Item<'a> {
    entity: Entity<'a>
}

struct Container<'a> {
    entity: Entity<'a>
}

struct Character<'a> {
    entity: Entity<'a>
}

struct NPC<'a> {
    character: Character<'a>
}

struct Player<'a> {
    character: Character<'a>
}

fn main() {
    let lobby = Area {
        id: 1,
        name: "Lobby"
    };

    let troll = NPC {
        character: Character {
            entity: Entity {
                id: 1,
                name: "Troll"
            }
        }
    };

    let player = Player {
        character: Character {
            entity: Entity {
                id: 2,
                name: "Player"
            }
        }
    };

    println!("Welcome, {}.", player.character.entity.name);
    println!("You arrive in {}.", lobby.name);
    println!("You see a {}.", troll.character.entity.name);
}
