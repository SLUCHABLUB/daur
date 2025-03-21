//! Items pertaining to the piano roll.

mod key;
mod row;
mod settings;

pub use key::piano_key;
pub use row::row;
pub use settings::Settings;

use crate::interval::Interval;
use crate::key::Key;
use crate::pitch::Pitch;
use crate::ui::{Mapping, Offset};
use crate::view::{feed, ruler, Direction, ToText as _, View};
use crate::{Changing, Clip};
use arcstr::{literal, ArcStr};
use saturating_cast::SaturatingCast as _;

const NO_CLIP_SELECTED: ArcStr = literal!("please select a clip to edit");

// The piano roll has a fixed lower pitch.
// Resizing it will thus cause the bottom to be fixed.
// Since the top is the thing you move this seems intuitive.
/// Return the view for the piano roll.
pub fn view(
    clip: Option<&Clip>,
    mapping: Mapping,
    settings: Settings,
    key: &Changing<Key>,
) -> View {
    let Some(_clip) = clip else {
        return NO_CLIP_SELECTED.centred();
    };

    let roll_start = mapping.instant(settings.piano_depth.get());
    let piano_key_key = key.get(roll_start);

    let ruler = View::Stack {
        direction: Direction::Right,
        elements: vec![
            View::Empty.quotated(settings.piano_depth.get()),
            ruler(mapping, Offset::negative(settings.x_offset)).fill_remaining(),
        ],
    };

    let workspace = feed(Direction::Up, -settings.y_offset, move |index| {
        let interval = Interval::from_semitones(index.saturating_cast());
        let pitch = Pitch::A440 + interval;

        let key = piano_key(pitch, piano_key_key, settings.black_key_depth);
        let row = row(pitch);

        let stack = View::Stack {
            direction: Direction::Right,
            elements: vec![
                key.quotated(settings.piano_depth.get()),
                row.fill_remaining(),
            ],
        };

        stack.quotated(settings.key_width.get())
    });

    View::Stack {
        direction: Direction::Right,
        elements: vec![ruler.quotated_minimally(), workspace.fill_remaining()],
    }
}
