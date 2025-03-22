use crate::Colour;
use crate::audio::Audio;
use crate::clip::{Clip, Content};
use crate::key::Key;
use crate::notes::Notes;
use crate::popup::Popup;
use crate::project::Action;
use crate::ratio::Ratio;
use crate::time::{Duration, Instant};
use crate::track::Track;
use arcstr::{ArcStr, literal};
use hound::WavReader;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use thiserror::Error;

const DEFAULT_NOTES_NAME: ArcStr = literal!("some notes");
const DEFAULT_NOTES_COLOUR: Colour = Colour {
    red: 255,
    green: 0,
    blue: 255,
};
const DEFAULT_NOTES_DURATION: Duration = Duration {
    whole_notes: Ratio::integer(4),
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
#[error("Unable to infer the audio format of file `{file}`")]
pub struct NoExtensionError {
    pub file: PathBuf,
}

// IMPORTANT: don't use any reference counters in here
//            the history will contain clones of each edit
#[derive(Clone, Debug)]
pub enum Edit {
    /// Inserts the clip into the selected track at the cursor
    AddClip {
        track: usize,
        position: Instant,
        clip: Clip,
    },
    /// Adds a track
    AddTrack(Track),
    /// Adds a key change
    ChangeKey { position: Instant, key: Key },
}

impl Edit {
    pub fn from_action(
        action: Action,
        cursor: Instant,
        selected_track: usize,
    ) -> Result<Edit, Popup> {
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
                    return Err(Popup::from(NoExtensionError { file }));
                };

                // TODO: look at the symphonia crate
                let audio = match extension.to_string_lossy().as_ref() {
                    "wav" | "wave" => {
                        let reader = WavReader::open(&file)?;
                        Audio::try_from(reader)?
                    }
                    _ => {
                        return Err(Popup::from(UnsupportedFormatError {
                            format: extension.to_owned(),
                        }));
                    }
                };

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
