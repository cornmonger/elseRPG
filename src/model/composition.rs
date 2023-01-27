pub trait CompositionTrait<'e> {
    type Alias;
    type Slot; 
    type Iterator: std::iter::Iterator;

    fn component(&self, alias: Self::Alias) -> Option<&Self::Slot>;
    fn components(&'e self) -> Self::Iterator;
}

pub trait ComponentTrait {
    type Type;

    fn get(&self) -> &Self::Type;
}

pub struct Component<T> {
    pub(crate) object: T
}

impl<T> ComponentTrait for Component<T> {
    type Type = T;

    fn get(&self) -> &Self::Type {
        &self.object
    }
}


