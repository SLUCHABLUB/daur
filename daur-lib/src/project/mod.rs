//! Items pertaining to [`Project`].

pub mod track;

mod action;
mod bar;
mod history;
mod manager;
mod renderer;
mod workspace;

pub use action::Edit;
pub use manager::Manager;
use std::ffi::OsStr;

#[doc(inline)]
pub use track::Track;

pub(crate) use bar::bar;
pub(crate) use history::HistoryEntry;
pub(crate) use renderer::Renderer;
pub(crate) use workspace::workspace;

use crate::audio::FixedLength;
use crate::metre::{
    Changing, Instant, NonZeroDuration, NonZeroInstant, TimeContext, TimeSignature,
};
use crate::note::Key;
use crate::project::track::{Clip, clip};
use crate::select::Selection;
use crate::time::Tempo;
use crate::{Audio, NonZeroRatio, Note};
use anyhow::{Result, anyhow, bail};
use arcstr::{ArcStr, literal};
use getset::{CloneGetters, Getters};
use indexmap::IndexMap;
use mitsein::iter1::IteratorExt as _;
use non_zero::non_zero;
use std::mem::replace;

const ADD_TRACK_LABEL: ArcStr = literal!("+");
const ADD_TRACK_DESCRIPTION: ArcStr = literal!("add track");
const DEFAULT_TRACK_TITLE: ArcStr = literal!("a track");

const DEFAULT_NOTES_DURATION: NonZeroDuration = NonZeroDuration {
    whole_notes: NonZeroRatio::integer(non_zero!(4)),
};

// TODO: Test that this isn't `Clone` (bc. id).
/// A musical piece consisting of multiple [tracks](Track).
#[cfg_attr(doc, doc(hidden))]
#[derive(Debug, Default, Getters, CloneGetters)]
pub struct Project {
    /// The name of the project.
    #[get_clone = "pub"]
    title: ArcStr,

    // TODO: continuous change
    /// The tempo of the project
    tempo: Changing<Tempo>,
    /// The time signature of the project.
    #[get = "pub(crate)"]
    time_signature: Changing<TimeSignature>,
    /// The key of the project.
    #[get = "pub(crate)"]
    key: Changing<Key>,

    /// The tracks in the project.
    tracks: IndexMap<track::Id, Track>,
}

impl Project {
    /// Returns a reference to a track.
    #[must_use]
    pub(super) fn track(&self, id: track::Id) -> Option<&Track> {
        self.tracks.get(&id)
    }

    /// Returns a mutable reference to a track.
    #[must_use]
    fn track_mut(&mut self, id: track::Id) -> Option<&mut Track> {
        self.tracks.get_mut(&id)
    }

    /// Returns a reference to a clip.
    #[must_use]
    pub(super) fn clip(&self, id: clip::Id) -> Option<(Instant, &Clip)> {
        self.track(id.track())?.clip(id)
    }

    /// Returns a mutable reference to a clip.
    #[must_use]
    fn clip_mut(&mut self, id: clip::Id) -> Option<(Instant, &mut Clip)> {
        self.track_mut(id.track())?.clip_mut(id)
    }

    fn resolve_track(&mut self, selection: &Selection) -> Option<&mut Track> {
        self.track_mut(selection.top_track()?)
    }

    fn resolve_clip(&mut self, selection: &Selection) -> Option<(Instant, &mut Clip)> {
        self.clip_mut(selection.top_clip()?)
    }

    pub(crate) fn time_context(&self) -> Changing<TimeContext> {
        &self.time_signature / &self.tempo
    }

    // TODO: `EditError`
    #[expect(clippy::too_many_lines, reason = "`Edit` is a large enum")]
    #[remain::check]
    pub(crate) fn edit(
        &mut self,
        action: Edit,
        cursor: Instant,
        selection: &mut Selection,
    ) -> Result<HistoryEntry> {
        #[sorted]
        match action {
            Edit::AddNote {
                position,
                pitch,
                mut duration,
            } => {
                let (clip_start, clip) = self
                    .resolve_clip(selection)
                    .ok_or(anyhow!("no clip selected"))?;

                if position < clip_start {
                    let difference = clip_start - position;
                    let max_duration = NonZeroDuration::from_duration(duration.get() - difference)
                        .ok_or(anyhow!("cannot insert a note outside the clip"))?;

                    duration = max_duration;
                }

                let position = position.relative_to(clip_start);

                let note = Note::new(duration, clip.id());

                let entry = HistoryEntry::InsertNote(note.id());

                clip.content_mut()
                    .as_notes_mut()
                    .ok_or(anyhow!("cannot add notes to a non-notes clip"))?
                    .try_insert(position, pitch, note)?;

                Ok(entry)
            }
            Edit::AddNoteGroup => {
                let track = self
                    .resolve_track(selection)
                    .ok_or(anyhow!("no track selected"))?;

                let clip = Clip::empty_notes(DEFAULT_NOTES_DURATION, track.id());
                let id = clip.id();

                track.try_insert_clip(cursor, clip)?;

                Ok(HistoryEntry::InsertClip(id))
            }
            Edit::AddTrack => {
                let track = Track::new();
                let id = track.id();

                selection.push_track(id);
                self.tracks.insert(id, track);

                Ok(HistoryEntry::AddTrack(id))
            }
            Edit::Delete => {
                let action = if let Some(_notes) = selection.take_notes() {
                    // TODO: delete notes
                    bail!("cannot delete notes yet");
                } else if let Some(clips) = selection.take_clips() {
                    Edit::DeleteClips(clips)
                } else if let Some(tracks) = selection.take_tracks() {
                    Edit::DeleteTracks(tracks)
                } else {
                    bail!("nothing is selected");
                };

                self.edit(action, cursor, selection)
            }
            Edit::DeleteClips(clips) => {
                let track = self
                    .resolve_track(selection)
                    .ok_or(anyhow!("no track selected"))?;

                clips
                    .into_iter()
                    .filter_map(|id| {
                        let (start, clip) = track.remove_clip(id)?;

                        Some(HistoryEntry::DeleteClip { start, clip })
                    })
                    .try_collect1()
                    .map_err(|_empty| anyhow!("no clips selected"))
            }
            Edit::DeleteTracks(tracks) => tracks
                .into_iter()
                .filter_map(|track| {
                    let index = self.tracks.get_index_of(&track)?;
                    let track = self.tracks.shift_remove(&track)?;

                    Some(HistoryEntry::DeleteTrack { index, track })
                })
                .try_collect1()
                .map_err(|_empty| anyhow!("no tracks selected")),
            Edit::ImportAudio { file } => {
                let time_context = self.time_context();

                let track = self
                    .resolve_track(selection)
                    .ok_or(anyhow!("no track selected"))?;

                let audio = Audio::read_from_file(&file)?;

                let audio = FixedLength::from_audio(audio, cursor, &time_context);

                let name = file
                    .file_stem()
                    .map(OsStr::to_string_lossy)
                    .map(ArcStr::from)
                    .unwrap_or_default();

                let clip = Clip::from_audio(name, audio, track.id());

                let entry = HistoryEntry::InsertClip(clip.id());

                track.try_insert_clip(cursor, clip)?;

                Ok(entry)
            }
            Edit::SetKey { instant, key } => {
                let old = if let Some(position) = NonZeroInstant::from_instant(instant) {
                    self.key.changes.insert(position, key)
                } else {
                    Some(replace(&mut self.key.start, key))
                };

                Ok(HistoryEntry::SetKey {
                    at: instant,
                    to: key,
                    from: old,
                })
            }
        }
    }
}
