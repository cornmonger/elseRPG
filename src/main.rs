use elserpg::model::{zone::{Zone, ZoneTrait}, area::{Area, AreaTrait}, template::{HumanoidEntityTemplate, EntityTrait}, DescriptionTrait};


fn main() {
    let mut zone = Zone::new();
    let lobby = Area::new(&mut zone);
    let player = HumanoidEntityTemplate::new_player(&mut zone);
    let troll = HumanoidEntityTemplate::new_npc(&mut zone);

    println!("Welcome to {}, {}.", zone.description().name(), player.character.entity.description().unwrap().name());
    println!("You arrive in {}.", lobby.description().name());
    println!("You see a {}.", troll.character.entity.description().unwrap().name());
    /* println!("You are wearing a {} on your {}.",
        player.attached(HumanoidComponents::Back).unwrap().name(),
        HumanoidComponents::Back.name() ); */
}
