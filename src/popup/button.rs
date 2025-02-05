use crate::app::action::Action;
use crate::id::Id;
use crate::popup::Popup;
use crate::widget::button::Button;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};

/// A simple button
#[derive(Clone, Debug)]
pub struct TerminatingButton {
    pub button: Button,
    /// The id of the containing popup
    pub id: Id<Popup>,
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
            action_queue.push(Action::ClosePopup(self.id));
        }
    }
}
