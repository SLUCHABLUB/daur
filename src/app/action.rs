use crate::app::error::{NoExtensionError, UnsupportedFormatError};
use crate::app::{or_popup, popup_error, App};
use crate::audio::Audio;
use crate::clip::Clip;
use crate::id::Id;
use crate::popup::Popup;
use crate::time::instant::Instant;
use crate::track::Track;
use hound::WavReader;
use ratatui_explorer::Input;
use rodio::Device;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Clone, Default)]
pub enum Action {
    /// Does nothing
    #[default]
    None,
    /// Add an empty track
    AddTrack,
    /// Close the popup with the given uuid
    ClosePopup(Id<Popup>),
    /// Save and exit the program
    Exit,
    /// Imports an audio file into the selected track
    ImportAudio {
        file: PathBuf,
    },
    /// Moves the (musical) cursor.
    MoveCursor(Instant),
    OpenPopup(Box<Popup>),
    /// Stop playing
    Pause,
    /// Start playing
    Play,
    /// `Play` or `Pause`
    PlayPause,
    /// Select an option in a popup
    Select {
        popup: Id<Popup>,
        index: usize,
    },
    /// Sets the audio output device
    SetDevice(Device),
    // TODO: add scripting
}

impl Action {
    pub fn take(self, app: &mut App) {
        match self {
            Action::AddTrack
            | Action::ClosePopup(_)
            | Action::ImportAudio { .. }
            | Action::MoveCursor(_)
            | Action::OpenPopup(_)
            | Action::Pause
            | Action::Play
            | Action::PlayPause
            | Action::Select { .. } => app.should_redraw = true,
            Action::None | Action::Exit | Action::SetDevice(_) => (),
        }

        match self {
            Action::None => (),
            Action::AddTrack => {
                let track = Track::new();
                app.selected_track = track.id;
                app.project.tracks.push(track);
            }
            Action::ClosePopup(id) => {
                if let Some(position) = app.popups.iter().position(|popup| popup.info().id() == id)
                {
                    app.popups.remove(position);
                }
            }
            Action::Exit => app.should_exit = true,
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

                let instant = app.cursor;

                if let Some(track) = app
                    .project
                    .tracks
                    .iter_mut()
                    .find(|track| track.id == app.selected_track)
                {
                    track.clips.insert(instant, Clip::from_audio(name, audio));
                }
            }
            Action::MoveCursor(instant) => {
                app.cursor = instant;

                if app.is_playing() {
                    app.start_playback();
                }
            }
            Action::OpenPopup(popup) => {
                app.popups.push(*popup);
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
            Action::Select { popup: id, index } => {
                if let Some(popup) = app.popups.iter_mut().find(|popup| popup.info().id() == id) {
                    match popup {
                        Popup::Explorer(ref mut popup) => {
                            let explorer = &mut popup.explorer;
                            if explorer.selected_idx() == index {
                                or_popup!(explorer.handle(Input::Right), app);
                            } else if index < explorer.files().len() {
                                explorer.set_selected_idx(index);
                            }
                        }
                        Popup::Buttons(_) | Popup::Error(_) => (),
                    }
                }
            }
            Action::SetDevice(device) => {
                app.device = Some(device);
            }
        }

        // TODO: add to action tree
    }
}
