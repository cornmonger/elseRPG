pub mod composition;
pub mod zone;
pub mod area;
pub mod entity;
pub mod character;
pub mod template;

/* TODO: pub enum Prototype<D: Clone + 'static> { */
pub enum Prototype<D> {
    // No data exists.
    None,
    // The local context mutably owns the data (D).
    Local (D),  
    // A template object currently owns the immutable data (D). Mutability is possible through the Clone trait and a transition to Local.
    Dynamic (D),  
    // A template object permanently owns the immutable data (D).
    Fixed (D)  
}

impl<D> Prototype<D> {
    pub fn unwrap(&self) -> &D {
        match self {
            Self::Local(d) => d,
            Self::Dynamic(d) => d,
            Self::Fixed(d) => d,
            _ => { panic!("Unable to unwrap Prototype object!"); }
        }
    }
}

pub trait DescriptionTrait<'d> {
    fn name(&self) -> &'d str;
}