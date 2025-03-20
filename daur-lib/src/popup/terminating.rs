use crate::app::Action;
use crate::popup::Popup;
use crate::view::{OnClick, View};
use std::sync::Weak;

/// A button that also closes the containing popup
pub fn terminating(content: View, popup: Weak<Popup>) -> View {
    content.on_click(OnClick::from(Action::ClosePopup(popup)))
}
