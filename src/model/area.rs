use super::{DescriptionTrait, zone::{Zone, ZoneTrait}};

pub trait AreaTrait<'a> {
    fn id(&self) -> u64;
    fn description(&self) -> &AreaDescription<'a>;
}

pub struct Area<'a> {
    id: u64,
    description: AreaDescription<'a>
}

impl<'a> AreaTrait<'a> for Area<'a> {
    fn id(&self) -> u64 {
        self.id
    }

    fn description(&self) -> &AreaDescription<'a> {
        &self.description
    }
}

pub struct AreaDescription<'a> {
    name: &'a str
}

impl<'a> DescriptionTrait<'a> for AreaDescription<'a> {
    fn name(&self) -> &'a str {
        self.name
    }
}

impl<'a> Area<'a> {
    pub fn new(zone: &mut Zone<'a>) -> Area<'a> {
        Area {
            id: zone.generate_id(),
            description: AreaDescription { name: "Lobby" }
        }
    }
}

