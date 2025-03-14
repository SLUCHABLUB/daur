use crate::app::Action;
use crate::pitch::Pitch;
use crate::ui::{Point, Rectangle};
use crate::widget::{Solid, Widget};
use crate::Clip;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::style::Color;
use std::sync::Arc;

#[derive(Debug)]
pub struct Row {
    pub clip: Arc<Clip>,
    pub pitch: Pitch,
}

impl Widget for Row {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        // TODO: draw notes
        // TODO: draw grid
        // TODO: highlight key based on settings
        let colour = if (self.pitch - Pitch::A440).semitones() % 2 == 0 {
            Color::Gray
        } else {
            Color::DarkGray
        };

        Solid { colour }.render(area, buffer, mouse_position);
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {
        // TODO: add notes
        // TODO: select notes
        // TODO: move cursor
        let _ = &self.clip;
    }
}
