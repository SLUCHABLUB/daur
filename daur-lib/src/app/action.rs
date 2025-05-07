use crate::app::HoldableObject;
use crate::popup::{Id, Popup};
use crate::time::Instant;
use crate::ui::{Point, Vector};
use crate::view::context::Menu;
use crate::{App, Clip, Track, UserInterface, project};
use derive_more::Debug;
use std::iter::once;
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

    /// Moves the overview.
    MoveOverview(Vector),

    /// Moves the piano roll.
    MovePianoRoll(Vector),

    /// Stop playing
    Pause,
    /// Start playing
    Play,
    /// Toggles whether the app is playing.
    TogglePlayback,

    /// Takes a project action
    Project(project::Action),

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
        self.take_actions(once(action));
    }

    /// Takes multiple actions on the app.
    pub fn take_actions<Actions: IntoIterator<Item = Action>>(&self, actions: Actions) {
        let mut should_rerender = false;

        for action in actions {
            self.take(action);
            should_rerender = true;
        }

        if should_rerender {
            self.ui.rerender();
        }
    }

    /// Takes a single action on the app and return whether the app needs to be rerendered.
    fn take(&self, action: Action) {
        // alternate spelling of "try"
        macro_rules! trie {
            ($result:expr) => {
                match $result {
                    Ok(ok) => ok,
                    Err(error) => {
                        self.popups.open(&Popup::from(error), &self.ui);
                        return;
                    }
                }
            };
        }

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

                if self.audio_config.is_player_playing() {
                    self.take(Action::Play);
                }
            }

            Action::MoveOverview(by) => {
                self.overview_offset.set(self.overview_offset.get() - by.x);
                // TODO: scroll tracks vertically
            }
            Action::MovePianoRoll(by) => {
                let mut settings = self.piano_roll_settings.get();

                settings.x_offset -= by.x;
                settings.y_offset += by.y;

                self.piano_roll_settings.set(settings);
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
            }
            Action::MoveHand(point) => {
                if let Some(object) = self.hand.get() {
                    object.update(self, point);
                }
            }
            Action::LetGo => {
                if let Some(object) = self.hand.take() {
                    object.let_go(self);
                }
            }

            Action::Pause => {
                if let Some(position) = self.audio_config.pause_player() {
                    let position = self.project.time_mapping().musical(position);
                    self.cursor.set(position);
                }
            }
            Action::Play => {
                let cursor = self.cursor.get();
                let from = self.project.time_mapping().real_time(cursor);

                let player = trie!(self.audio_config.player());

                self.renderer.play_when_finished(from, player);
            }
            Action::TogglePlayback => {
                if self.audio_config.is_player_playing() {
                    self.take(Action::Pause);
                } else {
                    self.take(Action::Play);
                }
            }
            Action::Project(action) => {
                let result =
                    self.project
                        .take(action, self.cursor.get(), self.selected_track.get());

                self.renderer.restart(
                    &self.project.tracks(),
                    &self.project.time_mapping(),
                    trie!(self.audio_config.sample_rate()),
                );

                trie!(result);
            }
            Action::SelectClip { track, clip, .. } => {
                self.selected_track.set(track);
                self.selected_clip.set(clip);
            }
        }
    }
}
