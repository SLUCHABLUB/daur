use crate::app::action::Action;
use crate::popup::Popup;
use crate::widget::button::Button;
use crate::widget::sized::Sized;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect, Size};
use std::sync::Weak;

/// A simple button
#[derive(Clone)]
pub struct TerminatingButton {
    pub button: Button,
    /// The id of the containing popup
    pub popup: Weak<Popup>,
}

impl Widget for TerminatingButton {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        self.button.render(area, buf, mouse_position);
    }

    fn click(
        &self,
        area: Rect,
        button: MouseButton,
        position: Position,
        action_queue: &mut Vec<Action>,
    ) {
        self.button.click(area, button, position, action_queue);
        if button == MouseButton::Left {
            action_queue.push(Action::ClosePopup(Weak::clone(&self.popup)));
        }
    }
}

impl PartialEq for TerminatingButton {
    fn eq(&self, other: &Self) -> bool {
        let TerminatingButton {
            button: self_button,
            popup: _,
        } = self;
        let TerminatingButton {
            button: other_button,
            popup: _,
        } = other;

        self_button == other_button
    }
}

impl Eq for TerminatingButton {}

impl Sized for TerminatingButton {
    fn size(&self) -> Size {
        self.button.size()
    }
}
