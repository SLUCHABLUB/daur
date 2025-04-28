use crate::app::HoldableObject;
use crate::popup::{Id, Popup};
use crate::time::Instant;
use crate::ui::Point;
use crate::view::context::Menu;
use crate::{App, Clip, Track, UserInterface, project};
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

    /// Sets the piano roll's height to half of the screen height.
    TogglePianoRoll,

    /// Picks up an object.
    PickUp(HoldableObject),
    /// Moves the held object.
    MoveHand(Point),
    /// Lets go of the held object.
    LetGo,

    // TODO: use Length
    /// Scrolls the overview to the left by one cell
    ScrollLeft,
    // TODO: use Length
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
}

impl<Ui: UserInterface> App<Ui> {
    /// Takes an action on the app.
    pub fn take_action(&self, action: Action) {
        let should_rerender = self.take_(action);

        if should_rerender {
            self.ui.rerender();
        }
    }

    /// Takes multiple actions on the app.
    pub fn take_actions<Actions: IntoIterator<Item = Action>>(&self, actions: Actions) {
        let mut should_rerender = false;

        for action in actions {
            should_rerender |= self.take_(action);
        }

        if should_rerender {
            self.ui.rerender();
        }
    }

    /// Takes a single action on the app and return whether the app needs to be rerendered.
    fn take_(&self, action: Action) -> bool {
        let mut need_rerender = true;

        match action {
            Action::OpenPopup(popup) => {
                self.popups.open(&popup, &self.ui);
            }
            Action::ClosePopup(popup) => {
                self.popups.close(popup);
            }
            Action::OpenContextMenu { menu, position } => {
                self.context_menu
                    .set(Some(menu.instantiate::<Ui>(position)));
            }
            Action::CloseContextMenu => {
                self.context_menu.set(None);
            }

            Action::Exit => self.ui.exit(),
            Action::MoveCursor(instant) => {
                self.cursor.set(instant);

                if self.is_playing() {
                    self.start_playback();
                }
            }

            Action::ScrollLeft => {
                self.overview_offset
                    .set(self.overview_offset.get() - self.grid.cell_width.get());
            }
            Action::ScrollRight => {
                self.overview_offset
                    .set(self.overview_offset.get() + self.grid.cell_width.get());
            }

            Action::TogglePianoRoll => {
                let mut settings = self.piano_roll_settings.get();
                settings.open = !settings.open;
                self.piano_roll_settings.set(settings);
            }

            Action::PickUp(object) => {
                if let Some(old) = self.hand.replace(Some(object)) {
                    old.let_go(self);
                }
                need_rerender = false;
            }
            Action::MoveHand(point) => match self.hand.get() {
                Some(object) => object.update(self, point),
                None => need_rerender = false,
            },
            Action::LetGo => match self.hand.take() {
                Some(object) => object.let_go(self),
                None => need_rerender = false,
            },

            Action::Pause => {
                self.stop_playback();
            }
            Action::Play => {
                self.start_playback();
            }
            Action::PlayPause => {
                if self.is_playing() {
                    self.stop_playback();
                } else {
                    self.start_playback();
                }
            }
            Action::Project(action) => {
                let result =
                    self.project
                        .take(action, self.cursor.get(), self.selected_track.get());

                if let Err(popup) = result {
                    self.popups.open(&popup, &self.ui);
                }
            }
            Action::SelectClip { track, clip, .. } => {
                self.selected_track.set(track);
                self.selected_clip.set(clip);
            }
            Action::SetDevice(device) => {
                self.device.set_value(Some(device));
                need_rerender = false;
            }
        }

        need_rerender
    }
}
