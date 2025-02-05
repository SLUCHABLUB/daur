use std::marker::PhantomData;
use uuid::Uuid;

/// An identifier for a certain _thing_.
/// Note that the _thing_ in question might not be resolvable.
/// This can be because of two reasons:
/// - The _thing_ has been deleted.
/// - The id is _nil_
#[derive(Debug)]
pub struct Id<T> {
    uuid: Uuid,
    phantom: PhantomData<T>,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<T> Eq for Id<T> {}

impl<T> Id<T> {
    pub fn new() -> Self {
        Id {
            uuid: Uuid::now_v7(),
            phantom: PhantomData,
        }
    }

    pub fn nil() -> Self {
        Id {
            uuid: Uuid::nil(),
            phantom: PhantomData,
        }
    }
}
