use crate::app::error::{NoExtensionError, UnsupportedFormatError};
use crate::app::reference::AppShare;
use crate::clip::audio::Audio;
use crate::clip::Clip;
use crate::id::Id;
use crate::popup::Popup;
use crate::time::instant::Instant;
use crate::track::Track;
use hound::WavReader;
use ratatui_explorer::Input;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

macro_rules! error {
    ($error:expr, $app:ident) => {{
        let popup = Popup::from($error);
        $app.popups.push(popup);
        return;
    }};
}

macro_rules! or_popup {
    ($result:expr, $app:ident) => {
        match $result {
            Ok(ok) => ok,
            Err(error) => error!(error, $app),
        }
    };
}

#[derive(Clone, Debug, Default)]
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
    /// Imports an audio file into a track
    ImportAudio {
        file: PathBuf,
        track: Id<Track>,
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
    // TODO: add scripting
}

impl Action {
    pub fn take(self, app: &AppShare) {
        // TODO: add to action tree
        match self {
            Action::None => (),
            Action::AddTrack => {
                let mut app = app.write_lock();
                let track = Track::new();
                app.selected_track = track.id;
                app.project.tracks.push(track);
            }
            Action::ClosePopup(id) => {
                let mut app = app.write_lock();
                if let Some(position) = app.popups.iter().position(|popup| popup.info().id() == id)
                {
                    app.popups.remove(position);
                }
            }
            Action::Exit => app.set_exit(),
            Action::ImportAudio {
                file,
                track: track_id,
            } => {
                let mut app = app.write_lock();

                let Some(extension) = file.extension() else {
                    error!(NoExtensionError { file }, app);
                };

                let audio = match extension.to_string_lossy().as_ref() {
                    "wav" | "wave" => {
                        let reader = or_popup!(WavReader::open(&file), app);
                        or_popup!(Audio::try_from(reader), app)
                    }
                    _ => error!(
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
                    .find(|track| track.id == track_id)
                {
                    track.clips.insert(instant, Clip::from_audio(name, audio));
                }
            }
            Action::MoveCursor(instant) => {
                let mut app = app.write_lock();

                app.cursor = instant;

                if app.is_playing() {
                    app.start_playback();
                }
            }
            Action::OpenPopup(popup) => {
                app.write_lock().popups.push(*popup);
            }
            Action::Pause => {
                app.write_lock().stop_playback();
            }
            Action::Play => {
                app.write_lock().start_playback();
            }
            Action::PlayPause => {
                let mut app = app.write_lock();
                if app.is_playing() {
                    app.stop_playback();
                } else {
                    app.start_playback();
                }
            }
            Action::Select { popup: id, index } => {
                let mut app = app.write_lock();

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
        }
    }
}
