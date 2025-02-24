mod key;
mod row;
mod settings;

pub use settings::PianoRollSettings;

use crate::app::Action;
use crate::interval::Interval;
use crate::key::Key;
use crate::piano_roll::key::PianoKey;
use crate::piano_roll::row::Row;
use crate::pitch::Pitch;
use crate::project::changing::Changing;
use crate::ui::{Mapping, Point, Rectangle};
use crate::widget::heterogeneous::TwoStack;
use crate::widget::homogenous::Stack;
use crate::widget::{Text, Widget};
use crate::Clip;
use arcstr::{literal, ArcStr};
use crossterm::event::MouseButton;
use itertools::chain;
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint;
use saturating_cast::SaturatingCast as _;
use std::iter::{once, repeat_n};
use std::sync::Arc;

const NO_CLIP_SELECTED: ArcStr = literal!("please select a clip to edit");

// The piano roll has a fixed lower pitch.
// Resizing it will thus cause the bottom to be fixed.
// Since the top is the thing you move this seems intuitive.
pub struct PianoRoll {
    pub clip: Option<Arc<Clip>>,

    pub mapping: Mapping,
    pub settings: PianoRollSettings,

    pub key: Arc<Changing<Key>>,
    pub lowest_pitch: Pitch,
}

impl PianoRoll {
    fn constraints(&self, key_count: usize) -> impl Iterator<Item = Constraint> {
        // since the highest key might be cut off, we use a ::Fill for it
        chain(
            once(Constraint::Fill(1)),
            repeat_n(
                self.settings.key_width.get().constraint(),
                key_count.saturating_sub(1),
            ),
        )
    }
}

impl Widget for PianoRoll {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let Some(clip) = self.clip.as_ref().map(Arc::clone) else {
            Text::centered(NO_CLIP_SELECTED).render(area, buffer, mouse_position);
            return;
        };

        let roll_start = self
            .mapping
            .instant(area.x + self.settings.piano_depth.get());
        let piano_key_key = self.key.get(roll_start);

        let key_count = (area.height / self.settings.key_width)
            .ceil()
            .saturating_cast();
        let constraints = self.constraints(key_count);

        Stack::vertical(constraints.enumerate().map(|(index, constraint)| {
            let interval = Interval::from_semitones(index.saturating_cast());
            let key = PianoKey {
                key: piano_key_key,
                pitch: self.lowest_pitch + interval,
                black_key_depth: self.settings.black_key_depth,
            };
            let row = Row {
                clip: Arc::clone(&clip),
            };

            let constraints = [
                self.settings.piano_depth.get().constraint(),
                Constraint::Fill(1),
            ];

            let stack = TwoStack::horizontal((key, row), constraints);

            (stack, constraint)
        }))
        .render(area, buffer, mouse_position);
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {
        // TODO: resizing the piano
        // TODO: selecting notes
    }
}
