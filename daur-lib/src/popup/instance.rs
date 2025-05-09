use crate::View;
use crate::popup::Id;
use crate::ui::Rectangle;
use alloc::sync::Arc;
use getset::{CloneGetters, CopyGetters};

/// An instance of a popup window.
#[derive(Clone, Debug, CopyGetters, CloneGetters)]
pub(crate) struct Instance {
    /// The id of the popup.
    #[get_copy = "pub(crate)"]
    id: Id,
    /// The area of the popup.
    area: Rectangle,
    /// The view of the popup.
    view: Arc<View>,
}

impl Instance {
    /// Constructs a new `Instance`.
    pub(crate) fn new(id: Id, area: Rectangle, view: Arc<View>) -> Instance {
        Instance { id, area, view }
    }

    /// Converts the popup into a [window view](View::Window).
    pub(crate) fn view(&self) -> View {
        View::Window {
            area: self.area,
            view: Arc::clone(&self.view),
        }
    }
}
