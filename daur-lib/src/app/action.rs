use crate::popup::{Id, Popup};
use crate::time::Instant;
use crate::ui::{Length, NonZeroLength, Point};
use crate::view::context::Menu;
use crate::{App, Clip, Ratio, Track, UserInterface, project};
use derive_more::Debug;
use rodio::Device;
use std::path::PathBuf;
use std::sync::Weak;

/// An action to take on the app
#[derive(Clone, Debug)]
pub enum Action {
    /// Opens a popup.
    OpenPopup(Popup),
    /// Closes a popup.
    ClosePopup(Id),
    /// Opens a context menu.
    OpenContextMenu {
        /// The context menu to open.
        menu: Menu,
        /// The position at which to open the context menu.
        /// (The mouse position.)
        position: Point,
    },
    /// Opens the context menu.
    CloseContextMenu,

    /// Moves the (musical) cursor.
    MoveCursor(Instant),
    /// Selects a clip and track
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
            Action::OpenPopup(popup) => {
                app.popups.open(&popup, &app.ui);
            }
            Action::ClosePopup(popup) => {
                app.popups.close(popup);
            }
            Action::OpenContextMenu { menu, position } => {
                app.context_menu.set(Some(menu.instantiate::<Ui>(position)));
            }
            Action::CloseContextMenu => {
                app.context_menu.set(None);
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
                Action::SetPianoRollHeight(app.ui.size().height * Ratio::HALF).take(app);
            }
            Action::SetPianoRollHeight(height) => {
                let mut settings = app.piano_roll_settings.get();
                settings.height = NonZeroLength::from_length(height);
                app.piano_roll_settings.set(settings);
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
