use elserpg::{Describable, Zone, Area, NPC, Player, HumanoidComponents};

fn main() {
    let mut zone = Zone::new();
    let lobby = Area::new(&mut zone);
    let troll = NPC::new(&mut zone);
    let player = Player::new(&mut zone);

    println!("Welcome to {}, {}.", zone.name(), player.name());
    println!("You arrive in {}.", lobby.name());
    println!("You see a {}.", troll.name());
    println!("You are wearing a {} on your {}.",
        player.attached(HumanoidComponents::Back.name()).name(),
        HumanoidComponents::Back.name() );
}
