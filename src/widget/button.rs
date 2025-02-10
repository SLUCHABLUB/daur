use crate::app::action::Action;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect, Size};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use saturating_cast::SaturatingCast;

#[derive(Clone, Default)]
pub struct Button {
    action: Action,
    label: String,
    description: Option<String>,
    bordered: bool,
}

impl Button {
    pub fn new(label: impl Into<String>, action: Action) -> Self {
        Button {
            action,
            label: label.into(),
            description: None,
            bordered: false,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub const fn bordered(mut self) -> Self {
        self.bordered = true;
        self
    }

    pub fn size(&self) -> Size {
        let border = if self.bordered { 2 } else { 0 };
        let width = usize::max(
            self.label.chars().count(),
            self.description
                .as_ref()
                .map_or(0, |description| description.chars().count()),
        )
        .saturating_cast();
        let height = 1 + border;

        Size { width, height }
    }
}

impl Widget for Button {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        let content = if area.contains(mouse_position) {
            self.description.as_deref().unwrap_or(self.label.as_str())
        } else {
            self.label.as_str()
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
