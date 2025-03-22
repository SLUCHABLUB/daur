use crate::Cell;
use crate::popup::Popup;
use crate::ui::Point;
use arcstr::ArcStr;
use std::sync::Weak;

#[derive(Clone, Debug)]
pub struct PopupInfo {
    title: ArcStr,
    pub position: Cell<Option<Point>>,
    this: Weak<Popup>,
}

impl PopupInfo {
    pub fn new(title: ArcStr, this: Weak<Popup>) -> PopupInfo {
        PopupInfo {
            title,
            position: Cell::new(None),
            this,
        }
    }

    // TODO: derive
    pub fn title(&self) -> ArcStr {
        ArcStr::clone(&self.title)
    }

    // TODO: derive
    pub fn this(&self) -> Weak<Popup> {
        Weak::clone(&self.this)
    }
}
