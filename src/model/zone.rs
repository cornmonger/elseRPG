use super::DescriptionTrait;

pub trait ZoneTrait {
    fn id(&self) -> u64;
    fn description(&self) -> Option<&ZoneDescription>;
}

pub struct Zone {
    id: u64,
    description: Option<ZoneDescription>,
    serial_id: u64,
}

impl ZoneTrait for Zone {
    fn id(&self) -> u64 {
        self.id
    }

    fn description(&self) -> Option<&ZoneDescription> {
        self.description.as_ref()
    }
}

pub struct ZoneDescription {
    name: String
}

impl DescriptionTrait for ZoneDescription {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn rename(&mut self, name: String) {
        self.name = name;
    }
}

impl Zone {
    pub fn new(id: u64) -> Zone {
        Zone {
            id,
            description: Some(ZoneDescription { name: "World".to_owned() }),
            serial_id: 0
        }
    }

    pub fn generate_id(&mut self) -> u64 {
        self.serial_id += 1;
        self.serial_id
    }
}

