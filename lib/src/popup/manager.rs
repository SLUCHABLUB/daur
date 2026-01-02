use crate::Id;
use crate::Popup;
use crate::UserInterface;
use crate::popup::Specification;
use crate::ui::Rectangle;
use indexmap::IndexMap;
use parking_lot::Mutex;

/// A manager for the open [popups](PopupSpecification).
/// It uses internal mutability so that multiple threads can open popups concurrently.
/// This is required to handle errors in audio rendering.
#[derive(Debug, Default)]
pub(crate) struct Manager {
    popups: Mutex<IndexMap<Id<Popup>, Popup>>,
}

impl Manager {
    /// Constructs a new manager with no popups.
    #[must_use]
    pub fn new() -> Self {
        Manager::default()
    }

    pub(crate) fn popups(&self) -> impl Iterator<Item = Popup> {
        self.popups.lock().clone().into_values()
    }

    /// Closes a [popup](Popup).
    pub fn close(&self, id: Id<Popup>) {
        let popup = self.popups.lock().shift_remove(&id);
        drop(popup);
    }

    pub fn transform_popup<F>(&self, id: Id<Popup>, transformer: F)
    where
        F: FnOnce(Rectangle) -> Rectangle,
    {
        let mut popups = self.popups.lock();

        let Some(popup) = popups.get_mut(&id) else {
            return;
        };

        popup.area = transformer(popup.area);
    }
}

impl Manager {
    /// Opens a new [popup](Popup).
    pub fn open<Ui: UserInterface>(&self, specification: &Specification, ui: &Ui) {
        let id = specification.generate_id();

        self.popups
            .lock()
            .insert(id, specification.instantiate::<Ui>(id, ui));
    }
}
