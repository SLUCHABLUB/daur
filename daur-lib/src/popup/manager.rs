use crate::UserInterface;
use crate::lock::Lock;
use crate::popup::{Id, Popup};
use std::collections::HashMap;

/// A manager for the open [popups](Popup).
#[derive(Debug)]
pub struct Manager<Ui: UserInterface> {
    handles: Lock<HashMap<Id, Ui::PopupHandle>>,
}

impl<Ui: UserInterface> Manager<Ui> {
    /// Constructs a new manager with no popups.
    #[must_use]
    pub fn new() -> Self {
        Manager {
            handles: Lock::new(HashMap::new()),
        }
    }

    /// Opens a new [popup](Popup).
    pub fn open(&self, popup: &Popup, ui: &Ui) {
        let id = Id::generate();
        let handle = ui.open_popup(popup.title(), popup.view(id), id);
        self.handles.write().insert(id, handle);
    }

    /// Closes a [popup](Popup).
    pub fn close(&self, popup: Id, ui: &Ui) {
        if let Some(handle) = self.handles.write().remove(&popup) {
            ui.close_popup(handle);
        }
    }
}

impl<Ui: UserInterface> Default for Manager<Ui> {
    fn default() -> Self {
        Self::new()
    }
}
