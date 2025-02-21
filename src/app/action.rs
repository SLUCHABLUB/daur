#![allow(
    clippy::used_underscore_binding,
    reason = "educe generates names with underscores in their implementations"
)]

use crate::app::App;
use crate::popup::Popup;
use crate::project;
use crate::time::instant::Instant;
use educe::Educe;
use rodio::Device;
use std::path::PathBuf;
use std::sync::{Arc, Weak};

#[derive(Clone, Default, Educe)]
#[educe(Eq, PartialEq)]
pub enum Action {
    /// Does nothing
    #[default]
    None,

    /// Opens the popup
    OpenPopup(Arc<Popup>),
    /// Closes the popup
    ClosePopup(#[educe(Eq(ignore))] Weak<Popup>),

    /// Moves the (musical) cursor.
    MoveCursor(Instant),
    /// Selects the given track
    SelectTrack(usize),

    /// Stop playing
    Pause,
    /// Start playing
    Play,
    /// `Play` or `Pause`
    PlayPause,

    Project(project::Action),

    /// Sets the audio output device
    SetDevice(#[educe(Eq(ignore))] Device),

    /// Saves and exits the program
    Exit,
    // TODO: add scripting
}

impl Action {
    pub fn import_audio<P: Into<PathBuf>>(file: P) -> Action {
        Action::Project(project::Action::ImportAudio { file: file.into() })
    }

    pub fn take(self, app: &App) {
        match self {
            Action::None => (),
            Action::ClosePopup(popup) => {
                app.popups.close(&popup);
            }
            Action::Exit => app.should_exit.set(true),
            Action::MoveCursor(instant) => {
                app.cursor.set(instant);

                if app.is_playing() {
                    app.start_playback();
                }
            }
            Action::OpenPopup(popup) => {
                app.popups.open(popup);
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
            Action::Project(action) => {
                let result =
                    app.project
                        .handle(action, app.cursor.get(), app.selected_track_index.get());

                if let Err(popup) = result {
                    app.popups.open(popup);
                }
            }
            Action::SelectTrack(index) => {
                app.selected_track_index.set(index);
            }
            Action::SetDevice(device) => {
                app.device.set(Some(device));
            }
        }

        // TODO: add to action tree
    }
}
