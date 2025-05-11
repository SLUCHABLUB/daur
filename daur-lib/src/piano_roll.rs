use crate::audio::Player;
use crate::metre::Instant;
use crate::notes::{Interval, Key, Pitch};
use crate::project::Settings;
use crate::ui::{Colour, Grid, Length, NonZeroLength, Offset, Point, Rectangle};
use crate::view::{Alignment, CursorWindow, Quotated, ToText as _, ruler};
use crate::{Action, Clip, HoldableObject, UserInterface, View};
use alloc::sync::{Arc, Weak};
use arcstr::{ArcStr, literal};
use core::cmp::Ordering;
use saturating_cast::SaturatingCast as _;

const PIANO_ROLL: ArcStr = literal!("piano roll");
const NO_CLIP_SELECTED: ArcStr = literal!("please select a clip to edit");

/// The pitch at the bottom of the piano roll (before scrolling).
/// Due to the way we calculate the lowest key's width (using modulo),
/// this is one semitone lower than the actual bottom pitch.
const BOTTOM: Pitch = Pitch::a_440_plus(-1);

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

#[derive(Copy, Clone)]
struct Keys {
    lowest_key_width: Length,
    lowest_key_pitch: Pitch,
    highest_key_pitch: Pitch,
    #[expect(clippy::struct_field_names, reason = "this reads better")]
    number_of_full_keys: u64,
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
            .grabbable(Self::grabber(title_height))
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
        let Some(clip) = clip.upgrade() else {
            return NO_CLIP_SELECTED.centred();
        };

        // The piano roll has a fixed lower pitch.
        // Resizing it will thus cause the bottom to be fixed.
        // Since the top is the thing being moved, this seems intuitive.
        let workspace = self.workspace::<Ui>(clip, &project, grid, player, cursor);

        let ruler = View::x_stack([
            View::Empty.quotated(self.piano_depth.get()),
            ruler::<Ui>(self.negative_x_offset, project, grid).fill_remaining(),
        ]);

        View::y_stack([
            ruler.quotated(Ui::RULER_HEIGHT.get()),
            workspace.fill_remaining(),
        ])
    }

    fn workspace<Ui: UserInterface>(
        self,
        _clip: Arc<Clip>,
        project: &Settings,
        grid: Grid,
        player: Option<Player>,
        cursor: Instant,
    ) -> View {
        let keys = self.keys::<Ui>();

        let piano = self.piano(keys, project, grid);
        let roll = self.roll(keys, project, grid);
        let cursor_window = CursorWindow::view(
            player,
            cursor,
            project.clone(),
            grid,
            self.negative_x_offset,
        );

        View::x_stack([
            piano.quotated(self.piano_depth.get()),
            View::Layers(vec![roll, cursor_window]).fill_remaining(),
        ])
    }

    fn piano(self, keys: Keys, project: &Settings, grid: Grid) -> View {
        let roll_start = Instant::from_x_offset(self.negative_x_offset, project, grid);
        let key = project.key.get(roll_start);

        let highest_key = self.piano_key(keys.highest_key_pitch, key).fill_remaining();
        let lowest_key = self
            .piano_key(keys.lowest_key_pitch, key)
            .quotated(keys.lowest_key_width);

        let mut piano = vec![highest_key];

        for semitones in (1..=keys.number_of_full_keys).rev() {
            let interval = Interval::from_semitones(semitones.saturating_cast());
            let pitch = keys.lowest_key_pitch + interval;
            let key = self.piano_key(pitch, key).quotated(self.key_width.get());
            piano.push(key);
        }

        piano.push(lowest_key);

        View::y_stack(piano)
    }

    fn roll(self, keys: Keys, project: &Settings, grid: Grid) -> View {
        let highest_row = Self::row(keys.highest_key_pitch, project, grid).fill_remaining();
        let lowest_row =
            Self::row(keys.lowest_key_pitch, project, grid).quotated(keys.lowest_key_width);

        let mut rows = vec![highest_row];

        for semitones in (1..=keys.number_of_full_keys).rev() {
            let interval = Interval::from_semitones(semitones.saturating_cast());
            let pitch = keys.lowest_key_pitch + interval;
            let row = Self::row(pitch, project, grid).quotated(self.key_width.get());
            rows.push(row);
        }

        rows.push(lowest_row);

        View::y_stack(rows)
    }

    /// Return the view for a (non-piano) row of the piano roll.
    fn row(pitch: Pitch, _project: &Settings, _grid: Grid) -> View {
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
        View::Solid(colour)
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

    fn keys<Ui: UserInterface>(self) -> Keys {
        let lowest_key_width = self.y_offset % self.key_width;
        // = floor(y_offset / key_width)
        let lowest_key_semitones = match self.y_offset.cmp(&Offset::ZERO) {
            Ordering::Less => (self.y_offset.abs() / self.key_width)
                .ceil()
                .saturating_cast::<i16>()
                .saturating_neg(),
            Ordering::Equal => 0,
            Ordering::Greater => (self.y_offset.rectify() / self.key_width)
                .floor()
                .saturating_cast(),
        };
        let lowest_key_pitch = BOTTOM + Interval::from_semitones(lowest_key_semitones);

        let workspace_height = self.content_height - Ui::RULER_HEIGHT.get();
        let remaining_key_space = workspace_height - lowest_key_width;

        let number_of_full_keys = (remaining_key_space / self.key_width).floor();

        let visible_interval =
            Interval::from_semitones(number_of_full_keys.saturating_add(1).saturating_cast());
        let highest_key_pitch = lowest_key_pitch + visible_interval;

        Keys {
            lowest_key_width,
            lowest_key_pitch,
            highest_key_pitch,
            number_of_full_keys,
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
}
