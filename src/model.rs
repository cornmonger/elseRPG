pub mod template;

pub enum Prototype<D: Clone + 'static> {
    // No data exists.
    None,
    // The local context mutably owns the data (D).
    Local (D),  
    // A template object currently owns the immutable data (D). Mutability is possible through the Clone trait and a transition to Local.
    Dynamic (D),  
    // A template object permanently owns the immutable data (D).
    Fixed (D)  
}

pub trait DescriptionTrait<'d> {
    fn name(&self) -> &'d str;
}

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

    fn description(&self) -> &ZoneDescription {
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

pub trait EntityTrait<'e, T: EntityTemplateTrait<'e>> {
    fn id(&self) -> u64;
    fn template(&self) -> Option<T>;
    fn permeability(&self) -> Option<Permeability>;
    fn description(&self) -> Prototype<EntityDescription>;
    fn components(&self) -> Prototype<T>;
    fn contents(&self) -> Prototype<Vec<Box<dyn EntityTrait<'e, T>>>>;
}

pub struct Entity<'e, T: EntityTemplateTrait<'e>> {
    id: u64,
    template: Option<T>,
    permeability: Option<Permeability>,
    description: Prototype<Description<'e>>,
    components: Prototype<T>,
    contains: Prototype<Vec<Box<dyn EntityTrait<'e,T>>>>
}

pub struct EntityDescription<'e> {
    name: &'e str
}

impl<'e, T: EntityTemplateTrait> DescriptionTrait for Entity<'e, T> {
    fn name(&self) -> &'d str {
        self.name
    }
}

pub trait PermeabilityTrait {
    fn max_health(&self) -> u16;
    fn max_resist(&self) -> u16;
    fn max_ability(&self) -> u16;
    fn health(&self) -> u16;
    fn resist(&self) -> u16;
    fn ability(&self) -> u16;
}

pub struct Permeability {
    max_health: u16,
    health: u16,
    max_resist: u16,
    resist: u16,
    ability: u16,
    max_ability: u16,
}

impl PermeabilityTrait for Permeability {
    fn max_health(&self) -> u16 {
        self.max_health
    }

    fn max_resist(&self) -> u16 {
        self.max_resist
    }

    fn max_ability(&self) -> u16 {
        self.max_ability
    }

    fn health(&self) -> u16 {
        self.health
    }

    fn resist(&self) -> u16 {
        self.resist
    }

    fn ability(&self) -> u16 {
        self.ability
    }
}

pub trait EntityTemplateTrait<'e> {
    type ComponentTemplate;

    fn components(&self) -> Box<dyn Self::ComponentTemplate<'e>>;
}

pub struct EntityTemplate<'e> {
    components: ComponentModel<'e>
}

impl EntityTemplateTrait for EntityTemplate {
    fn model(&self) -> ComponentModel<'e> {
        self.model
    }
}

pub struct Character<'e, M: ComponentModel<'e>> {
    entity: Entity<'e, M>,
}

pub struct NPC<'e, M: ComponentModel<'e>> {
    character: Character<'e, M>
}

pub struct Player<'e, M: ComponentModel<'e>> {
    character: Character<'e, M>
}

pub enum ComponentType<'e> {
    Entity (EntityComponent<'e, M: ComponentModel>),
    Logical (LogicalComponent<'e>)
}

pub enum NoComponents {}

pub struct NoComponentModel {

}

pub struct EntityComponent<'e, M: ComponentModel<'e>> {
    entity: Entity<'e, M>,
    subcomponents: Option<Box<Component>>
}

pub struct LogicalComponent<'e> {

}

pub trait ComponentModel<'e> {
    type Components;

    fn get(&self, component: Self::Components) -> Option<Box<&dyn EntityTrait<'e>>>;
}

impl<'e, M: ComponentModel<'e>> EntityTrait<'e> for Entity<'e, M> {
    fn name(&self) -> &'e str {
        self.name
    }
}

impl<'e> ComponentModel<'e> for NoComponentModel {
    type Components = NoComponents;

    fn get(&self, _component: Self::Components) -> Option<Box<&dyn EntityTrait<'e>>> {
        None
    }
}

pub trait Component<'e> {
    fn components(&self) -> Option<Box<dyn Component<'e>>>;
    fn get(&self) -> ComponentType<'e>;
}

impl<'e, M: ComponentModel<'e>> Component for EntityComponent<'e, M> {
}

impl<'e, M: ComponentModel<'e>> EntityComponent<'e, M> {
    pub fn entity(&self) -> &Entity<'e, M> {
        &self.entity
    }
}

pub trait Attachable<'e, M, E> {
    fn attached(&self, component: E) -> Option<Box<&dyn EntityTrait<'e>>>;
}

impl<'e, M: ComponentModel<'e>> Player<'e, M> {
    pub fn name(&self) -> &'e str {
        self.character.entity.name
    }
}

impl<'a> Zone<'a> {
    pub fn new() -> Zone<'a> {
        Zone {
            id: 0,
            name: "World",
            serial_id: 0
        }
    }

    pub fn next_id(&mut self) -> u64 {
        self.serial_id += 0;
        self.serial_id
    }

    pub fn name(&self) -> &'a str {
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

impl<'e, M: ComponentModel<'e>> NPC<'e, M> {
    pub fn name(&self) -> &'e str {
        self.character.entity.name
    }
}