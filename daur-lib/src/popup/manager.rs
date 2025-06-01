use crate::popup::{Id, Specification};
use crate::{Popup, UserInterface};
use indexmap::IndexMap;

/// A manager for the open [popups](PopupSpecification).
#[derive(Debug)]
pub(crate) struct Manager {
    popups: IndexMap<Id, Popup>,
}

impl Manager {
    /// Constructs a new manager with no popups.
    #[must_use]
    pub fn new() -> Self {
        Manager {
            popups: IndexMap::new(),
        }
    }

    pub(crate) fn popups(&self) -> impl Iterator<Item = &Popup> {
        self.popups.values()
    }

    /// Opens a new [popup](Popup).
    pub fn open<Ui: UserInterface>(&mut self, specification: &Specification, ui: &Ui) {
        let id = Id::generate();

        self.popups
            .insert(id, specification.instantiate::<Ui>(id, ui));
    }

    /// Closes a [popup](Popup).
    pub fn close(&mut self, id: Id) {
        let popup = self.popups.shift_remove(&id);
        drop(popup);
    }
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}
