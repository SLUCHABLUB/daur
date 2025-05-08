use crate::UserInterface;
use crate::lock::Lock;
use crate::popup::{Id, Instance, Popup};

/// A manager for the open [popups](Popup).
#[derive(Debug)]
pub(crate) struct Manager {
    popups: Lock<Vec<Instance>>,
}

impl Manager {
    /// Constructs a new manager with no popups.
    #[must_use]
    pub fn new() -> Self {
        Manager {
            popups: Lock::new(Vec::new()),
        }
    }

    /// Opens a new [popup](Popup).
    pub fn open<Ui: UserInterface>(&self, popup: &Popup, ui: &Ui) {
        let id = Id::generate();

        self.popups.write().push(popup.instantiate::<Ui>(id, ui));
    }

    /// Closes a [popup](Popup).
    pub fn close(&self, popup: Id) {
        let mut popups = self.popups.write();

        if let Some(index) = popups.iter().position(|instance| instance.id() == popup) {
            let popup = popups.remove(index);
            drop(popup);
        }
    }

    pub(crate) fn to_vec(&self) -> Vec<Instance> {
        self.popups.read().clone()
    }
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}
