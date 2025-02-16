use crate::app::action::Action;
use crate::track::Track;
use crate::widget::block::Block;
use crate::widget::to_widget::ToWidget;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::widgets::Paragraph;
use std::sync::Arc;

pub struct Settings {
    pub track: Arc<Track>,
    pub selected: bool,
    pub index: usize,
}

impl Settings {
    fn visual(&self) -> impl Widget {
        let block = Block::new(self.track.name.clone(), self.selected);
        Paragraph::new("TODO").block(block.to_widget())
    }
}

impl Widget for Settings {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        self.visual().render(area, buf, mouse_position);
    }

    fn click(
        &self,
        area: Rect,
        button: MouseButton,
        position: Position,
        action_queue: &mut Vec<Action>,
    ) {
        action_queue.push(Action::SelectTrack(self.index));
        self.visual().click(area, button, position, action_queue);
    }
}
