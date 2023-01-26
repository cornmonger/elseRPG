use elserpg::model::{zone::{Zone, ZoneTrait}, area::{Area, AreaTrait}, template::{HumanoidEntityTemplate, EntityTrait, HumanoidComponentAlias, HumanoidComponentSlot, EntityTemplateTrait}, DescriptionTrait, component::{ComponentModelTrait, ComponentTrait}};
use strum::IntoEnumIterator;


fn main() {
    let mut zone = Zone::new();
    let lobby = Area::new(&mut zone);
    let player = HumanoidEntityTemplate::new_player(&mut zone);
    let troll = HumanoidEntityTemplate::new_npc(&mut zone);

    println!("Welcome to {}, {}.", zone.description().name(), player.character.entity.description().unwrap().name());
    println!("You arrive in {}.", lobby.description().name());
    println!("You see a {}.", troll.character.entity.description().unwrap().name());
    
    // pragmatically
    let slot = player.character.entity.components().unwrap().component(HumanoidComponentAlias::Back).unwrap();
    if let HumanoidComponentSlot::Back(component) = slot {
        println!("You are wearing a {} on your {}.",
            component.get().description().unwrap().name(),
            HumanoidComponentAlias::Back.name() ); 
    }

    // by name
    let alias = HumanoidComponentAlias::by("back").unwrap();
    let slot = player.character.entity.components().unwrap().component(alias).unwrap();
    if let HumanoidComponentSlot::Back(component) = slot {
        println!("You are wearing a {} on your {}.",
            component.get().description().unwrap().name(),
            alias.name() ); 
    }
    
    // by entity's template
    let template = player.character.entity.template().as_ref().unwrap();
    let alias = template.component_model();
    let aliases = HumanoidComponentAlias::iter();
    for i in aliases {
        println!("Components: {}", i.name());
    }



}
