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
use crate::widget::{Direction, Feed, Ruler, Solid, Text, ToWidget, Widget};
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

impl ToWidget for PianoRoll {
    type Widget<'widget> =
        Result<TwoStack<TwoStack<Solid, Ruler>, Feed<'widget, TwoStack<PianoKey, Row>>>, Text>;

    fn to_widget(&self) -> Self::Widget<'_> {
        let Some(clip) = self.clip.as_ref().map(Arc::clone) else {
            return Err(Text::centred(NO_CLIP_SELECTED));
        };

        let horizontal_constraints = [
            self.settings.piano_depth.get().constraint(),
            Constraint::Fill(1),
        ];
        let vertical_constraints = [Constraint::Length(2), Constraint::Fill(1)];

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

        let roll_start = self.mapping.instant(self.settings.piano_depth.get());
        let piano_key_key = self.key.get(roll_start);

        Ok(TwoStack::vertical(
            (
                ruler,
                Feed::new(Direction::Up, -self.settings.y_offset, move |index| {
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
                }),
            ),
            vertical_constraints,
        ))
    }
}

impl Widget for PianoRoll {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        self.to_widget().render(area, buffer, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        self.to_widget().click(area, button, position, actions);
    }
}
