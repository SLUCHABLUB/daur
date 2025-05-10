//! Items pertaining to the piano roll.

mod key;
mod row;
mod settings;

pub use settings::Settings;

use crate::audio::Player;
use crate::metre::Instant;
use crate::notes::{Interval, Pitch};
use crate::ui::{Direction, Grid, Length, Point, Rectangle};
use crate::view::{Quotated, ToText as _, View, feed, ruler};
use crate::{Action, Clip, HoldableObject, UserInterface, project};
use alloc::sync::Weak;
use arcstr::{ArcStr, literal};
use key::piano_key;
use row::row;
use saturating_cast::SaturatingCast as _;

const PIANO_ROLL: ArcStr = literal!("piano roll");
const NO_CLIP_SELECTED: ArcStr = literal!("please select a clip to edit");

/// Returns the view for the piano roll.
pub(crate) fn view<Ui: UserInterface>(
    clip: &Weak<Clip>,
    settings: Settings,
    project_settings: project::Settings,
    grid: Grid,
    player: Option<Player>,
    cursor: Instant,
) -> Quotated {
    if !settings.open {
        return Quotated::EMPTY;
    }

    let view = content::<Ui>(clip, settings, project_settings, grid, player, cursor);

    let title = clip.upgrade().as_deref().map_or(PIANO_ROLL, Clip::name);

    let title_height = Ui::title_height(&title, &view);

    view.scrollable(Action::MovePianoRoll)
        .titled(title)
        .grabbable(grabber(title_height))
        .quotated(settings.content_height + title_height)
}

fn content<Ui: UserInterface>(
    clip: &Weak<Clip>,
    settings: Settings,
    project_settings: project::Settings,
    grid: Grid,
    player: Option<Player>,
    cursor: Instant,
) -> View {
    let Some(_clip) = clip.upgrade() else {
        return NO_CLIP_SELECTED.centred();
    };

    let roll_start = Instant::from_x_offset(settings.x_offset, &project_settings, grid);
    let piano_key_key = project_settings.key.get(roll_start);

    let ruler = View::x_stack([
        View::Empty.quotated(settings.piano_depth.get()),
        ruler::<Ui>(settings.x_offset, project_settings.clone(), grid).fill_remaining(),
    ]);

    // The piano roll has a fixed lower pitch.
    // Resizing it will thus cause the bottom to be fixed.
    // Since the top is the thing being moved, this seems intuitive.
    let workspace = feed::<Ui, _>(Direction::Up, -settings.y_offset, move |index| {
        let interval = Interval::from_semitones(index.saturating_cast());
        let pitch = Pitch::A_440 + interval;

        let key = piano_key(pitch, piano_key_key, settings.black_key_depth);
        let row = row(
            pitch,
            &settings,
            project_settings.clone(),
            grid,
            player.clone(),
            cursor,
        );

        let stack = View::x_stack([
            key.quotated(settings.piano_depth.get()),
            row.fill_remaining(),
        ]);

        stack.quotated(settings.key_width.get())
    });

    View::y_stack([
        ruler.quotated(Ui::RULER_HEIGHT.get()),
        workspace.fill_remaining(),
    ])
}

fn grabber(
    title_height: Length,
) -> impl Fn(Rectangle, Point) -> Option<HoldableObject> + Send + Sync + 'static {
    move |area, position| {
        let relative_position = position - area.position.position();

        #[expect(clippy::if_then_some_else_none, reason = "see todo")]
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
