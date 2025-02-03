use crate::app::action::Action;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::widgets::{Block, Padding, Paragraph};

pub struct Button<'a> {
    pub action: Action,
    pub label: &'static str,
    pub description: &'static str,
    pub block: Block<'a>,
}

impl Widget for Button<'_> {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        let content = if area.contains(mouse_position) {
            self.description
        } else {
            self.label
        };

        // - 2 for the border, - 1 to favour the top
        let padding = Padding::top(area.height.saturating_sub(3) / 2);

        // TODO: fix the block situation
        Paragraph::new(content)
            .centered()
            .block(self.block.clone().padding(padding))
            .render(area, buf, mouse_position);
    }

    fn click(&self, _: Rect, button: MouseButton, _: Position, action_queue: &mut Vec<Action>) {
        if button != MouseButton::Left {
            return;
        }

        action_queue.push(self.action);
    }
}
