use crate::popup::{Id, Popup};
use crate::time::Instant;
use crate::ui::{Length, NonZeroLength};
use crate::{App, Clip, Track, UserInterface, project};
use derive_more::Debug;
use rodio::Device;
use std::path::PathBuf;
use std::sync::Weak;

/// An action to take on the app
#[derive(Clone, Debug)]
pub enum Action {
    /// Opens the popup
    OpenPopup(Popup),
    /// Closes the popup
    ClosePopup(Id),

    /// Moves the (musical) cursor.
    MoveCursor(Instant),
    /// Selects the track with the given index
    SelectTrack(Weak<Track>),
    /// Selects a clip
    SelectClip {
        /// The index of the track in which the clip resides
        track: Weak<Track>,
        /// The index of the clip to select
        clip: Weak<Clip>,
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
    /// Inverts the playback state.
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
    pub fn take<Ui: UserInterface>(self, app: &App<Ui>) {
        match self {
            Action::ClosePopup(popup) => {
                app.popups.close(popup);
            }
            Action::Exit => app.ui.exit(),
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
                // TODO: do this more cleanly
                Action::SetPianoRollHeight(Ui::PROJECT_BAR_HEIGHT.get()).take(app);
            }
            Action::SetPianoRollHeight(height) => {
                let mut settings = app.piano_roll_settings.get();
                settings.height = NonZeroLength::from_length(height);
                app.piano_roll_settings.set(settings);
            }

            Action::OpenPopup(popup) => {
                app.popups.open(&popup, &app.ui);
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
                let result = app
                    .project
                    .take(action, app.cursor.get(), app.selected_track.get());

                if let Err(popup) = result {
                    app.popups.open(&popup, &app.ui);
                }
            }
            Action::SelectTrack(index) => {
                app.selected_track.set(index);
            }
            Action::SelectClip { track, clip, .. } => {
                app.selected_track.set(track);
                app.selected_clip.set(clip);
            }
            Action::SetDevice(device) => {
                app.device.set_value(Some(device));
            }
        }
    }
}
