use crate::cell::Cell;
use crate::length::point::Point;
use crate::popup::Popup;
use arcstr::ArcStr;
use std::sync::Weak;

#[derive(Clone, Debug)]
pub struct PopupInfo {
    pub title: ArcStr,
    pub position: Cell<Option<Point>>,
    /// Whether the popup may close when unfocused
    pub unimportant: bool,
    this: Weak<Popup>,
}

impl PopupInfo {
    pub fn new(title: ArcStr, this: Weak<Popup>) -> PopupInfo {
        PopupInfo {
            title,
            position: Cell::new(None),
            unimportant: false,
            this,
        }
    }

    pub fn this(&self) -> Weak<Popup> {
        Weak::clone(&self.this)
    }
}

impl PartialEq for PopupInfo {
    fn eq(&self, other: &Self) -> bool {
        let PopupInfo {
            title: self_title,
            position: self_position,
            unimportant: self_unimportant,
            this: _,
        } = self;
        let PopupInfo {
            title: other_title,
            position: other_position,
            unimportant: other_unimportant,
            this: _,
        } = other;

        self_title == other_title
            && self_position == other_position
            && self_unimportant == other_unimportant
    }
}

impl Eq for PopupInfo {}
