use crossbeam::atomic::AtomicCell;

#[derive(Default)]
pub struct Cell<T> {
    inner: AtomicCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            inner: AtomicCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        self.inner.store(value);
    }
}

impl<T: Copy> Cell<T> {
    pub fn get(&self) -> T {
        self.inner.load()
    }
}

impl<T: Default> Cell<T> {
    pub fn take(&self) -> T {
        self.inner.take()
    }
}

impl<T: Copy> Clone for Cell<T> {
    fn clone(&self) -> Self {
        Cell::new(self.get())
    }
}

impl<T: Copy + PartialEq> PartialEq for Cell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl<T: Copy + Eq> Eq for Cell<T> {}
