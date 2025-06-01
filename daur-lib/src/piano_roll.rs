use crate::app::Action;
use crate::audio::Player;
use crate::metre::{Changing, Instant, NonZeroDuration, OffsetMapping, Quantisation, TimeContext};
use crate::note::{Group, Interval, Key, Pitch};
use crate::project::track;
use crate::project::track::{Clip, clip};
use crate::ui::{Colour, Length, NonZeroLength, Size, ThemeColour, Vector, relative};
use crate::view::{Alignment, CursorWindow, Quotated, RenderArea, ToText as _, ruler};
use crate::{HoldableObject, Project, Ratio, Selection, UserInterface, View, project};
use arcstr::{ArcStr, literal};
use closure::closure;
use getset::{CopyGetters, Setters};
use itertools::chain;
use saturating_cast::SaturatingCast as _;
use std::cmp::{max, min};
use std::iter::once;

const PIANO_ROLL: ArcStr = literal!("piano roll");
const NO_CLIP_SELECTED: ArcStr = literal!("please select a clip to edit");
// TODO: add audio clip "editing"
const AUDIO_CLIP_SELECTED: ArcStr = literal!("cannot edit audio clips (yet)");

/// Settings for the piano roll.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Setters, CopyGetters)]
pub struct PianoRoll {
    /// How far to the left the piano roll is moved.
    negative_x_offset: Length,
    /// How far down the piano roll is moved.
    y_offset: Length,
    /// The height of the piano roll content (excluding the title).
    #[set = "pub(crate)"]
    content_height: Length,
    /// Whether the piano roll is open.
    #[set = "pub(crate)"]
    #[get_copy = "pub(crate)"]
    is_open: bool,

    /// The width of piano key.
    key_width: NonZeroLength,
    /// The full depth of a white keys.
    piano_depth: NonZeroLength,
    /// The depth of a black keys.
    black_key_depth: NonZeroLength,
}

impl PianoRoll {
    pub(crate) fn new_in<Ui: UserInterface>() -> PianoRoll {
        let a3_offset = Ui::KEY_WIDTH.get() * Ratio::integer(57);
        let three_octaves = Ui::KEY_WIDTH.get() * Ratio::integer(3 * 12) + Ui::RULER_HEIGHT.get();

        PianoRoll {
            negative_x_offset: Length::ZERO,
            y_offset: a3_offset,
            content_height: three_octaves,
            is_open: false,
            key_width: Ui::KEY_WIDTH,
            piano_depth: Ui::PIANO_DEPTH,
            black_key_depth: Ui::BLACK_KEY_DEPTH,
        }
    }

    pub(crate) fn y_offset<Ui: UserInterface>(self) -> Length {
        let full_roll_heigh = self.key_width.get() * Ratio::integer(128);
        let workspace_height = self.content_height - Ui::RULER_HEIGHT.get();

        min(self.y_offset, full_roll_heigh - workspace_height)
    }

    pub(crate) fn move_by<Ui: UserInterface>(&mut self, by: Vector) {
        self.negative_x_offset -= by.x;
        self.y_offset += by.y;

        self.y_offset = self.y_offset::<Ui>();
    }

    // TODO: merge some settings into "temporary settings"
    /// Returns the view for the piano roll.
    #[expect(clippy::too_many_arguments, reason = "see TODO")]
    pub(crate) fn view<Ui: UserInterface>(
        self,
        selection: &Selection,
        project: &Project,
        quantisation: Quantisation,
        player: Option<Player>,
        cursor: Instant,
        held_object: Option<HoldableObject>,
        edit_mode: bool,
    ) -> Quotated {
        if !self.is_open {
            return Quotated::EMPTY;
        }

        let (clip_start, clip) = project
            .track(selection.track)
            .and_then(|track| track.clip(*selection.clips.last()?))
            .map_or((Instant::START, None), |(start, clip)| (start, Some(clip)));

        let title = clip.map_or(PIANO_ROLL, Clip::name);

        let view = self.content::<Ui>(
            clip,
            clip_start,
            project,
            quantisation,
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
        quantisation: Quantisation,
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

        let offset_mapping = OffsetMapping::new(project.time_signature().clone(), quantisation);

        // The piano roll has a fixed lower pitch.
        // Resizing it will thus cause the bottom to be fixed.
        // Since the top is the thing being moved, this seems intuitive.
        let workspace = self.workspace::<Ui>(
            clip_start,
            notes,
            clip.colour(),
            offset_mapping.clone(),
            project.time_context(),
            player,
            cursor,
            held_object,
            edit_mode,
            project.key(),
        );

        let ruler = ruler::<Ui>(self.negative_x_offset, offset_mapping)
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
        offset_mapping: OffsetMapping,
        time_context: Changing<TimeContext>,
        player: Option<Player>,
        cursor: Instant,
        held_object: Option<HoldableObject>,
        edit_mode: bool,
        key: &Changing<Key>,
    ) -> View {
        let roll = self.roll::<Ui>(
            clip_start,
            notes,
            clip_colour,
            &offset_mapping,
            edit_mode,
            key,
        );

        let held_object = self.held_object(held_object, clip_colour, &offset_mapping);
        let cursor_window = CursorWindow::builder()
            .cursor(cursor)
            .offset_mapping(offset_mapping)
            .player(player)
            .time_context(time_context)
            .window_offset(self.negative_x_offset)
            .build()
            .view();

        let overlay =
            View::Layers(vec![cursor_window, held_object]).x_positioned(self.piano_depth.get());

        View::Layers(vec![roll, overlay])
    }

    fn roll<Ui: UserInterface>(
        self,
        clip_start: Instant,
        notes: &Group,
        clip_colour: Colour,
        offset_mapping: &OffsetMapping,
        edit_mode: bool,
        key: &Changing<Key>,
    ) -> View {
        let y_offset = self.y_offset::<Ui>();

        let lowest_visible_pitch = Pitch::LOWEST
            + Interval::from_semitones((y_offset / self.key_width).floor().saturating_cast());

        let lowest_row_height = y_offset % self.key_width;
        let lowest_row_height = if lowest_row_height == Length::ZERO {
            self.key_width.get()
        } else {
            lowest_row_height
        };

        let remaining_space = self.content_height - Ui::RULER_HEIGHT.get() - lowest_row_height;

        let number_of_full_keys = (remaining_space / self.key_width).floor();

        let highest_visible_pitch = lowest_visible_pitch
            + Interval::from_semitones(number_of_full_keys.saturating_cast())
            + Interval::SEMITONE;

        let lowest_row = self
            .row(
                clip_start,
                notes,
                clip_colour,
                lowest_visible_pitch,
                offset_mapping.clone(),
                edit_mode,
                key,
            )
            .quotated(lowest_row_height);

        let highest_row = self
            .row(
                clip_start,
                notes,
                clip_colour,
                highest_visible_pitch,
                offset_mapping.clone(),
                edit_mode,
                key,
            )
            .fill_remaining();

        let mut rows = vec![highest_row];

        for semitones in (1..=number_of_full_keys).rev() {
            let interval = Interval::from_semitones(semitones.saturating_cast());
            let pitch = lowest_visible_pitch + interval;

            rows.push(
                self.row(
                    clip_start,
                    notes,
                    clip_colour,
                    pitch,
                    offset_mapping.clone(),
                    edit_mode,
                    key,
                )
                .quotated(self.key_width.get()),
            );
        }

        rows.push(lowest_row);

        View::y_stack(rows)
    }

    /// Return the view for a row in the piano roll.
    #[expect(clippy::too_many_arguments, reason = "the method is private")]
    fn row(
        self,
        clip_start: Instant,
        notes: &Group,
        clip_colour: Colour,
        pitch: Pitch,
        offset_mapping: OffsetMapping,
        edit_mode: bool,
        key: &Changing<Key>,
    ) -> View {
        let key = key.get(offset_mapping.instant(self.negative_x_offset));

        // TODO:
        //  - colour for out-of-bounds
        //  - draw grid
        //  - highlight key based on settings
        // TODO: use a theme colour
        let background_colour = if (pitch - Pitch::LOWEST).semitones() % 2 == 0 {
            ThemeColour::Custom(Colour::gray_scale(0xAA))
        } else {
            ThemeColour::Custom(Colour::gray_scale(0x55))
        };

        let background = View::Solid(background_colour);

        let view = View::Layers(
            chain(
                once(background),
                notes.with_pitch(pitch).map(|(note_start, note)| {
                    let start = offset_mapping.offset(clip_start + note_start.since_start)
                        - self.negative_x_offset;
                    let end = offset_mapping
                        .offset(clip_start + note_start.since_start + note.duration().get())
                        - self.negative_x_offset;

                    let width = end - start;

                    Self::note(clip_colour).quotated(width).x_positioned(start)
                }),
            )
            .collect(),
        );

        let grabber = closure!([clone offset_mapping] move |render_area: RenderArea| {
            let mouse_position = render_area.relative_mouse_position()?;

            Some(if edit_mode {
                let start = offset_mapping.quantised_instant(mouse_position.x + self.negative_x_offset);

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
            let end = offset_mapping.quantised_instant(
                render_area.relative_mouse_position()?.x + self.negative_x_offset,
            );

            let (start, end) = (min(start, end), max(start, end));

            let duration = NonZeroDuration::from_duration(end - start)?;

            Some(Action::Project(project::Action::Track(
                track::Action::Clip(clip::Action::AddNote {
                    position: start,
                    pitch,
                    duration,
                }),
            )))
        };

        let overview = view.grabbable(grabber).object_accepting(dropper);

        View::x_stack([
            self.piano_key(pitch, key).quotated(self.piano_depth.get()),
            overview.fill_remaining(),
        ])
    }

    fn note(colour: Colour) -> View {
        View::Solid(ThemeColour::Custom(colour))
    }

    // TODO: use `Button` for:
    //  - plinking the key
    //  - selecting all notes with the key's pitch
    /// Return the view for a key on the piano-roll piano.
    fn piano_key(self, pitch: Pitch, key: Key) -> View {
        let top = View::Solid(if pitch.chroma().is_black_key() {
            ThemeColour::BlackKey
        } else {
            ThemeColour::WhiteKey
        });

        let text = if pitch.chroma() == key.tonic {
            ArcStr::from(pitch.name(key.sign))
        } else {
            ArcStr::new()
        }
        .aligned_to(Alignment::BottomRight);

        let bottom = View::Layers(vec![View::Solid(ThemeColour::WhiteKey), text]);

        View::x_stack([
            top.quotated(self.black_key_depth.get()),
            bottom.fill_remaining(),
        ])
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
        offset_mapping: &OffsetMapping,
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

        let start = offset_mapping.offset(start) - self.negative_x_offset;

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
