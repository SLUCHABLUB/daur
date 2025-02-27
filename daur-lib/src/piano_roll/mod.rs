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
use crate::ui::{Mapping, Offset, Point, Rectangle};
use crate::widget::heterogeneous::TwoStack;
use crate::widget::{feed, Direction, Ruler, Solid, Text, Widget};
use crate::Clip;
use arcstr::{literal, ArcStr};
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint;
use saturating_cast::SaturatingCast as _;
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
}

impl Widget for PianoRoll {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let Some(clip) = self.clip.as_ref().map(Arc::clone) else {
            Text::centred(NO_CLIP_SELECTED).render(area, buffer, mouse_position);
            return;
        };

        let horizontal_constraints = [
            self.settings.piano_depth.get().constraint(),
            Constraint::Fill(1),
        ];
        let vertical_constraints = [Constraint::Max(2), Constraint::Fill(1)];

        let ruler = TwoStack::horizontal(
            (
                Solid::EMPTY,
                Ruler {
                    mapping: self.mapping.clone(),
                    offset: Offset::negative(self.settings.x_offset),
                },
            ),
            horizontal_constraints,
        );

        let roll_start = self
            .mapping
            .instant(area.position.x + self.settings.piano_depth.get());
        let piano_key_key = self.key.get(roll_start);

        // TODO: change the anchor point to be at the bottom
        TwoStack::vertical(
            (
                ruler,
                feed(
                    Direction::Up,
                    -self.settings.y_offset,
                    area.size.height,
                    |index| {
                        let interval = Interval::from_semitones(index.saturating_cast());
                        let pitch = Pitch::A440 + interval;

                        let key = PianoKey {
                            key: piano_key_key,
                            pitch,
                            black_key_depth: self.settings.black_key_depth,
                        };
                        let row = Row {
                            clip: Arc::clone(&clip),
                            pitch,
                        };

                        let stack = TwoStack::horizontal((key, row), horizontal_constraints);

                        (stack, self.settings.key_width.get())
                    },
                ),
            ),
            vertical_constraints,
        )
        .render(area, buffer, mouse_position);
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {}
}
