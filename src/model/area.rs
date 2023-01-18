pub trait AreaTrait<'a> {
    fn id(&self) -> u64;
    fn description(&self) -> &AreaDescription<'z>;
}

pub struct Area<'a> {
    id: u64,
    description: AreaDescription<'z>
}

impl<'a> AreaTrait<'a> for Area<'a> {
    fn id(&self) -> u64 {
        self.id
    }

    fn description(&self) -> &AreaDescription<'z> {
        &self.description
    }
}

struct AreaDescription<'a> {
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
            id: zone.next_id(),
            name: "Lobby"
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }
}

