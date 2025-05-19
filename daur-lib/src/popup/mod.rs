//! Types pertaining to [`Popup`].

mod manager;
mod specification;

pub(crate) use manager::Manager;
pub use specification::Specification;

use crate::View;
use crate::id::Id;
use crate::ui::Rectangle;
use getset::{CloneGetters, CopyGetters};
use std::sync::Arc;

/// An instance of a popup window.
#[derive(Clone, Debug, CopyGetters, CloneGetters)]
pub struct Popup {
    /// The id of the popup.
    #[get_copy = "pub(crate)"]
    id: Id<Popup>,
    /// The area of the popup.
    area: Rectangle,
    /// The view of the popup.
    view: Arc<View>,
}

impl Popup {
    /// Constructs a new `Instance`.
    pub(crate) fn new(id: Id<Popup>, area: Rectangle, view: Arc<View>) -> Popup {
        Popup { id, area, view }
    }

    /// Converts the popup into a [window view](View::Window).
    pub(crate) fn view(&self) -> View {
        View::Window {
            area: self.area,
            view: Arc::clone(&self.view),
        }
    }
}
