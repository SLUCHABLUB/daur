//! Types pertaining to [`Popup`].

mod id;
mod manager;
mod specification;

pub use id::Id;
pub use specification::Specification;

pub(crate) use manager::Manager;

use crate::View;
use crate::ui::{Point, Rectangle};
use getset::{CloneGetters, CopyGetters};
use std::sync::Arc;

/// An instance of a popup window.
#[derive(Clone, Debug, CopyGetters, CloneGetters)]
pub struct Popup {
    /// The area of the popup.
    area: Rectangle,
    /// The view of the popup.
    view: Arc<View>,
}

impl Popup {
    /// Constructs a new `Instance`.
    pub(crate) fn new(view: Arc<View>, area: Rectangle) -> Popup {
        Popup { area, view }
    }

    /// Converts the popup into a [window view](View::Window).
    pub(crate) fn view(&self) -> View {
        // We call `.relative_to(Point::ZERO)` since popups are positioned absolutely.
        View::Shared(Arc::clone(&self.view))
            .quotated_2d(self.area.size)
            .positioned(self.area.position.relative_to(Point::ZERO))
    }
}
