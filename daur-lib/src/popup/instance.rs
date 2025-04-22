use crate::View;
use crate::popup::Id;
use crate::ui::Rectangle;
use std::sync::Arc;

/// An instance of a popup window.
#[derive(Clone, Debug)]
pub struct Instance {
    /// The id of the popup.
    pub id: Id,
    /// The area of the popup.
    pub area: Rectangle,
    /// The view of the popup.
    pub view: Arc<View>,
}

impl Instance {
    /// Converts the popup into a [window view](View::Window).
    pub fn into_view(self) -> View {
        View::Window {
            area: self.area,
            view: self.view,
        }
    }
}
