pub mod zone;
pub mod area;
pub mod entity;
pub mod character;
pub mod humanoid;

pub trait DescriptionTrait {
    fn name(&self) -> &str;
    fn rename(&mut self, name: String);
}