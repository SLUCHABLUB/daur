use crate::audio::Player;
use crate::metre::Instant;
use crate::notes::{Interval, Key, Pitch};
use crate::project::Settings;
use crate::ui::{Colour, Direction, Grid, Length, NonZeroLength, Offset, Point, Rectangle};
use crate::view::{Alignment, CursorWindow, Quotated, ToText as _, feed, ruler};
use crate::{Action, Clip, HoldableObject, UserInterface, View};
use alloc::sync::Weak;
use arcstr::{ArcStr, literal};
use saturating_cast::SaturatingCast as _;

const PIANO_ROLL: ArcStr = literal!("piano roll");
const NO_CLIP_SELECTED: ArcStr = literal!("please select a clip to edit");

/// Settings for the piano roll.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PianoRoll {
    /// How far to the left the piano roll is moved.
    pub negative_x_offset: Length,
    /// How far from A4 the piano roll is scrolled.
    pub y_offset: Offset,
    /// The height of the piano roll content (excluding the title).
    pub content_height: Length,
    /// Whether the piano roll is open.
    pub is_open: bool,

    /// The width of the keys
    pub key_width: NonZeroLength,
    /// The full depth of the white keys
    pub piano_depth: NonZeroLength,
    /// The depth of the black keys
    pub black_key_depth: NonZeroLength,
}

impl PianoRoll {
    /// Returns the view for the piano roll.
    pub(crate) fn view<Ui: UserInterface>(
        self,
        clip: &Weak<Clip>,
        project: Settings,
        grid: Grid,
        player: Option<Player>,
        cursor: Instant,
    ) -> Quotated {
        if !self.is_open {
            return Quotated::EMPTY;
        }

        let view = self.content::<Ui>(clip, project, grid, player, cursor);

        let title = clip.upgrade().as_deref().map_or(PIANO_ROLL, Clip::name);

        let title_height = Ui::title_height(&title, &view);

        view.scrollable(Action::MovePianoRoll)
            .titled(title)
            .grabbable(grabber(title_height))
            .quotated(self.content_height + title_height)
    }

    fn content<Ui: UserInterface>(
        self,
        clip: &Weak<Clip>,
        project: Settings,
        grid: Grid,
        player: Option<Player>,
        cursor: Instant,
    ) -> View {
        let Some(_clip) = clip.upgrade() else {
            return NO_CLIP_SELECTED.centred();
        };

        let roll_start = Instant::from_x_offset(self.negative_x_offset, &project, grid);
        let piano_key_key = project.key.get(roll_start);

        let ruler = View::x_stack([
            View::Empty.quotated(self.piano_depth.get()),
            ruler::<Ui>(self.negative_x_offset, project.clone(), grid).fill_remaining(),
        ]);

        // The piano roll has a fixed lower pitch.
        // Resizing it will thus cause the bottom to be fixed.
        // Since the top is the thing being moved, this seems intuitive.
        let workspace = feed::<Ui, _>(Direction::Up, -self.y_offset, move |index| {
            let interval = Interval::from_semitones(index.saturating_cast());
            let pitch = Pitch::A_440 + interval;

            let key = self.piano_key(pitch, piano_key_key);
            let row = self.row(pitch, &project, grid, player.clone(), cursor);

            let stack = View::x_stack([key.quotated(self.piano_depth.get()), row.fill_remaining()]);

            stack.quotated(self.key_width.get())
        });

        View::y_stack([
            ruler.quotated(Ui::RULER_HEIGHT.get()),
            workspace.fill_remaining(),
        ])
    }

    /// Return the view for a (non-piano) row of the piano roll.
    fn row(
        self,
        pitch: Pitch,
        project: &Settings,
        grid: Grid,
        player: Option<Player>,
        cursor: Instant,
    ) -> View {
        // TODO:
        //  - draw notes
        //  - draw grid
        //  - highlight key based on settings
        let colour = if (pitch - Pitch::A_440).semitones() % 2 == 0 {
            Colour::gray_scale(0xAA)
        } else {
            Colour::gray_scale(0x55)
        };

        // TODO: use `Grabbable` for
        //  - adding notes
        //  - selecting notes
        //  - moving the cursor
        // TODO: move the cursor window up (so there is only one cursor window)
        View::Layers(vec![
            View::Solid(colour),
            CursorWindow::view(
                player,
                cursor,
                project.clone(),
                grid,
                self.negative_x_offset,
            ),
        ])
    }

    // TODO: use `Button` for:
    //  - plinking the key
    //  - selecting all notes with the key's pitch
    /// Return the view for a key on the piano-roll piano.
    fn piano_key(&self, pitch: Pitch, key: Key) -> View {
        let top = View::Solid(if pitch.chroma().is_black_key() {
            Colour::BLACK
        } else {
            Colour::WHITE
        });

        let text = if pitch.chroma() == key.tonic {
            ArcStr::from(pitch.name(key.sign))
        } else {
            ArcStr::new()
        }
        .aligned_to(Alignment::BottomRight);

        let bottom = View::Layers(vec![View::Solid(Colour::WHITE), text]);

        View::x_stack([
            top.quotated(self.black_key_depth.get()),
            bottom.fill_remaining(),
        ])
    }
}

fn grabber(
    title_height: Length,
) -> impl Fn(Rectangle, Point) -> Option<HoldableObject> + Send + Sync + 'static {
    move |area, position| {
        let relative_position = position - area.position.position();

        (relative_position.y < title_height).then_some(HoldableObject::PianoRollHandle {
            y: relative_position.y,
        })
    }
}
