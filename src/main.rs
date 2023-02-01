use elserpg::model::{
    zone::{Zone, ZoneTrait},
    area::{Area, AreaTrait},
    template::{Humanoid, EntityTrait, Entity, HumanoidPart},
    DescriptionTrait,
    entity::EntityComponentTrait};

fn main() {
    let mut zone = Zone::new(1);
    let lobby = Area::new(zone.generate_id());
    let player = Humanoid::new_player(&mut zone);

    println!("Welcome to {}, {}.",
        zone.description().unwrap().name(),
        player.character().entity().description().unwrap().name());

    println!("You arrive in {}.", lobby.description().unwrap().name());

    test_composition(player.character().entity());
    test_backpack(player.character().entity());
    //test_backpack_parent(player.character().entity());
}

fn test_backpack(entity: &Entity) {
    println!("You have a {} inside of the {} on your {}.",
        entity
            .component(HumanoidPart::Back as isize).unwrap()
            .attachment(HumanoidPart::Back as isize).unwrap().entity().unwrap()
            .contents().unwrap()
            .get(0).unwrap()
            .description().unwrap().name(),
        entity
            .component(HumanoidPart::Back as isize).unwrap()
            .attachment(HumanoidPart::Back as isize).unwrap().entity().unwrap()
            .description().unwrap().name(),
        entity
            .component(HumanoidPart::Back as isize).unwrap().entity().unwrap()
            .description().unwrap().name() );
}

fn test_composition(entity: &Entity) {
    let mut names = Vec::<&str>::new();

    entity.components().unwrap().iter().for_each(|component| {
        names.push(component.entity().unwrap().description().unwrap().name());
    });

    println!("You are composed of: {}.", names.join(", "));
}


fn test_backpack_parent(entity: &Entity) {
    let backpack_component= entity.component(HumanoidPart::Back as isize).unwrap()
        .attachment(HumanoidPart::Back as isize).unwrap();
    
    let original_entity = backpack_component.parent().unwrap()
        .parent().unwrap().entity().unwrap();

    println!("The {}'s owner is {}.",
        backpack_component.entity().unwrap().description().unwrap().name(),
        original_entity.description().unwrap().name()
    );
}