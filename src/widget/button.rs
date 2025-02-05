use crate::app::action::Action;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

// TODO: remove pub from fields
#[derive(Clone, Debug, Default)]
pub struct Button {
    pub action: Action,
    pub label: &'static str,
    pub description: &'static str,
    pub bordered: bool,
}

impl Button {
    pub fn new(label: &'static str, action: Action) -> Self {
        Button {
            action,
            label,
            description: label,
            bordered: false,
        }
    }
}

impl Widget for Button {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        let content = if area.contains(mouse_position) {
            self.description
        } else {
            self.label
        };

        // - 2 for the border, - 1 to favour the top
        let padding = Padding::top(area.height.saturating_sub(3) / 2);

        let mut block = Block::new().padding(padding);

        if self.bordered {
            block = block.borders(Borders::ALL);
        }

        Paragraph::new(content)
            .centered()
            .block(block)
            .render(area, buf, mouse_position);
    }

    fn click(&self, _: Rect, button: MouseButton, _: Position, action_queue: &mut Vec<Action>) {
        if button != MouseButton::Left {
            return;
        }

        action_queue.push(self.action.clone());
    }
}
