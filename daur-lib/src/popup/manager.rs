use crate::popup::{Instance, Popup};
use crate::{Id, UserInterface};
use getset::Getters;

/// A manager for the open [popups](Popup).
#[derive(Debug, Getters)]
pub(crate) struct Manager {
    #[getset(get = "pub(crate)")]
    popups: Vec<Instance>,
}

impl Manager {
    /// Constructs a new manager with no popups.
    #[must_use]
    pub fn new() -> Self {
        Manager { popups: Vec::new() }
    }

    /// Opens a new [popup](Popup).
    pub fn open<Ui: UserInterface>(&mut self, popup: &Popup, ui: &Ui) {
        let id = Id::generate();

        self.popups.push(popup.instantiate::<Ui>(id, ui));
    }

    /// Closes a [popup](Popup).
    pub fn close(&mut self, popup: Id<Instance>) {
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
