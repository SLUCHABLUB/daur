#![allow(
    clippy::used_underscore_binding,
    reason = "educe generates code with underscore prefixes"
)]

use crate::popup::Popup;
use crate::time::Instant;
use crate::ui::Length;
use crate::{project, App, Ratio};
use derive_more::Debug;
use rodio::Device;
use std::path::PathBuf;
use std::sync::{Arc, Weak};

/// An action to take on the app
#[derive(Clone, Debug)]
pub enum Action {
    /// Opens the popup
    OpenPopup(Arc<Popup>),
    /// Closes the popup
    ClosePopup(#[debug(skip)] Weak<Popup>),

    /// Moves the (musical) cursor.
    MoveCursor(Instant),
    /// Selects the track with the given index
    SelectTrack(usize),
    /// Selects a clip
    SelectClip {
        /// The index of the track in which the clip resides
        track_index: usize,
        /// The index of the clip to select
        index: usize,
    },

    /// Sets the piano roll's height to half the screen
    OpenPianoRoll,
    /// Sets the piano roll's height
    SetPianoRollHeight(Length),

    /// Scrolls the overview to the left by one cell
    ScrollLeft,
    /// Scrolls the overview to the right by one cell
    ScrollRight,

    /// Stop playing
    Pause,
    /// Start playing
    Play,
    /// `Play` or `Pause`
    PlayPause,

    /// Takes a project action
    Project(project::Action),

    /// Sets the audio output device
    SetDevice(#[debug(ignore)] Device),

    /// Saves and exits the program
    Exit,
    // TODO: add scripting
}

impl Action {
    /// Returns an action for importing audio
    pub fn import_audio<P: Into<PathBuf>>(file: P) -> Action {
        Action::Project(project::Action::ImportAudio { file: file.into() })
    }

    /// Take the action on the app
    pub fn take(self, app: &App) {
        match self {
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

            Action::ScrollLeft => {
                app.overview_offset
                    .set(app.overview_offset.get() - app.grid.cell_width.get());
            }
            Action::ScrollRight => {
                app.overview_offset
                    .set(app.overview_offset.get() + app.grid.cell_width.get());
            }

            Action::OpenPianoRoll => {
                Action::SetPianoRollHeight(app.last_size.get().height * Ratio::HALF).take(app);
            }
            Action::SetPianoRollHeight(height) => {
                let mut settings = app.piano_roll_settings.get();
                settings.height = height;
                app.piano_roll_settings.set(settings);
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
                        .take(action, app.cursor.get(), app.selected_track_index.get());

                if let Err(popup) = result {
                    app.popups.open(popup);
                }
            }
            Action::SelectTrack(index) => {
                app.selected_track_index.set(index);
            }
            Action::SelectClip { track_index, index } => {
                app.selected_track_index.set(track_index);
                app.selected_clip_index.set(index);
            }
            Action::SetDevice(device) => {
                app.device.set(Some(device));
            }
        }
    }
}
