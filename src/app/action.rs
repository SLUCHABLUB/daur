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
use std::time::SystemTime;

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

                // Since the cursor only technically moves when playback stops,
                // we need to reset it. Shouldn't be noticeable though.
                if app.playback_start.is_some() {
                    app.playback_start = Some(SystemTime::now());
                }
            }
            Action::OpenPopup(popup) => {
                let mut app = app.write_lock();
                app.popups.push(*popup);
            }
            Action::Pause => {
                let mut app = app.write_lock();
                app.cursor = app.playback_position();
                app.playback_start = None;
            }
            Action::Play => {
                let mut app = app.write_lock();
                app.playback_start = Some(SystemTime::now());
            }
            Action::PlayPause => {
                let mut app = app.write_lock();
                if app.playback_start.is_some() {
                    app.cursor = app.playback_position();
                    app.playback_start = None;
                } else {
                    app.playback_start = Some(SystemTime::now());
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
