use elserpg::{Zone, Area, NPC, Player, HumanoidComponents};

fn main() {
    let mut zone = Zone::new();
    let lobby = Area::new(&mut zone);
    let troll = NPC::new(&mut zone);
    let player = Player::new(&mut zone);

    println!("Welcome to {}, {}.", zone, player);
    println!("You arrive in {}.", lobby);
    println!("You see a {}.", troll);
    println!("You are wearing a {} on your {}.",
        player.attached(HumanoidComponents::Back), HumanoidComponents::Back);
}
