use super::DescriptionTrait;

pub trait AreaTrait {
    fn id(&self) -> u64;
    fn description(&self) -> Option<&AreaDescription>;
}

pub struct Area {
    id: u64,
    description: Option<AreaDescription>
}

impl AreaTrait for Area {
    fn id(&self) -> u64 {
        self.id
    }

    fn description(&self) -> Option<&AreaDescription> {
        self.description.as_ref()
    }
}

pub struct AreaDescription {
    name: String
}

impl DescriptionTrait for AreaDescription {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Area {
    pub fn new(id: u64) -> Area {
        Area {
            id,
            description: Some(AreaDescription { name: "Lobby".to_owned() })
        }
    }
}

