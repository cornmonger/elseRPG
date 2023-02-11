#![allow(dead_code)]
use std::{thread, any::Any};
use elserpg::model::{
    zone::{Zone, ZoneTrait},
    area::{Area, AreaTrait},
    humanoid::{Humanoid, HumanoidPart},
    humanoid::{humanoid, HumanoidComposition},
    DescriptionTrait,
    entity::{Entity, EntityTrait, RelationTrait, EntityRef, PermeabilityTrait, EntityBuilder, RelationMapTrait, Filter}, character::Character};

fn main() {
    let mut zone = Zone::new(1);
    let lobby = Area::new(zone.generate_id());
    let player = Humanoid::new_player(&mut zone);

    println!("Welcome to {}, {}.",
        zone.description().unwrap().name(),
        player.character().entity().borrow().description().unwrap().name());

    println!("You arrive in {}.", lobby.description().unwrap().name());

    test_rename(&mut player.character().entity().borrow_mut(), "Link".to_string(), true);
    test_composition(&player.character().entity().borrow());
    test_backpack(&player.character().entity().borrow());
    test_backpack_parent(&player.character().entity().borrow());
    test_hurt(&mut player.character().entity().borrow_mut());
    test_add_to_backpack(&mut player.character().entity().borrow_mut(), &mut zone);
    test_backpack(&player.character().entity().borrow());

    let nemesis = Humanoid::new_player(&mut zone);
    test_rename(&mut nemesis.character().entity().borrow_mut(), "Ganondorf".to_string(), false);
    println!("{} has arrived in {}!", nemesis.character().entity().borrow().description().unwrap().name(), lobby.description().unwrap().name());

    test_backpack(&nemesis.character().entity().borrow());
    let flute = test_remove_from_backpack(&mut player.character().entity().borrow_mut());
    println!("{} stole a {} from {}!",
        nemesis.character().entity().borrow().description().unwrap().name(),
        flute.borrow().description().unwrap().name(),
        player.character().entity().borrow().description().unwrap().name());

    test_move_to_backpack(&mut nemesis.character().entity().borrow_mut(), flute);

    test_backpack(&player.character().entity().borrow());
    test_backpack(&nemesis.character().entity().borrow());
}

fn test_backpack(entity: &Entity) {
    let components = entity.components().unwrap().borrow();
    let composition = HumanoidComposition::from(&components);

    println!("{} has the following inside of the {} on their {}:",
        entity.description().unwrap().name(),
        composition.back().borrow().entity().unwrap().borrow()
            .attachment_entity(HumanoidPart::Back as isize).unwrap().borrow()
            .description().unwrap().name(),
        entity
            .components().unwrap().borrow().relation(HumanoidPart::Back as isize).unwrap().borrow().entity().unwrap().borrow()
            .description().unwrap().name() );

    let backpack = entity
        .components().unwrap().borrow().relation(HumanoidPart::Back as isize).unwrap().borrow().entity().unwrap().borrow()
        .attachments().unwrap().borrow().relation(HumanoidPart::Back as isize).unwrap().borrow().entity().unwrap().clone();
    
    for ent in backpack.borrow().contents().unwrap() {
        println!("  - {}", ent.borrow().description().unwrap().name());
    }
}

fn test_composition(entity: &Entity) {
    let mut names = Vec::<String>::new();

    for c in entity.components().unwrap().borrow().iter() {
        let name = String::from(c.borrow().entity().unwrap().borrow().description().unwrap().name());
        names.push(name);
    }

    println!("{} is composed of: {}.", entity.description().unwrap().name(), names.join(", "));
}

fn test_backpack_parent(entity: &Entity) {
    let back_strapto_rel = entity
        .component_entity(humanoid::Component::Torso as isize).unwrap().borrow()  // entity.components.back
        .attachment(humanoid::torso::Attachment::StrapToBack as isize).unwrap().clone();  // entity.components.back.entity.attachments.strapped_to (backpack)
    
    println!("The {}'s owner is {}.",
        back_strapto_rel.borrow().entity().unwrap().borrow() // backpack entity
            .description().unwrap().name(),
        back_strapto_rel.borrow().parent().borrow() // attachment composition for back
            .entity().borrow() // back entity
            .parent().unwrap().borrow() // back relation from back entity
            .parent().borrow() // human composition
            .entity().borrow() // the og entity
            .description().unwrap().name()
    );
}

fn test_hurt(entity: &mut Entity) {
    let health = entity.permeability().unwrap().health();
    entity.permeability_mut().unwrap().set_health(health - 10);

    println!("{}'s health before falling: {}. After falling: {}",
        entity.description().unwrap().name(),
        health,
        entity.permeability().unwrap().health() );
}

fn test_add_to_backpack(entity: &mut Entity, zone: &mut Zone) {
    let backpack = entity
        .component_entity(humanoid::Component::Torso as isize).unwrap().borrow()  // entity.components.back
        .attachment_entity(humanoid::torso::Attachment::StrapToBack as isize).unwrap().clone();  // entity.components.back.entity.attachments.strapped_to (backpack)

    let flute = Humanoid::new_flute(zone);
    backpack.borrow_mut().contents_mut().unwrap().push(flute.clone());
    println!("A {} was placed in your {}.",
        flute.borrow().description().unwrap().name(),
        backpack.borrow().description().unwrap().name()
    );
}

fn test_rename(entity: &mut Entity, name: String, announce: bool) {
    let old_name = String::from(entity.description().unwrap().name());
    entity.description_mut().unwrap().rename(name);

    if announce {
        println!("{} was renamed to {}.", old_name, entity.description().unwrap().name());
    }
}

fn test_remove_from_backpack(entity: &mut Entity) -> EntityRef {
    let backpack = entity
        .component_entity(HumanoidPart::Back as isize).unwrap().borrow()  // entity.components.back
        .attachment_entity(HumanoidPart::Back as isize).unwrap().clone();  // entity.components.back.entity.attachments.strapped_to (backpack)

    let flute = backpack.borrow_mut().contents_mut().unwrap().pop().unwrap();
    println!("A {} was removed from {}'s {}.",
        flute.borrow().description().unwrap().name(),
        entity.description().unwrap().name(),
        backpack.borrow().description().unwrap().name()
    );

    flute
}

fn test_move_to_backpack(entity: &mut Entity, item: EntityRef) {
    let backpack = entity
        .component_entity(humanoid::Component::Torso as isize).unwrap().borrow()  // entity.components.back
        .attachment_entity(humanoid::torso::Attachment::StrapToBack as isize).unwrap().clone();  // entity.components.back.entity.attachments.strapped_to (backpack)

    backpack.borrow_mut().contents_mut().unwrap().push(item);
}

fn test_relation_query(entity: &Entity) {
    //let head = entity.query_relation(humanoid::torso::Component::Attachment::StrapToBack, &[Filter::InnerContainer]);
    //let backpack = entity.query_relation(humanoid::torso::Attachment::StrapToBack, &[Filter::InnerContainer]);
}

fn test_butterflies(entity: &mut Entity, zone: &mut Zone) {
    let mut num = String::new();
    println!("\nHow many butterflies?");
    std::io::stdin().read_line(&mut num).expect("Bad butterfly!");
    num.pop();
    let num = num.parse::<isize>().unwrap();

    let time = std::time::SystemTime::now();

    println!("\nButterflies begin floating down ...");
    let mut butterflies: Vec<Character> = Vec::with_capacity(num as usize);
    for i in 0..num {
        let entity = EntityBuilder::new().id_zone(zone).description_name(&format!("Butterfly {}", i)).create();
        butterflies.push(Character::new(entity));
    }
    
    println!("{} butterflies have floated into {}.", butterflies.len(), zone.description().unwrap().name());

    println!("The butterflies are flying into your backpack ...");
    for _i in 0..num {
        let butterfly = butterflies.pop().as_mut().unwrap().entity().to_owned();
        test_move_to_backpack(entity, butterfly);
    }
    
    println!("All of the butterflies have floated into your backpack.");  
    println!("Butterflies spent {} seconds fluttering about.", std::time::SystemTime::now().duration_since(time).unwrap().as_secs());
}