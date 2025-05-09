use crate::NonZeroRatio;
use crate::audio::{Audio, NonEmpty};
use crate::clip::{Clip, Content};
use crate::key::Key;
use crate::metre::{Instant, NonZeroDuration};
use crate::notes::Notes;
use crate::project::Action;
use crate::track::Track;
use crate::ui::Colour;
use anyhow::{Result, anyhow, bail};
use arcstr::{ArcStr, literal};
use hound::WavReader;
use non_zero::non_zero;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::sync::Weak;
use thiserror::Error;

const DEFAULT_NOTES_NAME: ArcStr = literal!("some notes");
const DEFAULT_NOTES_COLOUR: Colour = Colour {
    red: 255,
    green: 0,
    blue: 255,
};
const DEFAULT_NOTES_DURATION: NonZeroDuration = NonZeroDuration {
    whole_notes: NonZeroRatio::integer(non_zero!(4)),
};

const DEFAULT_AUDIO_COLOUR: Colour = Colour {
    red: 0,
    green: 255,
    blue: 0,
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

#[derive(Clone, Debug)]
pub enum Edit {
    /// Inserts the clip into the selected track at the cursor
    AddClip {
        track: Weak<Track>,
        position: Instant,
        clip: Clip,
    },
    /// Adds a track
    AddTrack(Track),
    /// Adds a key change
    ChangeKey { position: Instant, key: Key },
}

impl Edit {
    // TODO: EditError?
    pub fn from_action(
        action: Action,
        cursor: Instant,
        selected_track: Weak<Track>,
    ) -> Result<Edit> {
        Ok(match action {
            Action::AddNotes => Edit::AddClip {
                track: selected_track,
                position: cursor,
                clip: Clip {
                    name: DEFAULT_NOTES_NAME,
                    colour: DEFAULT_NOTES_COLOUR,
                    content: Content::Notes(Notes::empty(DEFAULT_NOTES_DURATION)),
                },
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
                    track: selected_track,
                    position: cursor,
                    clip: Clip {
                        name,
                        colour: DEFAULT_AUDIO_COLOUR,
                        content: Content::Audio(audio),
                    },
                }
            }
            Action::SetKey { instant, key } => Edit::ChangeKey {
                position: instant,
                key,
            },
        })
    }
}
