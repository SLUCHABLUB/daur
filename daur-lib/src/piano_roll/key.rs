use crate::app::Action;
use crate::key::Key;
use crate::pitch::Pitch;
use crate::ui::{NonZeroLength, Point, Rectangle};
use crate::widget::heterogeneous::{Layers, TwoStack};
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
        let top = if self.pitch.chroma().is_black_key() {
            Solid::BLACK
        } else {
            Solid::WHITE
        };

        let text = Text::bottom_right(if self.pitch.chroma() == self.key.tonic {
            ArcStr::from(self.pitch.name(self.key.sign))
        } else {
            ArcStr::new()
        });

        let bottom = Layers::new((Solid::WHITE, text));

        let constraints = [self.black_key_depth.get().constraint(), Constraint::Fill(1)];

        TwoStack::horizontal((top, bottom), constraints).render(area, buffer, mouse_position);
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {
        // TODO: resizing the piano
        // TODO: plink the key
        // TODO: select all notes with the keys pitch
    }
}
