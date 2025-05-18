use crate::popup::Specification;
use crate::{Id, Popup, UserInterface};
use getset::Getters;

/// A manager for the open [popups](PopupSpecification).
#[derive(Debug, Getters)]
pub(crate) struct Manager {
    #[getset(get = "pub(crate)")]
    popups: Vec<Popup>,
}

impl Manager {
    /// Constructs a new manager with no popups.
    #[must_use]
    pub fn new() -> Self {
        Manager { popups: Vec::new() }
    }

    /// Opens a new [popup](Popup).
    pub fn open<Ui: UserInterface>(&mut self, specification: &Specification, ui: &Ui) {
        let id = Id::generate();

        self.popups.push(specification.instantiate::<Ui>(id, ui));
    }

    /// Closes a [popup](Popup).
    pub fn close(&mut self, popup: Id<Popup>) {
        if let Some(index) = self
            .popups
            .iter()
            .position(|instance| instance.id() == popup)
        {
            let popup = self.popups.remove(index);
            drop(popup);
        }
    }
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}
