use crate::app::Selection;
use crate::audio::{Audio, NonEmpty};
use crate::metre::{Instant, NonZeroDuration};
use crate::notes::{Key, Note, Pitch};
use crate::project::Action;
use crate::track::Track;
use crate::{Clip, Id, NonZeroRatio};
use anyhow::{Result, anyhow, bail};
use arcstr::ArcStr;
use hound::WavReader;
use non_zero::non_zero;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use thiserror::Error;

const DEFAULT_NOTES_DURATION: NonZeroDuration = NonZeroDuration {
    whole_notes: NonZeroRatio::integer(non_zero!(4)),
};

#[derive(Debug, Error)]
#[error("The format `{}` is not (yet) supported", format.to_string_lossy())]
pub struct UnsupportedFormatError {
    pub format: OsString,
}

#[derive(Debug, Error)]
#[error("Unable to infer the audio format of the file `{file}`")]
pub struct NoExtensionError {
    pub file: PathBuf,
}

#[derive(Debug)]
pub enum Edit {
    /// Inserts a note into a clip.
    AddNote {
        /// The track.
        track: Id<Track>,
        /// The clip.
        clip: Id<Clip>,
        /// The position at which to insert the clip.
        position: Instant,
        /// The pitch at which to insert the note.
        pitch: Pitch,
        /// The note.
        note: Note,
    },
    /// Inserts a clip into a track.
    AddClip {
        /// The track.
        track: Id<Track>,
        /// The position at which to insert the clip.
        position: Instant,
        /// The clip to insert.
        clip: Clip,
    },
    /// Adds a track
    AddTrack(Track),
    /// Adds a key change
    ChangeKey { position: Instant, key: Key },
}

impl Edit {
    // TODO: EditError?
    pub fn from_action(action: Action, cursor: Instant, selection: &Selection) -> Result<Edit> {
        Ok(match action {
            Action::AddNote {
                position,
                pitch,
                note,
            } => Edit::AddNote {
                track: selection.track(),
                clip: selection.clip(),
                position,
                pitch,
                note,
            },
            Action::AddNotes => Edit::AddClip {
                track: selection.track(),
                position: cursor,
                clip: Clip::empty_notes(DEFAULT_NOTES_DURATION),
            },
            Action::AddTrack => Edit::AddTrack(Track::new()),
            Action::ImportAudio { file } => {
                let Some(extension) = file.extension() else {
                    bail!(NoExtensionError { file });
                };

                // TODO: look at the symphonia crate
                let audio = match extension.to_string_lossy().as_ref() {
                    "wav" | "wave" => {
                        let reader = WavReader::open(&file)?;
                        Audio::try_from(reader)?
                    }
                    _ => {
                        bail!(UnsupportedFormatError {
                            format: extension.to_owned(),
                        });
                    }
                };

                let audio = NonEmpty::from_audio(audio)
                    .ok_or(anyhow!("cannot insert an empty audio clip"))?;

                let name = file
                    .file_stem()
                    .map(OsStr::to_string_lossy)
                    .map(ArcStr::from)
                    .unwrap_or_default();

                Edit::AddClip {
                    track: selection.track(),
                    position: cursor,
                    clip: Clip::from_audio(name, audio),
                }
            }
            Action::SetKey { instant, key } => Edit::ChangeKey {
                position: instant,
                key,
            },
        })
    }
}
