use elserpg::model::{zone::{Zone, ZoneTrait}, area::{Area, AreaTrait}, template::{Humanoid, EntityTrait, Entity, HumanoidPart}, DescriptionTrait, entity::EntityComponentTrait};



fn main() {
    let mut zone = Zone::new(1);
    let lobby = Area::new(zone.generate_id());
    let player = Humanoid::new_player(&mut zone);

    println!("Welcome to {}, {}.", zone.description().unwrap().name(), player.character().entity().description().unwrap().name());
    println!("You arrive in {}.", lobby.description().unwrap().name());

    test_backpack(player.character().entity());
}

fn test_backpack(entity: &Entity) {
    println!("You are wearing a {} on your {}.",
        entity.component(HumanoidPart::Back as isize).unwrap().entity().unwrap().description().unwrap().name(),
        HumanoidPart::Back.name() );
}
