use crate::app::HoldableObject;
use crate::metre::Instant;
use crate::popup::Specification;
use crate::project::Track;
use crate::project::track::Clip;
use crate::ui::{Point, Vector};
use crate::view::context::Menu;
use crate::{Actions, App, Id, Popup, UserInterface, project};
use anyhow::Result;
use derive_more::Debug;
use std::path::PathBuf;

/// An action to take on the app
#[derive(Clone, Debug)]
pub enum Action {
    /// Opens a popup.
    OpenPopup(Specification),
    /// Closes a popup.
    ClosePopup(Id<Popup>),
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
    /// Selects a track.
    SelectTrack(Id<Track>),
    /// Selects a clip in a track.
    SelectClip {
        /// The index of the track in which the clip resides
        track: Id<Track>,
        /// The index of the clip to select
        clip: Id<Clip>,
    },

    /// Sets the piano roll's height to half of the screen height.
    TogglePianoRoll,

    /// Enters _edit mode_.
    EnterEditMode,
    /// Exits _edit mode_.
    ExitEditMode,
    /// Toggles _edit mode_.
    ToggleEditMode,

    /// Picks up an object.
    PickUp(HoldableObject),
    /// Moves the held object.
    MoveHeldObject(Point),

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
    pub fn take_action(&mut self, action: Action) {
        self.take(action);
        self.rerender();
    }

    /// Takes multiple actions on the app.
    pub fn take_actions(&mut self, actions: Actions) {
        let should_rerender = !actions.is_empty();

        for action in actions.into_vec() {
            self.take(action);
        }

        if should_rerender {
            self.rerender();
        }
    }

    fn take(&mut self, action: Action) {
        if let Err(error) = self.try_take(action) {
            self.popup_manager
                .open(&Specification::from(error), &self.ui);
        }
    }

    fn try_take(&mut self, action: Action) -> Result<()> {
        match action {
            Action::OpenPopup(popup) => {
                self.popup_manager.open(&popup, &self.ui);
            }
            Action::ClosePopup(popup) => {
                self.popup_manager.close(popup);
            }
            Action::OpenContextMenu { menu, position } => {
                self.context_menu = Some(menu.instantiate::<Ui>(position));
            }
            Action::CloseContextMenu => {
                self.context_menu = None;
            }

            Action::Exit => self.ui.exit(),
            Action::MoveCursor(instant) => {
                self.cursor = instant;

                if self.audio_config.is_player_playing() {
                    self.take(Action::Play);
                } else {
                    self.audio_config.pause_player();
                }
            }

            Action::MoveOverview(by) => {
                self.negative_overview_offset -= by.x;
                // TODO: scroll tracks vertically
            }
            Action::MovePianoRoll(by) => {
                self.piano_roll.negative_x_offset -= by.x;
                self.piano_roll.y_offset += by.y;
            }

            Action::TogglePianoRoll => {
                self.piano_roll.is_open = !self.piano_roll.is_open;
            }

            Action::EnterEditMode => self.edit_mode = true,
            Action::ExitEditMode => self.edit_mode = false,
            Action::ToggleEditMode => self.edit_mode = !self.edit_mode,

            // the currently held object should already have been let go.
            Action::PickUp(object) => self.held_object = Some(object),
            Action::MoveHeldObject(point) => {
                if let Some(object) = self.held_object {
                    object.update(self, point);
                }
            }

            Action::Pause => {
                if let Some(position) = self.audio_config.pause_player() {
                    self.cursor = position.to_metre(self.project_manager.project().settings());
                }
            }
            Action::Play => {
                let from = self
                    .cursor()
                    .to_real_time(self.project_manager.project().settings());

                let player = self.audio_config.player()?;

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
                self.project_manager
                    .take(action, self.cursor, self.selection)?;

                self.renderer.restart(
                    self.project_manager.project(),
                    self.project_manager.project().settings(),
                    self.audio_config.sample_rate()?,
                );
            }
            Action::SelectTrack(track) => {
                self.selection.set_track(track);
            }
            Action::SelectClip { track, clip, .. } => {
                self.selection.set_track(track);
                self.selection.set_clip(clip);
            }
        }

        Ok(())
    }
}
