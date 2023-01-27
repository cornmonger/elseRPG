use elserpg::model::{zone::{Zone, ZoneTrait}, area::{Area, AreaTrait}, template::{Humanoid, EntityTrait, HumanoidPart, HumanoidComponentSlot}, DescriptionTrait, composition::{CompositionTrait, ComponentTrait, CompositionIteratorTrait}};
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
    let slot = player.character.entity.components().unwrap().component(HumanoidPart::Back).unwrap();
    if let HumanoidComponentSlot::Back(component) = slot {
        println!("You are wearing a {} on your {}.",
            component.get().description().unwrap().name(),
            HumanoidPart::Back.name() ); 
    }

    // by name
    let alias = HumanoidPart::by("back").unwrap();
    let slot = player.character.entity.components().unwrap().component(alias).unwrap();
    if let HumanoidComponentSlot::Back(component) = slot {
        println!("You are wearing a {} on your {}.",
            component.get().description().unwrap().name(),
            alias.name() ); 
    }
    
    // by entity template enum iteration directly
    //let template = player.character.entity.template().as_ref().unwrap();
    //let alias = template.component_model();
    let aliases = HumanoidPart::iter();
    for i in aliases {
        println!("Components: {}", i.name());
    }

    let player_composition = player.character.entity.components().unwrap();
    for c in player_composition.components() {
        if let Some(component) = c {
            println!("Some");
            if let Some(entity) = component.entity() {
                println!("Iterator Component: {}", entity.description().unwrap().name());
            }
        }
    }

}
