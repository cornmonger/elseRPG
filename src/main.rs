use elserpg::{Zone, Area, HumanoidComponents, HumanoidModel, Attachable};

fn main() {
    let mut zone = Zone::new();
    let lobby = Area::new(&mut zone);
    let player = HumanoidModel::new_player(&mut zone);
    let troll = HumanoidModel::new_npc(&mut zone);

    println!("Welcome to {}, {}.", zone.name(), player.name());
    println!("You arrive in {}.", lobby.name());
    println!("You see a {}.", troll.name());
    println!("You are wearing a {} on your {}.",
        player.attached(HumanoidComponents::Back).unwrap().name(),
        HumanoidComponents::Back.name() );
}
