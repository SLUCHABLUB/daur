use crate::app::action::Action;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::widgets::Paragraph;

pub struct Placeholder(pub &'static str);

impl Widget for Placeholder {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        Paragraph::new(self.0)
            .centered()
            .render(area, buf, mouse_position);
    }

    fn click(&self, _: Rect, _: MouseButton, _: Position, _: &mut Vec<Action>) {}
}
