pub mod composition;
pub mod zone;
pub mod area;
pub mod entity;
pub mod character;
pub mod template;

pub trait DescriptionTrait {
    fn name(&self) -> &str;
}