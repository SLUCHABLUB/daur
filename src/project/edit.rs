use crate::audio::Audio;
use crate::clip::content::Content;
use crate::clip::Clip;
use crate::key::Key;
use crate::notes::Notes;
use crate::popup::Popup;
use crate::project::Action;
use crate::time::duration::Duration;
use crate::time::instant::Instant;
use crate::time::Ratio;
use crate::track::Track;
use hound::WavReader;
use ratatui::prelude::Color;
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

const DEFAULT_NOTES_NAME: &str = "some notes";
const DEFAULT_NOTES_COLOUR: Color = Color::Magenta;
const DEFAULT_NOTES_DURATION: Duration = Duration {
    whole_notes: Ratio::int(4),
};

const DEFAULT_AUDIO_COLOUR: Color = Color::Green;

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

#[derive(Clone)]
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
    ) -> Result<Edit, Arc<Popup>> {
        Ok(match action {
            Action::AddNotes => Edit::AddClip {
                track: selected_track,
                position: cursor,
                clip: Clip {
                    name: DEFAULT_NOTES_NAME.to_owned(),
                    colour: DEFAULT_NOTES_COLOUR,
                    content: Content::Notes(Notes::empty(DEFAULT_NOTES_DURATION)),
                },
            },
            Action::AddTrack => Edit::AddTrack(Track::new()),
            Action::ImportAudio { file } => {
                let Some(extension) = file.extension() else {
                    return Err(Popup::error(NoExtensionError { file }));
                };

                // TODO: look at the symphonia crate
                let audio = match extension.to_string_lossy().as_ref() {
                    "wav" | "wave" => {
                        let reader = WavReader::open(&file).map_err(Popup::error)?;
                        Audio::try_from(reader).map_err(Popup::error)?
                    }
                    _ => {
                        return Err(Popup::error(UnsupportedFormatError {
                            format: extension.to_owned(),
                        }))
                    }
                };

                let name = file
                    .file_stem()
                    .map(OsStr::to_string_lossy)
                    .map(Cow::into_owned)
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
            Action::SetDefaultKey(key) => Edit::ChangeKey {
                position: Instant::START,
                key,
            },
        })
    }
}
