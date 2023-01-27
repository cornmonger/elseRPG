pub trait CompositionTrait<'e> {
    type Alias;
    type Slot; 

    fn component(&self, alias: Self::Alias) -> Option<&Self::Slot>;
}

pub trait CompositionIteratorTrait<'i, 'e> {
    type Iterator: std::iter::Iterator;

    fn components(&'i self) -> Self::Iterator;
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


