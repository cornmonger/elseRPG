pub mod zone;
pub mod area;
pub mod entity;
pub mod character;
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