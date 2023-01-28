use elserpg::model::{zone::{Zone, ZoneTrait}, area::{Area, AreaTrait}, template::{Humanoid, EntityTrait, HumanoidPart, HumanoidComponentSlot, Entity, EntityTemplateTrait}, DescriptionTrait, composition::{CompositionTrait, ComponentTrait, CompositionIteratorTrait}};
use strum::IntoEnumIterator;


fn main() {
    let mut zone = Zone::new();
    let lobby = Area::new(&mut zone);
    let player = Humanoid::new_player(&mut zone);
    let troll = Humanoid::new_npc(&mut zone);

    println!("Welcome to {}, {}.", zone.description().name(), player.character.entity.description().unwrap().name());
    println!("You arrive in {}.", lobby.description().name());
    println!("You see a {}.", troll.character.entity.description().unwrap().name());
    
    // pragmatically
    let slot = player.character.entity.composition().unwrap().component(HumanoidPart::Back).unwrap();
    if let HumanoidComponentSlot::Back(component) = slot {
        println!("You are wearing a {} on your {}.",
            component.get().description().unwrap().name(),
            HumanoidPart::Back.name() ); 
    }

    // by name
    let alias = HumanoidPart::by("back").unwrap();
    let slot = player.character.entity.composition().unwrap().component(alias).unwrap();
    if let HumanoidComponentSlot::Back(component) = slot {
        println!("You are wearing a {} on your {}.",
            component.get().description().unwrap().name(),
            alias.name() ); 
    }
    
    let character = player.character();
    let player_entity= character.entity();
    test_backpack(&player_entity);

    // by entity template enum iteration directly
    //let template = player.character.entity.template().as_ref().unwrap();
    //let alias = template.component_model();
    //let aliases = HumanoidPart::iter();
    //for i in aliases {
    //    println!("Component by alias: {}", i.name());
    //}

    /* for c in player_composition.components() {
        if let Some(component) = c {
            println!("Some");
            if let Some(entity) = component.entity() {
                println!("Iterator Component: {}", entity.description().unwrap().name());
            }
        }
    } */
}

fn test_backpack<'e:'i,'i, T: EntityTemplateTrait<'e,'i>>(humanoid: &'i Entity<'e,'i, T>) {
    //let template = humanoid.template().as_ref().unwrap();
    //let composition = humanoid.composition().unwrap();

    //let iter = composition.components();
}
