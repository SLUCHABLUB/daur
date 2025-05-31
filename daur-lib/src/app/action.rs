use crate::app::Actions;
use crate::metre::Instant;
use crate::popup::Specification;
use crate::project::track::Clip;
use crate::project::{Track, track};
use crate::ui::{Length, Point, Vector};
use crate::view::context::Menu;
use crate::{App, HoldableObject, Id, Note, Popup, UserInterface, project};
use anyhow::Result;
use derive_more::Debug;
use std::path::PathBuf;

/// An action to take on the app
#[derive(Clone, Debug)]
#[must_use = "actions are lazy and must be \"taken\""]
#[remain::sorted]
pub enum Action {
    /// Opens the context menu.
    CloseContextMenu,
    /// Closes a popup.
    ClosePopup(Id<Popup>),
    /// Enters _edit mode_.
    EnterEditMode,
    /// Saves and exits the program
    Exit,
    /// Exits _edit mode_.
    ExitEditMode,
    /// Removes the held object.
    LetGo,
    /// Moves the (musical) cursor.
    MoveCursor(Instant),
    /// Moves the held object.
    MoveHeldObject(Point),
    /// Moves the overview.
    MoveOverview(Vector),
    /// Moves the piano roll.
    MovePianoRoll(Vector),
    /// Opens a context menu.
    OpenContextMenu {
        /// The context menu to open.
        menu: Menu,
        /// The position at which to open the context menu.
        /// (The mouse position.)
        position: Point,
    },
    /// Opens a popup.
    OpenPopup(Specification),
    /// Stop playing.
    Pause,
    /// Picks up an object.
    PickUp(HoldableObject),
    /// Start playing.
    Play,
    /// A project action.
    Project(project::Action),
    /// Selects a clip in a track.
    SelectClip {
        /// The id of the track in which the clip resides.
        track: Id<Track>,
        /// The id of the clip to select.
        clip: Id<Clip>,
    },
    /// Selects a note.
    SelectNote {
        /// The id of the track in which the clip resides.
        track: Id<Track>,
        /// The id of the clip in which the note resides.
        clip: Id<Clip>,
        /// The id of the clip to select.
        note: Id<Note>,
    },
    /// Selects a track.
    SelectTrack(Id<Track>),
    /// Toggles _edit mode_.
    ToggleEditMode,
    /// Sets the piano roll's height to half of the screen height.
    TogglePianoRoll,
    /// Toggles whether the app is playing.
    TogglePlayback,
    // TODO: add scripting
}

impl Action {
    /// Returns an action for importing audio
    pub fn import_audio<P: Into<PathBuf>>(file: P) -> Action {
        Action::Project(project::Action::Track(track::Action::ImportAudio {
            file: file.into(),
        }))
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

    #[remain::check]
    fn try_take(&mut self, action: Action) -> Result<()> {
        #[sorted]
        match action {
            Action::CloseContextMenu => {
                self.context_menu = None;
            }
            Action::ClosePopup(popup) => {
                self.popup_manager.close(popup);
            }
            Action::EnterEditMode => self.edit_mode = true,
            Action::Exit => self.ui.exit(),
            Action::ExitEditMode => self.edit_mode = false,
            Action::LetGo => self.held_object = None,
            Action::MoveCursor(instant) => {
                self.cursor = instant;

                if self.audio_config.is_player_playing() {
                    self.take(Action::Play);
                } else {
                    self.audio_config.pause_player();
                }
            }
            Action::MoveHeldObject(to) => {
                let Some(object) = self.held_object else {
                    return Ok(());
                };

                match object {
                    HoldableObject::PianoRollHandle { y } => {
                        self.piano_roll
                            .set_content_height(self.ui.size().height - to.y + y - Length::PIXEL);
                    }
                    // These are processed when they are dropped.
                    HoldableObject::NoteCreation { .. } | HoldableObject::SelectionBox { .. } => (),
                }
            }
            Action::MoveOverview(by) => {
                self.ui_settings.negative_overview_offset -= by.x;
                // TODO: scroll tracks vertically
            }
            Action::MovePianoRoll(by) => {
                self.piano_roll.move_by::<Ui>(by);
            }
            Action::OpenContextMenu { menu, position } => {
                self.context_menu = Some(menu.instantiate::<Ui>(position, self.ui()));
            }
            Action::OpenPopup(popup) => {
                self.popup_manager.open(&popup, &self.ui);
            }
            Action::Pause => {
                if let Some(position) = self.audio_config.pause_player() {
                    self.cursor = position / &self.project_manager.project().time_context();
                }
            }
            // the currently held object should already have been let go.
            Action::PickUp(object) => self.held_object = Some(object),
            Action::Play => {
                let from = self.cursor() * &self.project_manager.project().time_context();

                let player = self.audio_config.player()?;

                self.renderer.play_when_finished(from, player);
            }
            Action::Project(action) => {
                self.project_manager
                    .take_action(action, self.cursor(), &mut self.selection)?;

                self.renderer.restart(
                    self.project_manager.project(),
                    self.audio_config.sample_rate()?,
                );
            }
            Action::SelectClip { track, clip, .. } => {
                self.selection.set_track(track);
                self.selection.set_clip(clip);
            }
            Action::SelectNote { .. } => {
                // TODO: select the note
            }
            Action::SelectTrack(track) => {
                self.selection.set_track(track);
            }
            Action::ToggleEditMode => self.edit_mode = !self.edit_mode,
            Action::TogglePianoRoll => {
                self.piano_roll.set_is_open(!self.piano_roll.is_open());
            }
            Action::TogglePlayback => {
                if self.audio_config.is_player_playing() {
                    self.take(Action::Pause);
                } else {
                    self.take(Action::Play);
                }
            }
        }

        Ok(())
    }
}
