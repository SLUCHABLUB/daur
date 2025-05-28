use crate::app::Action;
use crate::audio::Player;
use crate::metre::{Instant, NonZeroDuration};
use crate::note::{Group, Interval, Key, Note, Pitch};
use crate::project::track;
use crate::project::track::{Clip, clip};
use crate::ui::{Colour, Grid, Length, NonZeroLength, Offset, Size, relative};
use crate::view::{Alignment, CursorWindow, Quotated, RenderArea, ToText as _, ruler};
use crate::{HoldableObject, Project, Selection, UserInterface, View, project};
use arcstr::{ArcStr, literal};
use closure::closure;
use itertools::chain;
use saturating_cast::SaturatingCast as _;
use std::cmp::{Ordering, max, min};
use std::iter::once;

const PIANO_ROLL: ArcStr = literal!("piano roll");
const NO_CLIP_SELECTED: ArcStr = literal!("please select a clip to edit");
// TODO: add audio clip "editing"
const AUDIO_CLIP_SELECTED: ArcStr = literal!("cannot edit audio clips (yet)");

/// The pitch at the bottom of the piano roll (before scrolling).
/// Due to the way we calculate the lowest key's width (using modulo),
/// this is one semitone lower than the actual bottom pitch.
const BOTTOM: Pitch = Pitch::a_440_plus(-1);

/// Settings for the piano roll.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PianoRoll {
    /// How far to the left the piano roll is moved.
    pub negative_x_offset: Length,
    /// How far down the piano roll is moved.
    pub y_offset: Offset,
    /// The height of the piano roll content (excluding the title).
    pub content_height: Length,
    /// Whether the piano roll is open.
    pub is_open: bool,

    /// The width of piano key.
    pub key_width: NonZeroLength,
    /// The full depth of a white keys.
    pub piano_depth: NonZeroLength,
    /// The depth of a black keys.
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
    // TODO: merge some settings into "temporary settings"
    /// Returns the view for the piano roll.
    #[expect(clippy::too_many_arguments, reason = "see TODO")]
    pub(crate) fn view<Ui: UserInterface>(
        self,
        selection: Selection,
        project: &Project,
        grid: Grid,
        player: Option<Player>,
        cursor: Instant,
        held_object: Option<HoldableObject>,
        edit_mode: bool,
    ) -> Quotated {
        if !self.is_open {
            return Quotated::EMPTY;
        }

        let (clip_start, clip) = project
            .track(selection.track())
            .and_then(|track| track.clip(selection.clip()))
            .map_or((Instant::START, None), |(start, clip)| (start, Some(clip)));

        let title = clip.map_or(PIANO_ROLL, Clip::name);

        let view = self.content::<Ui>(
            clip,
            clip_start,
            project,
            grid,
            player,
            cursor,
            held_object,
            edit_mode,
        );

        let title_height = Ui::title_height(&title, &view);

        view.scrollable(Action::MovePianoRoll)
            .titled(title)
            .grabbable(Self::handle_grabber(title_height))
            .quotated(self.content_height + title_height)
    }

    #[expect(clippy::too_many_arguments, reason = "the method is private")]
    fn content<Ui: UserInterface>(
        self,
        clip: Option<&Clip>,
        clip_start: Instant,
        project: &Project,
        grid: Grid,
        player: Option<Player>,
        cursor: Instant,
        held_object: Option<HoldableObject>,
        edit_mode: bool,
    ) -> View {
        let Some(clip) = clip else {
            return NO_CLIP_SELECTED.centred();
        };

        let Some(notes) = clip.content().as_notes() else {
            return AUDIO_CLIP_SELECTED.centred();
        };

        // The piano roll has a fixed lower pitch.
        // Resizing it will thus cause the bottom to be fixed.
        // Since the top is the thing being moved, this seems intuitive.
        let workspace = self.workspace::<Ui>(
            clip_start,
            notes,
            clip.colour(),
            project.settings(),
            grid,
            player,
            cursor,
            held_object,
            edit_mode,
        );

        let ruler = ruler::<Ui>(self.negative_x_offset, project.settings().clone(), grid)
            .fill_remaining()
            .x_positioned(self.piano_depth.get());

        View::y_stack([
            ruler.quotated(Ui::RULER_HEIGHT.get()),
            workspace.fill_remaining(),
        ])
    }

    #[expect(clippy::too_many_arguments, reason = "the method is internal")]
    fn workspace<Ui: UserInterface>(
        self,
        clip_start: Instant,
        notes: &Group,
        clip_colour: Colour,
        project_settings: &project::Settings,
        grid: Grid,
        player: Option<Player>,
        cursor: Instant,
        held_object: Option<HoldableObject>,
        edit_mode: bool,
    ) -> View {
        let keys = self.keys::<Ui>();

        let piano = self.piano(keys, project_settings, grid);
        let roll = self.roll(
            clip_start,
            notes,
            clip_colour,
            keys,
            project_settings,
            grid,
            edit_mode,
        );
        let cursor_window = CursorWindow::builder()
            .cursor(cursor)
            .grid(grid)
            .player(player)
            .project_settings(project_settings.clone())
            .window_offset(self.negative_x_offset)
            .build()
            .view();
        let held_object = self.held_object(held_object, clip_colour, project_settings, grid);

        View::x_stack([
            piano.quotated(self.piano_depth.get()),
            View::Layers(vec![roll, cursor_window, held_object]).fill_remaining(),
        ])
    }

    fn piano(self, keys: Keys, project_settings: &project::Settings, grid: Grid) -> View {
        let roll_start = Instant::from_x_offset(self.negative_x_offset, project_settings, grid);
        let key = project_settings.key.get(roll_start);

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

    #[expect(clippy::too_many_arguments, reason = "the method is private")]
    fn roll(
        self,
        clip_start: Instant,
        notes: &Group,
        clip_colour: Colour,
        keys: Keys,
        project_settings: &project::Settings,
        grid: Grid,
        edit_mode: bool,
    ) -> View {
        let highest_row = self
            .row(
                clip_start,
                notes,
                clip_colour,
                keys.highest_key_pitch,
                project_settings.clone(),
                grid,
                edit_mode,
            )
            .fill_remaining();
        let lowest_row = self
            .row(
                clip_start,
                notes,
                clip_colour,
                keys.lowest_key_pitch,
                project_settings.clone(),
                grid,
                edit_mode,
            )
            .quotated(keys.lowest_key_width);

        let mut rows = vec![highest_row];

        for semitones in (1..=keys.number_of_full_keys).rev() {
            let interval = Interval::from_semitones(semitones.saturating_cast());
            let pitch = keys.lowest_key_pitch + interval;
            let row = self
                .row(
                    clip_start,
                    notes,
                    clip_colour,
                    pitch,
                    project_settings.clone(),
                    grid,
                    edit_mode,
                )
                .quotated(self.key_width.get());
            rows.push(row);
        }

        rows.push(lowest_row);

        View::y_stack(rows)
    }

    /// Return the view for a (non-piano) row of the piano roll.
    #[expect(clippy::too_many_arguments, reason = "the method is private")]
    fn row(
        self,
        clip_start: Instant,
        notes: &Group,
        clip_colour: Colour,
        pitch: Pitch,
        project_settings: project::Settings,
        grid: Grid,
        edit_mode: bool,
    ) -> View {
        // TODO:
        //  - colour for out-of-bounds
        //  - draw grid
        //  - highlight key based on settings
        let background_colour = if (pitch - Pitch::A_440).semitones() % 2 == 0 {
            Colour::gray_scale(0xAA)
        } else {
            Colour::gray_scale(0x55)
        };

        let background = View::Solid(background_colour);

        let view = View::Layers(
            chain(
                once(background),
                notes.with_pitch(pitch).map(|(note_start, note)| {
                    let start = (clip_start + note_start.since_start)
                        .to_x_offset(&project_settings, grid)
                        - self.negative_x_offset;
                    let end = (clip_start + note_start.since_start + note.duration.get())
                        .to_x_offset(&project_settings, grid)
                        - self.negative_x_offset;

                    let width = end - start;

                    Self::note(clip_colour).quotated(width).x_positioned(start)
                }),
            )
            .collect(),
        );

        let grabber = closure!([clone project_settings] move |render_area: RenderArea| {
            let mouse_position = render_area.relative_mouse_position()?;

            Some(if edit_mode {
                let start = Instant::quantised_from_x_offset(
                    mouse_position.x + self.negative_x_offset,
                    &project_settings,
                    grid,
                );

               HoldableObject::NoteCreation { start }
            } else {
                HoldableObject::SelectionBox {
                    start: render_area.mouse_position,
                }
            })
        });

        let dropper = move |object, render_area: RenderArea| {
            let HoldableObject::NoteCreation { start } = object else {
                return None;
            };
            let end = Instant::quantised_from_x_offset(
                render_area.relative_mouse_position()?.x + self.negative_x_offset,
                &project_settings,
                grid,
            );

            let (start, end) = (min(start, end), max(start, end));

            let duration = NonZeroDuration::from_duration(end - start)?;

            Some(Action::Project(project::Action::Track(
                track::Action::Clip(clip::Action::AddNote {
                    position: start,
                    pitch,
                    note: Note { duration },
                }),
            )))
        };

        view.grabbable(grabber).object_accepting(dropper)
    }

    fn note(colour: Colour) -> View {
        View::Solid(colour)
    }

    // TODO: use `Button` for:
    //  - plinking the key
    //  - selecting all notes with the key's pitch
    /// Return the view for a key on the piano-roll piano.
    fn piano_key(self, pitch: Pitch, key: Key) -> View {
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

    fn handle_grabber(
        title_height: Length,
    ) -> impl Fn(RenderArea) -> Option<HoldableObject> + Send + Sync + 'static {
        move |render_area| {
            let y = render_area.relative_mouse_position()?.y;

            (y < title_height).then_some(HoldableObject::PianoRollHandle { y })
        }
    }

    fn held_object(
        self,
        held_object: Option<HoldableObject>,
        clip_colour: Colour,
        project_settings: &project::Settings,
        grid: Grid,
    ) -> View {
        let Some(held_object) = held_object else {
            return View::Empty;
        };

        let start = match held_object {
            HoldableObject::NoteCreation { start } => start,
            // selection boxes are drawn on the app level
            HoldableObject::PianoRollHandle { .. } | HoldableObject::SelectionBox { .. } => {
                return View::Empty;
            }
        };

        let start = start.to_x_offset(project_settings, grid) - self.negative_x_offset;

        View::reactive(move |ui_info| {
            // TODO: quantise
            let end = ui_info.mouse_position.x - ui_info.area.position.x;

            let y_offset = ui_info.mouse_position.y - ui_info.area.position.y;

            let (start, end) = (min(start, end), max(start, end));
            let width = end - start;

            let size = Size {
                width,
                height: self.key_width.get(),
            };

            let position = relative::Point {
                x: start,
                y: y_offset,
            };

            Self::note(clip_colour)
                .quotated_2d(size)
                .positioned(position)
        })
    }
}
