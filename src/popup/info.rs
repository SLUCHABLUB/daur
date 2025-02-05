use crate::id::Id;
use crate::popup::Popup;
use ratatui::layout::Position;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PopupInfo {
    pub title: String,
    id: Id<Popup>,
    pub position: Option<Position>,
    /// Whether the popup may close when unfocused
    pub unimportant: bool,
}

impl PopupInfo {
    pub fn new(title: String) -> PopupInfo {
        PopupInfo {
            title,
            id: Id::new(),
            position: None,
            unimportant: false,
        }
    }

    pub fn id(&self) -> Id<Popup> {
        self.id
    }
}
