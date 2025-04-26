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
use crate::ui::{Length, Mapping, Offset, Point, Rectangle};
use crate::view::{Direction, Quotated, ToText as _, View, feed, ruler};
use crate::{Changing, Clip, HoldableObject, UserInterface};
use arcstr::{ArcStr, literal};
use saturating_cast::SaturatingCast as _;
use std::sync::Weak;

const PIANO_ROLL: ArcStr = literal!("piano roll");
const NO_CLIP_SELECTED: ArcStr = literal!("please select a clip to edit");

/// Returns the view for the piano roll.
pub fn view<Ui: UserInterface>(
    clip: &Weak<Clip>,
    mapping: Mapping,
    settings: Settings,
    key: &Changing<Key>,
) -> Quotated {
    if !settings.open {
        return Quotated::EMPTY;
    }

    let view = content::<Ui>(clip, mapping, settings, key);

    let title = clip.upgrade().as_deref().map_or(PIANO_ROLL, Clip::name);

    let title_height = Ui::title_height(&title, &view);

    view.titled(title)
        .grabbable(grabber(title_height))
        .quotated(settings.content_height + title_height)
}

fn content<Ui: UserInterface>(
    clip: &Weak<Clip>,
    mapping: Mapping,
    settings: Settings,
    key: &Changing<Key>,
) -> View {
    let Some(_clip) = clip.upgrade() else {
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

    // The piano roll has a fixed lower pitch.
    // Resizing it will thus cause the bottom to be fixed.
    // Since the top is the thing being moved, this seems intuitive.
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
        direction: Direction::Down,
        elements: vec![ruler.quotated_minimally::<Ui>(), workspace.fill_remaining()],
    }
}

fn grabber(
    title_height: Length,
) -> impl Fn(Rectangle, Point) -> Option<HoldableObject> + Send + Sync + 'static {
    move |area, position| {
        let relative_position = position - area.position.position();

        if relative_position.y < title_height {
            Some(HoldableObject::PianoRollHandle {
                y: relative_position.y,
            })
        } else {
            // TODO: grab contents, i.e. notes
            None
        }
    }
}
