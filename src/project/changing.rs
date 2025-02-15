use crate::cell::Cell;
use crate::locked_tree::LockedTree;
use crate::time::instant::Instant;

#[derive(Clone, Default)]
pub struct Changing<T: Copy> {
    pub start: Cell<T>,
    pub changes: LockedTree<Instant, T>,
}

impl<T: Copy> Changing<T> {
    pub fn get(&self, instant: Instant) -> T {
        self.changes.get_lte(instant).unwrap_or(self.start.get())
    }
}

impl<T: Copy> From<T> for Changing<T> {
    fn from(start: T) -> Self {
        Changing {
            start: Cell::new(start),
            changes: LockedTree::new(),
        }
    }
}
