use crate::lock::Lock;
use crate::popup::Popup;
use std::sync::{Arc, Weak};

/// A manager for the open [popups](Popup).
#[derive(Debug)]
pub struct Popups {
    list: Lock<Vec<Arc<Popup>>>,
}

impl Popups {
    /// Constructs a new [popup manager](Popups).
    #[must_use]
    pub fn new() -> Popups {
        Popups {
            list: Lock::new(Vec::new()),
        }
    }

    /// Opens a new [popup](Popup).
    pub fn open(&self, popup: Arc<Popup>) {
        self.list.write().push(popup);
    }

    /// Closes a [popup](Popup).
    pub fn close(&self, popup: &Weak<Popup>) {
        let Some(arc) = Weak::upgrade(popup) else {
            return;
        };

        let mut list = self.list.write();

        let Some(index) = list.iter().position(|popup| Arc::ptr_eq(popup, &arc)) else {
            return;
        };

        list.remove(index);
    }

    /// Returns the frontmost [popup](Popup).
    pub fn top(&self) -> Option<Arc<Popup>> {
        self.list.read().last().map(Arc::clone)
    }

    /// Returns all the popups in a [vector](Vec) ordered back to front.
    pub fn to_stack(&self) -> Vec<Arc<Popup>> {
        self.list.read().clone()
    }
}

impl Default for Popups {
    fn default() -> Self {
        Self::new()
    }
}
