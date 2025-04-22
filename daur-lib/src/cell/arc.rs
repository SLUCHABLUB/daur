use crate::cell::clone::CloneCell;
use std::sync::Arc;

/// A cell containing an [atomically reference counted](Arc) value.
pub type ArcCell<T> = CloneCell<Arc<T>>;

impl<T: Sized> ArcCell<T> {
    /// Construct a new cell from a value.
    pub fn from_value(value: T) -> ArcCell<T> {
        ArcCell::new(Arc::new(value))
    }

    /// Sets the value.
    pub fn set_value(&self, value: T) {
        self.set(Arc::new(value));
    }
}
