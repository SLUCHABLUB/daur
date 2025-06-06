use crate::popup::Specification;
use crate::{Id, Popup, UserInterface};
use indexmap::IndexMap;

/// A manager for the open [popups](PopupSpecification).
#[derive(Debug, Default)]
pub(crate) struct Manager {
    popups: IndexMap<Id<Popup>, Popup>,
}

impl Manager {
    /// Constructs a new manager with no popups.
    #[must_use]
    pub fn new() -> Manager {
        Manager::default()
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
    pub fn close(&mut self, id: Id<Popup>) {
        let popup = self.popups.shift_remove(&id);
        drop(popup);
    }
}
