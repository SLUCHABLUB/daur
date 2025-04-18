use daur::popup::Id;
use daur::{Lock, View};
use ratatui::layout::Rect;
use std::sync::Arc;

pub struct PopupHandle {
    popups: Arc<Lock<Vec<(Id, Rect, View)>>>,
    id: Id,
}

impl PopupHandle {
    pub fn new(popups: Arc<Lock<Vec<(Id, Rect, View)>>>, id: Id) -> PopupHandle {
        PopupHandle { popups, id }
    }
}

impl Drop for PopupHandle {
    fn drop(&mut self) {
        let mut popups = self.popups.write();

        if let Some(index) = popups.iter().position(|(id, _, _)| *id == self.id) {
            let popup = popups.remove(index);
            drop(popup);
        }
    }
}
