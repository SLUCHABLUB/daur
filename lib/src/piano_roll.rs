//! Items pertaining to the [piano roll](PianoRoll).

use crate::Holdable;
use crate::Id;
use crate::Project;
use crate::Ratio;
use crate::Selectable;
use crate::UserInterface;
use crate::View;
use crate::app::Action;
use crate::audio::Player;
use crate::metre::Changing;
use crate::metre::Instant;
use crate::metre::NonZeroDuration;
use crate::metre::OffsetMapping;
use crate::metre::Quantisation;
use crate::metre::TimeContext;
use crate::note;
use crate::note::Group;
use crate::note::Interval;
use crate::note::Key;
use crate::note::Pitch;
use crate::project::Edit;
use crate::project::Track;
use crate::project::track::Clip;
use crate::select::Selection;
use crate::ui::Colour;
use crate::ui::Length;
use crate::ui::NonZeroLength;
use crate::ui::Size;
use crate::ui::ThemeColour;
use crate::ui::Vector;
use crate::ui::relative;
use crate::view::Alignment;
use crate::view::CursorWindow;
use crate::view::Quoted;
use crate::view::RenderArea;
use crate::view::ToText as _;
use crate::view::ruler;
use arcstr::ArcStr;
use arcstr::literal;
use bon::bon;
use getset::CopyGetters;
use getset::Setters;
use itertools::chain;
use saturating_cast::SaturatingCast as _;
use std::cmp::max;
use std::cmp::min;
use std::iter::once;

/// The title of the piano roll pane.
const PIANO_ROLL: ArcStr = literal!("piano roll");
/// The error message shown if no clip is selected.
const NO_CLIP_SELECTED: ArcStr = literal!("please select a clip to edit");
// TODO: add audio clip "editing"
/// The error message shown if an audio clip is selected.
const AUDIO_CLIP_SELECTED: ArcStr = literal!("cannot edit audio clips (yet)");

/// Volatile settings for the piano roll.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Setters, CopyGetters)]
pub struct PianoRoll {
    /// How far to the left the piano roll is moved.
    negative_x_offset: Length,
    /// How far down the piano roll is moved.
    ///
    /// With this set to zero, the bottom key is C<sub>-1</sub>.
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

// TODO: Sort builder parameter & arguments.
#[bon]
impl PianoRoll {
    /// Returns the default piano roll settings for a given ui.
    pub(crate) fn default_in<Ui: UserInterface>() -> PianoRoll {
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

    /// Returns [`self.y_offset`] but clamped such that the piano roll is not scrolled past the top key.
    fn clamped_y_offset<Ui: UserInterface>(self) -> Length {
        let full_roll_height = self.key_width.get() * Ratio::integer(128);
        let workspace_height = self.content_height - Ui::RULER_HEIGHT.get();

        min(self.y_offset, full_roll_height - workspace_height)
    }

    /// Moves the piano roll by an offset.
    pub(crate) fn move_by<Ui: UserInterface>(&mut self, by: Vector) {
        self.negative_x_offset -= by.x;
        self.y_offset += by.y;

        self.y_offset = self.clamped_y_offset::<Ui>();
    }

    /// Returns the view for the piano roll.
    #[builder]
    pub(crate) fn view<Ui: UserInterface>(
        self,
        selection: &Selection,
        project: &Project,
        quantisation: Quantisation,
        cursor: Instant,
        player: Option<Player>,
        held_object: Option<Holdable>,
        edit_mode: bool,
    ) -> Quoted {
        if !self.is_open {
            return Quoted::EMPTY;
        }

        let clip_name = selection
            .top_clip()
            .and_then(|id| project.clip(id))
            .map(|(_, clip)| clip.name());

        let highlighted = clip_name.is_some();
        let title = clip_name.unwrap_or(PIANO_ROLL);

        let content = self
            .content::<Ui>()
            .cursor(cursor)
            .project(project)
            .quantisation(quantisation)
            .selection(selection)
            .maybe_player(player)
            .maybe_held_object(held_object)
            .edit_mode(edit_mode)
            .call();

        let title_height = Ui::string_height(&title) + Ui::TITLE_PADDING * Ratio::integer(2);

        let title = View::TitleBar { title, highlighted };

        View::y_stack([
            title.grabbable(Self::handle_grabber).quoted_minimally(),
            content.fill_remaining(),
        ])
        .quoted(self.content_height + title_height)
    }

    #[builder]
    fn content<Ui: UserInterface>(
        self,
        selection: &Selection,
        project: &Project,
        quantisation: Quantisation,
        player: Option<Player>,
        cursor: Instant,
        held_object: Option<Holdable>,
        edit_mode: bool,
    ) -> View {
        let Some(clip_path) = selection.top_clip() else {
            return NO_CLIP_SELECTED.centred();
        };

        let Some((clip_start, clip)) = project.clip(clip_path) else {
            return NO_CLIP_SELECTED.centred();
        };

        let Some(notes) = clip.content().as_notes() else {
            return AUDIO_CLIP_SELECTED.centred();
        };

        let offset_mapping = OffsetMapping::new(project.time_signature().clone(), quantisation);

        // The piano roll has a fixed lower pitch.
        // Resizing it will thus cause the bottom to be fixed.
        // Since the top is the thing being moved, this seems intuitive.
        let workspace = self
            .workspace::<Ui>()
            .track(clip_path.track)
            .clip_start(clip_start)
            .clip(clip)
            .notes(notes)
            .offset_mapping(offset_mapping.clone())
            .time_context(project.time_context())
            .maybe_player(player)
            .cursor(cursor)
            .maybe_held_object(held_object)
            .edit_mode(edit_mode)
            .key(project.key())
            .call();

        let ruler = ruler(self.negative_x_offset, offset_mapping)
            .fill_remaining()
            .x_positioned(self.piano_depth.get());

        View::y_stack([ruler.quoted(Ui::RULER_HEIGHT), workspace.fill_remaining()])
            .scrollable(Action::MovePianoRoll)
    }

    #[builder]
    fn workspace<Ui: UserInterface>(
        self,
        track: Id<Track>,
        clip_start: Instant,
        clip: &Clip,
        notes: &Group,
        offset_mapping: OffsetMapping,
        time_context: Changing<TimeContext>,
        player: Option<Player>,
        cursor: Instant,
        held_object: Option<Holdable>,
        edit_mode: bool,
        key: &Changing<Key>,
    ) -> View {
        let roll = self
            .roll::<Ui>()
            .track(track)
            .clip_start(clip_start)
            .clip(clip)
            .notes(notes)
            .offset_mapping(&offset_mapping)
            .edit_mode(edit_mode)
            .key(key)
            .call();

        let held_object = self.held_object(held_object, clip.colour(), &offset_mapping);
        let cursor_window = CursorWindow::builder()
            .cursor(cursor)
            .offset_mapping(offset_mapping)
            .maybe_player(player)
            .time_context(time_context)
            .window_offset(self.negative_x_offset)
            .build()
            .view();

        let overlay =
            View::Layers(vec![cursor_window, held_object]).x_positioned(self.piano_depth.get());

        View::Layers(vec![roll, overlay])
    }

    #[builder]
    fn roll<Ui: UserInterface>(
        self,
        track: Id<Track>,
        clip_start: Instant,
        clip: &Clip,
        notes: &Group,
        offset_mapping: &OffsetMapping,
        edit_mode: bool,
        key: &Changing<Key>,
    ) -> View {
        let y_offset = self.clamped_y_offset::<Ui>();

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
            .row()
            .track(track)
            .clip_start(clip_start)
            .clip(clip)
            .edit_mode(edit_mode)
            .key(key)
            .notes(notes)
            .offset_mapping(offset_mapping.clone())
            .pitch(lowest_visible_pitch)
            .call()
            .quoted(lowest_row_height);

        let highest_row = self
            .row()
            .track(track)
            .clip_start(clip_start)
            .clip(clip)
            .edit_mode(edit_mode)
            .key(key)
            .notes(notes)
            .offset_mapping(offset_mapping.clone())
            .pitch(highest_visible_pitch)
            .call()
            .fill_remaining();

        let mut rows = vec![highest_row];

        for semitones in (1..=number_of_full_keys).rev() {
            let interval = Interval::from_semitones(semitones.saturating_cast());
            let pitch = lowest_visible_pitch + interval;

            rows.push(
                self.row()
                    .track(track)
                    .clip_start(clip_start)
                    .clip(clip)
                    .edit_mode(edit_mode)
                    .key(key)
                    .notes(notes)
                    .offset_mapping(offset_mapping.clone())
                    .pitch(pitch)
                    .call()
                    .quoted(self.key_width),
            );
        }

        rows.push(lowest_row);

        View::y_stack(rows)
    }

    /// Return the view for a row in the piano roll.
    #[builder]
    fn row(
        self,
        track: Id<Track>,
        clip_start: Instant,
        clip: &Clip,
        notes: &Group,
        pitch: Pitch,
        offset_mapping: OffsetMapping,
        edit_mode: bool,
        key: &Changing<Key>,
    ) -> View {
        fn grabber(
            negative_x_offset: Length,
            offset_mapping: OffsetMapping,
            edit_mode: bool,
        ) -> impl Fn(RenderArea) -> Option<Holdable> {
            move |render_area: RenderArea| {
                let mouse_position = render_area.relative_mouse_position()?;

                Some(if edit_mode {
                    let start =
                        offset_mapping.quantised_instant(mouse_position.x + negative_x_offset);

                    Holdable::NoteCreation { start }
                } else {
                    Holdable::SelectionBox {
                        start: render_area.mouse_position,
                    }
                })
            }
        }

        let key = key.get(offset_mapping.instant(self.negative_x_offset));

        // TODO:
        //  - colour for out-of-bounds
        //  - draw grid
        //  - highlight key based on settings
        // TODO: use a theme colour
        let background_colour = if (pitch - Pitch::LOWEST).semitones() % 2 == 0 {
            ThemeColour::PianoRollBackground
        } else {
            ThemeColour::AlternatePianoRollBackground
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

                    let path = note::Path::new(track, clip.id(), note.id());

                    Self::note_visual(clip.colour())
                        .selectable(Selectable::Note(path))
                        .quoted(width)
                        .x_positioned(start)
                }),
            )
            .collect(),
        );

        let grabber = grabber(self.negative_x_offset, offset_mapping.clone(), edit_mode);

        let dropper = move |object, render_area: RenderArea| {
            let Holdable::NoteCreation { start } = object else {
                return None;
            };
            let end = offset_mapping.quantised_instant(
                render_area.relative_mouse_position()?.x + self.negative_x_offset,
            );

            let (start, end) = (min(start, end), max(start, end));

            let duration = NonZeroDuration::from_duration(end - start)?;

            Some(Action::Edit(Edit::AddNote {
                position: start,
                pitch,
                duration,
            }))
        };

        let overview = view.grabbable(grabber).object_accepting(dropper);

        View::x_stack([
            self.piano_key(pitch, key).quoted(self.piano_depth),
            overview.fill_remaining(),
        ])
    }

    /// Return a purely visual [view](View) of a note.
    fn note_visual(colour: Colour) -> View {
        View::Solid(ThemeColour::Custom(colour))
    }

    // TODO: use `Button` for:
    //  - plinking the key
    //  - selecting all notes with the key's pitch
    /// Return the view for a key on the piano-roll piano.
    fn piano_key(self, pitch: Pitch, key: Key) -> View {
        let top = View::Solid(if pitch.class().is_black_key() {
            ThemeColour::BlackKey
        } else {
            ThemeColour::WhiteKey
        });

        let text = if pitch.class() == key.tonic {
            ArcStr::from(pitch.name(key.sign))
        } else {
            ArcStr::new()
        }
        .aligned_to(Alignment::BottomRight);

        let bottom = View::Layers(vec![View::Solid(ThemeColour::WhiteKey), text]);

        View::x_stack([top.quoted(self.black_key_depth), bottom.fill_remaining()])
    }

    /// Returns the [holdable object](Holdable) representing the handle (top edge) of the piano roll.
    fn handle_grabber(render_area: RenderArea) -> Option<Holdable> {
        let y = render_area.relative_mouse_position()?.y;

        Some(Holdable::PianoRollHandle { y })
    }

    /// Returns the view for the object held inside the piano roll.
    fn held_object(
        self,
        held_object: Option<Holdable>,
        clip_colour: Colour,
        offset_mapping: &OffsetMapping,
    ) -> View {
        let Some(held_object) = held_object else {
            return View::Empty;
        };

        let start = match held_object {
            Holdable::NoteCreation { start } => start,
            Holdable::Clip(_)
            | Holdable::PianoRollHandle { .. }
            | Holdable::Popup { .. }
            | Holdable::PopupSide { .. }
            | Holdable::SelectionBox { .. } => {
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

            Self::note_visual(clip_colour)
                .quoted_2d(size)
                .positioned(position)
        })
    }
}
