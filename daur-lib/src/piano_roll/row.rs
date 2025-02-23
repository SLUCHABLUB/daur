use crate::app::Action;
use crate::ui::{Point, Rectangle};
use crate::widget::Widget;
use crate::Clip;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use std::sync::Arc;

pub struct Row {
    pub clip: Arc<Clip>,
}

impl Widget for Row {
    fn render(&self, _: Rectangle, _: &mut Buffer, _: Point) {
        // TODO: draw notes
        // TODO: draw grid
        let _ = &self.clip;
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {
        // TODO: add notes
        // TODO: select notes
        // TODO: move cursor
    }
}
