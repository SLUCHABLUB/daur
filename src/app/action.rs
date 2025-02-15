// educe generates names with underscores in their implementations
#![allow(clippy::used_underscore_binding)]

use crate::app::error::{NoExtensionError, UnsupportedFormatError};
use crate::app::{or_popup, popup_error, App};
use crate::audio::Audio;
use crate::clip::Clip;
use crate::key::Key;
use crate::popup::Popup;
use crate::time::instant::Instant;
use crate::track::Track;
use educe::Educe;
use hound::WavReader;
use rodio::Device;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::{Arc, Weak};

#[derive(Clone, Default, Educe)]
#[educe(Eq, PartialEq)]
pub enum Action {
    /// Does nothing
    #[default]
    None,
    /// Add an empty track
    AddTrack,
    /// Close the popup with the given uuid
    ClosePopup(#[educe(Eq(ignore))] Weak<Popup>),
    /// Save and exit the program
    Exit,
    /// Imports an audio file into the selected track
    ImportAudio {
        file: PathBuf,
    },
    /// Moves the (musical) cursor.
    MoveCursor(Instant),
    OpenPopup(Arc<Popup>),
    /// Stop playing
    Pause,
    /// Start playing
    Play,
    /// `Play` or `Pause`
    PlayPause,
    /// Sets the audio output device
    SetDevice(#[educe(Eq(ignore))] Device),
    SetKey(Instant, Key),
    // TODO: add scripting
}

impl Action {
    pub fn take(self, app: &App) {
        match self {
            Action::None => (),
            Action::AddTrack => {
                let index = app.project.tracks.push(Arc::new(Track::new()));

                app.selected_track_index.set(index);
            }
            Action::ClosePopup(popup) => {
                app.popups.remove(&popup);
            }
            Action::Exit => app.should_exit.set(true),
            Action::ImportAudio { file } => {
                let Some(extension) = file.extension() else {
                    popup_error!(NoExtensionError { file }, app);
                };

                // TODO: look at the symphonia crate
                let audio = match extension.to_string_lossy().as_ref() {
                    "wav" | "wave" => {
                        let reader = or_popup!(WavReader::open(&file), app);
                        or_popup!(Audio::try_from(reader), app)
                    }
                    _ => popup_error!(
                        UnsupportedFormatError {
                            format: extension.to_owned()
                        },
                        app
                    ),
                };

                let name = file
                    .file_stem()
                    .map(OsStr::to_string_lossy)
                    .map(Cow::into_owned)
                    .unwrap_or_default();

                let instant = app.cursor.get();

                app.project
                    .tracks
                    .update(app.selected_track_index.get(), |track| {
                        track
                            .clips
                            .insert(instant, Arc::new(Clip::from_audio(name, audio)));
                    });
            }
            Action::MoveCursor(instant) => {
                app.cursor.set(instant);

                if app.is_playing() {
                    app.start_playback();
                }
            }
            Action::OpenPopup(popup) => {
                app.popups.push(popup);
            }
            Action::Pause => {
                app.stop_playback();
            }
            Action::Play => {
                app.start_playback();
            }
            Action::PlayPause => {
                if app.is_playing() {
                    app.stop_playback();
                } else {
                    app.start_playback();
                }
            }
            Action::SetDevice(device) => {
                app.device.set(Some(device));
            }
            Action::SetKey(instant, key) => {
                if instant == Instant::START {
                    app.project.key.start.set(key);
                } else {
                    app.project.key.changes.insert(instant, key);
                }
            }
        }

        // TODO: add to action tree
    }
}
