use crate::popup::Specification;
use crate::ui::Rectangle;
use crate::{Id, Popup, UserInterface};
use indexmap::IndexMap;
use parking_lot::Mutex;

/// A manager for the open [popups](PopupSpecification).
/// It uses internal mutability so that multiple threads can open popups concurrently.
/// This is required to handle errors in audio rendering.
#[derive(Debug)]
pub(crate) struct Manager<Ui: 'static> {
    popups: Mutex<IndexMap<Id<Popup>, Popup>>,
    // TODO: remove
    ui: &'static Ui,
}

impl<Ui> Manager<Ui> {
    /// Constructs a new manager with no popups.
    #[must_use]
    pub fn new(ui: &'static Ui) -> Self {
        Manager {
            popups: Mutex::new(IndexMap::new()),
            ui,
        }
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

impl<Ui: UserInterface> Manager<Ui> {
    /// Opens a new [popup](Popup).
    pub fn open(&self, specification: &Specification) {
        let id = specification.generate_id();

        self.popups
            .lock()
            .insert(id, specification.instantiate::<Ui>(id, self.ui));
    }
}
