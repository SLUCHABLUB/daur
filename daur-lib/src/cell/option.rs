use crate::cell::clone::CloneCell;
use std::sync::Arc;

/// A cell maybe containing an [atomically reference counted](Arc) value.
pub type OptionArcCell<T> = CloneCell<Option<Arc<T>>>;

impl<T: ?Sized> OptionArcCell<T> {
    /// Construct a new cell from a pointer.
    #[must_use]
    pub const fn some(value: Arc<T>) -> OptionArcCell<T> {
        OptionArcCell::new(Some(value))
    }

    /// Constructs a new empty cell.
    #[must_use]
    pub const fn none() -> OptionArcCell<T> {
        OptionArcCell::new(None)
    }

    /// Sets the pointer [`None`].
    pub fn set_none(&self) {
        self.set(None);
    }

    /// Sets the pointer to a new value.
    pub fn set_some(&self, value: Arc<T>) {
        self.set(Some(value));
    }

    /// See [`Option::get_or_insert_with`].
    pub fn get_or_insert_with<F: FnOnce() -> Arc<T>>(&self, f: F) -> Arc<T> {
        Arc::clone(self.lock_ref().write().get_or_insert_with(f))
    }
}

impl<T> OptionArcCell<T> {
    /// Construct a new cell from an optional value.
    #[must_use]
    pub fn from_value(value: Option<T>) -> OptionArcCell<T> {
        OptionArcCell::new(value.map(Arc::new))
    }

    /// Sets the optional value.
    pub fn set_value(&self, value: Option<T>) {
        self.set(value.map(Arc::new));
    }

    /// Sets the optional value.
    pub fn set_some_value(&self, value: T) {
        self.set_some(Arc::new(value));
    }

    /// Like [`OptionArcCell::get_or_insert_with`] but the value is automatically wrapped in a pointer.
    pub fn get_or_insert_value_with<F: FnOnce() -> T>(&self, f: F) -> Arc<T> {
        self.get_or_insert_with(|| Arc::new(f()))
    }
}
