use crate::app::Action;
use crate::key::Key;
use crate::pitch::Pitch;
use crate::ui::{NonZeroLength, Point, Rectangle};
use crate::widget::heterogeneous::TwoStack;
use crate::widget::{Solid, Text, Widget};
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint;

pub struct PianoKey {
    pub key: Key,
    pub pitch: Pitch,
    pub black_key_depth: NonZeroLength,
}

impl Widget for PianoKey {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let black_part = if self.pitch.chroma().is_black_key() {
            Solid::BLACK
        } else {
            Solid::WHITE
        };

        // TODO: only do this for the tonic
        let white_part = Text::bottom_right(ArcStr::from(self.pitch.name(self.key.sign)));

        let constraints = [self.black_key_depth.get().constraint(), Constraint::Fill(1)];

        TwoStack::horizontal((black_part, white_part), constraints).render(
            area,
            buffer,
            mouse_position,
        );
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {
        // TODO: plink the key
        // TODO: select all notes with the keys pitch
    }
}
