use crate::app::Action;
use crate::pitch::Pitch;
use crate::ui::{NonZeroLength, Point, Rectangle};
use crate::widget::heterogeneous::TwoStack;
use crate::widget::solid::Solid;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint;

pub struct Key {
    pub pitch: Pitch,
    pub black_key_depth: NonZeroLength,
}

impl Widget for Key {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        let black_part = if self.pitch.chroma().is_black_key() {
            Solid::BLACK
        } else {
            Solid::WHITE
        };

        let constraints = [self.black_key_depth.get().constraint(), Constraint::Fill(1)];

        TwoStack::horizontal((black_part, Solid::WHITE), constraints).render(
            area,
            buf,
            mouse_position,
        );
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {
        // TODO: plink the key
        // TODO: select all notes with the keys pitch
    }
}
