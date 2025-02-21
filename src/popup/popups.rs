use crate::lock::Lock;
use crate::popup::Popup;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub struct Popups {
    list: Lock<Vec<Arc<Popup>>>,
}

impl Popups {
    #[must_use]
    pub fn new() -> Popups {
        Popups {
            list: Lock::new(Vec::new()),
        }
    }

    pub fn open(&self, popup: Arc<Popup>) {
        self.list.write().push(popup);
    }

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

    pub fn top(&self) -> Option<Arc<Popup>> {
        self.list.read().last().map(Arc::clone)
    }

    pub fn to_stack(&self) -> Vec<Arc<Popup>> {
        self.list.read().clone()
    }
}

impl Default for Popups {
    fn default() -> Self {
        Self::new()
    }
}
