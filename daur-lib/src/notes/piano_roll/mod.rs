mod key;
mod row;

use crate::app::Action;
use crate::interval::Interval;
use crate::notes::piano_roll::key::Key;
use crate::notes::piano_roll::row::Row;
use crate::pitch::Pitch;
use crate::ui::{NonZeroLength, Point, Rectangle};
use crate::widget::heterogeneous::TwoStack;
use crate::widget::homogenous::Stack;
use crate::widget::Widget;
use crate::Clip;
use crossterm::event::MouseButton;
use itertools::chain;
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint;
use saturating_cast::SaturatingCast as _;
use std::iter::{once, repeat_n};
use std::sync::Arc;

// The piano roll has a fixed lower pitch.
// Resizing it will thus cause the bottom to be fixed.
// Since the top is the thing you move this seems intuitive.
pub struct PianoRoll {
    pub clip: Arc<Clip>,

    pub key_height: NonZeroLength,
    pub piano_depth: NonZeroLength,
    pub black_key_depth: NonZeroLength,

    pub lowest_pitch: Pitch,
}

impl PianoRoll {
    fn constraints(&self, key_count: usize) -> impl Iterator<Item = Constraint> {
        // since the highest key might be cut off, we use a ::Fill for it
        chain(
            once(Constraint::Fill(1)),
            repeat_n(
                self.key_height.get().constraint(),
                key_count.saturating_sub(1),
            ),
        )
    }
}

impl Widget for PianoRoll {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        let key_count = (area.height / self.key_height).ceil().saturating_cast();
        let constraints = self.constraints(key_count);

        Stack::vertical(constraints.enumerate().map(|(index, constraint)| {
            let interval = Interval::from_semitones(index.saturating_cast());
            let key = Key {
                pitch: self.lowest_pitch + interval,
                black_key_depth: self.black_key_depth,
            };
            let row = Row {
                clip: Arc::clone(&self.clip),
            };

            let constraints = [self.piano_depth.get().constraint(), Constraint::Fill(1)];

            let stack = TwoStack::horizontal((key, row), constraints);

            (stack, constraint)
        }))
        .render(area, buf, mouse_position);
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {
        // TODO: resizing the piano
        // TODO: selecting notes
    }
}
