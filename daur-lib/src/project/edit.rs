use crate::audio::{FixedLength, ImportAudioError};
use crate::metre::{Instant, NonZeroDuration, NonZeroInstant};
use crate::note::{Key, Pitch};
use crate::project::track::{Clip, ClipInsertionError, clip};
use crate::project::{DEFAULT_NOTES_DURATION, HistoryEntry, Track, track};
use crate::select::Selection;
use crate::{Audio, Note, Project, note};
use arcstr::ArcStr;
use mitsein::iter1::IteratorExt as _;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::mem::replace;
use std::path::PathBuf;
use thiserror::Error;

/// An edit to a [project](super::Project).
#[derive(Clone, Debug)]
#[remain::sorted]
pub enum Edit {
    /// Adds a note to the selected clip.
    AddNote {
        /// The position of the note.
        position: Instant,
        /// The pitch of the note.
        pitch: Pitch,
        /// The note.
        duration: NonZeroDuration,
    },
    /// Inserts an empty note clip into the selected track at the cursor.
    AddNoteGroup,
    /// Adds an empty track.
    AddTrack,
    /// Deletes the selected item(s).
    Delete,
    /// Deletes some clips.
    DeleteClips(HashSet<clip::Id>),
    /// Deletes some notes.
    DeleteNotes(HashSet<note::Id>),
    /// Deletes a track.
    DeleteTracks(HashSet<track::Id>),
    /// Imports an audio file into the selected track at the cursor.
    ImportAudio {
        /// The path to the file.
        file: PathBuf,
    },
    /// Sets the key at an instant in the project.
    SetKey {
        /// The instant of the key change.
        instant: Instant,
        /// The new key.
        key: Key,
    },
}

#[derive(Debug, Error)]
#[remain::sorted]
pub enum Error {
    /// Tried inserting a note outside the selected clip.
    #[error("{0}")]
    ClipInsertion(#[from] ClipInsertionError),
    /// Failed to import audio from a file.
    #[error("{0}")]
    ImportAudio(#[from] ImportAudioError),
    /// The action required a clip to be selected.
    #[error("no clip is selected")]
    NoClipSelected,
    /// The action required a note to be selected.
    #[error("no note is selected")]
    NoNoteSelected,
    /// The action required a note clip to be selected.
    #[error("the selected clip is not a note clip")]
    NonNoteCLip,
    /// The action required a track to be selected.
    #[error("no track is selected")]
    NoTrackSelected,
    /// Tried inserting a note outside the selected clip.
    #[error("{0}")]
    NoteInsertion(#[from] note::InsertionError),
    /// The action required something to be selected.
    #[error("nothing is selected")]
    NothingSelected,
}

impl Project {
    fn resolve_track(&mut self, selection: &Selection) -> Result<&mut Track, Error> {
        self.track_mut(selection.top_track().ok_or(Error::NoTrackSelected)?)
            .ok_or(Error::NoTrackSelected)
    }

    fn resolve_clip(&mut self, selection: &Selection) -> Result<(Instant, &mut Clip), Error> {
        self.clip_mut(selection.top_clip().ok_or(Error::NoClipSelected)?)
            .ok_or(Error::NoClipSelected)
    }

    // TODO: `EditError`
    #[expect(clippy::too_many_lines, reason = "`Edit` is a large enum")]
    #[remain::check]
    pub(crate) fn edit(
        &mut self,
        action: Edit,
        cursor: Instant,
        selection: &mut Selection,
    ) -> Result<HistoryEntry, Error> {
        #[sorted]
        match action {
            Edit::AddNote {
                position,
                pitch,
                mut duration,
            } => {
                let (clip_start, clip) = self.resolve_clip(selection)?;

                if position < clip_start {
                    let difference = clip_start - position;
                    let max_duration = NonZeroDuration::from_duration(duration.get() - difference)
                        .ok_or(Error::NoteInsertion(note::InsertionError::OutsideClip))?;

                    duration = max_duration;
                }

                let position = position.relative_to(clip_start);

                let note = Note::new(duration, clip.id());

                let entry = HistoryEntry::InsertNote(note.id());

                clip.content_mut()
                    .as_notes_mut()
                    .ok_or(Error::NonNoteCLip)?
                    .try_insert(position, pitch, note)?;

                Ok(entry)
            }
            Edit::AddNoteGroup => {
                let track = self.resolve_track(selection)?;

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
                let action = if let Some(notes) = selection.take_notes() {
                    Edit::DeleteNotes(notes)
                } else if let Some(clips) = selection.take_clips() {
                    Edit::DeleteClips(clips)
                } else if let Some(tracks) = selection.take_tracks() {
                    Edit::DeleteTracks(tracks)
                } else {
                    return Err(Error::NothingSelected);
                };

                self.edit(action, cursor, selection)
            }
            Edit::DeleteClips(clips) => clips
                .into_iter()
                .filter_map(|clip| {
                    let track = self.track_mut(clip.track())?;
                    let (start, clip) = track.remove_clip(clip)?;

                    Some(HistoryEntry::DeleteClip { start, clip })
                })
                .try_collect1()
                .map_err(|_empty| Error::NoClipSelected),
            Edit::DeleteNotes(notes) => notes
                .into_iter()
                .filter_map(|note| {
                    let (_, clip) = self.clip_mut(note.clip())?;

                    let (instant, pitch, note) = clip.content_mut().as_notes_mut()?.remove(note)?;

                    Some(HistoryEntry::DeleteNote {
                        instant,
                        pitch,
                        note,
                    })
                })
                .try_collect1()
                .map_err(|_empty| Error::NoNoteSelected),
            Edit::DeleteTracks(tracks) => tracks
                .into_iter()
                .filter_map(|track| {
                    let index = self.tracks.get_index_of(&track)?;
                    let track = self.tracks.shift_remove(&track)?;

                    Some(HistoryEntry::DeleteTrack { index, track })
                })
                .try_collect1()
                .map_err(|_empty| Error::NoTrackSelected),
            Edit::ImportAudio { file } => {
                let time_context = self.time_context();

                let track = self.resolve_track(selection)?;

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
