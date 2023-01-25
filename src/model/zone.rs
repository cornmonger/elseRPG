use super::DescriptionTrait;

pub trait ZoneTrait<'z> {
    fn id(&self) -> u64;
    fn description(&self) -> &ZoneDescription<'z>;
    fn generate_id(&mut self) -> u64;
}

pub struct Zone<'z> {
    id: u64,
    description: ZoneDescription<'z>,
    serial_id: u64,
}

impl<'z> ZoneTrait<'z> for Zone<'z> {
    fn id(&self) -> u64 {
        self.id
    }

    fn description(&self) -> &ZoneDescription<'z> {
        &self.description
    }

    fn generate_id(&mut self) -> u64 {
        self.serial_id += 1;
        self.serial_id
    }
}

struct ZoneDescription<'z> {
    name: &'z str
}

impl<'z> DescriptionTrait<'z> for ZoneDescription<'z> {
    fn name(&self) -> &'z str {
        self.name
    }
}

impl<'a> Zone<'a> {
    pub fn new() -> Zone<'a> {
        Zone {
            id: 0,
            description: ZoneDescription { name: "World" },
            serial_id: 0
        }
    }
}

