use crate::audio::FixedLength;
use crate::metre::{Instant, NonZeroDuration, NonZeroInstant};
use crate::note::{Key, Pitch};
use crate::project::track::{Clip, clip};
use crate::project::{DEFAULT_NOTES_DURATION, HistoryEntry, Track, track};
use crate::select::Selection;
use crate::{Audio, Note, Project};
use anyhow::{anyhow, bail};
use arcstr::ArcStr;
use mitsein::iter1::IteratorExt as _;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::mem::replace;
use std::path::PathBuf;

/// An action to take on a [project](super::Project).
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

impl Project {
    // TODO: `EditError`
    #[expect(clippy::too_many_lines, reason = "`Edit` is a large enum")]
    #[remain::check]
    pub(crate) fn edit(
        &mut self,
        action: Edit,
        cursor: Instant,
        selection: &mut Selection,
    ) -> anyhow::Result<HistoryEntry> {
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
