
impl<'e> Attachable<'e, HumanoidModel<'e>, HumanoidComponents> for Player<'e, HumanoidModel<'e>>  {
    fn attached(&self, component: HumanoidComponents) -> Option<Box<&dyn EntityTrait<'e>>> {
        self.character.entity.components.as_ref().unwrap().get(component)
    }
}

impl HumanoidComponents {
    pub fn name(&self) -> &'static str {
        match self {
            HumanoidComponents::Head => "head",
            HumanoidComponents::Back => "back"
        }
    }
}


impl<'e> HumanoidModel<'e> {
    pub fn new(zone: &mut Zone) -> HumanoidModel<'e> {
        HumanoidModel::<'e> {
            head: None,
            back: Some(
                    Entity::<'e, NoComponentModel> {
                        id: zone.next_id(),
                        name: "Backpack",
                        max_health: 100,
                        health: 100,
                        max_resist: 0,
                        resist: 0,
                        max_ability: 0,
                        ability: 100,
                        contains: None,
                        components: None
                    }
            ) 
        }
    }

    pub fn new_player(zone: &mut Zone) -> Player<'e, HumanoidModel<'e>> {
        Player {
                character: Character {
                    entity: Entity {
                        id: zone.next_id(),
                        name: "Player",
                        max_health: 100,
                        health: 100,
                        max_resist: 100,
                        resist: 0,
                        max_ability: 100,
                        ability: 100,
                        contains: None,
                        components: Some(HumanoidModel::new(zone))
                    }
                }
            }
    }

    pub fn new_npc(zone: &mut Zone) -> NPC<'e, HumanoidModel<'e>> {
        NPC {
                character: Character {
                    entity: Entity {
                        id: zone.next_id(),
                        name: "Troll",
                        max_health: 40,
                        health: 40,
                        max_resist: 0,
                        resist: 0,
                        max_ability: 0,
                        ability: 0,
                        contains: None,
                        components: Some(HumanoidModel::new(zone))
                    }
                }
            }
    }

}

pub struct HumanoidModel<'e> {
    head: Option<Entity<'e, NoComponentModel>>,
    back: Option<Entity<'e, NoComponentModel>>
}

pub enum HumanoidComponents {
    Head,
    Back 
}


impl<'e> ComponentModel<'e> for HumanoidModel<'e> {
    type Components = HumanoidComponents;

    fn get(&self, component: Self::Components) -> Option<Box<&dyn EntityTrait<'e>>> {
        match component {
            HumanoidComponents::Head => Some(Box::new(self.head.as_ref().unwrap())),
            HumanoidComponents::Back => Some(Box::new(self.back.as_ref().unwrap()))
        }
    }
}