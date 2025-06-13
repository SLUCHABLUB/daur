use crate::app::Actions;
use crate::metre::Instant;
use crate::popup::Specification;
use crate::project::Edit;
use crate::ui::{Length, Point, Rectangle, Vector};
use crate::view::context::Menu;
use crate::{App, Holdable, Id, Popup, Selectable, UserInterface};
use anyhow::Result;
use derive_more::Debug;
use std::env::current_dir;

const DEFAULT_EXPORT_FILE_NAME: &str = "render";

/// An action to take on the app
#[derive(Clone, Debug)]
#[must_use = "actions are lazy and must be \"taken\""]
#[remain::sorted]
pub enum Action {
    /// Clears the selection.
    ClearSelection,
    /// Opens the context menu.
    CloseContextMenu,
    /// Closes a popup.
    ClosePopup(Id<Popup>),
    /// A project edit.
    Edit(Edit),
    /// Enters _edit mode_.
    EnterEditMode,
    /// Saves and exits the program
    Exit,
    /// Exits _edit mode_.
    ExitEditMode,
    /// Renders and exports the project to a file.
    ExportProject,
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
    PickUp(Holdable),
    /// Start playing.
    Play,
    /// Selects an item.
    Select(Selectable),
    /// Toggles _edit mode_.
    ToggleEditMode,
    /// Sets the piano roll's height to half of the screen height.
    TogglePianoRoll,
    /// Toggles whether the app is playing.
    TogglePlayback,
    // TODO: add scripting
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
            self.popup_manager.open(&Specification::from(error));
        }
    }

    #[remain::check]
    fn try_take(&mut self, action: Action) -> Result<()> {
        #[sorted]
        match action {
            Action::ClearSelection => {
                self.selection.clear();
            }
            Action::CloseContextMenu => {
                self.context_menu = None;
            }
            Action::ClosePopup(popup) => {
                self.popup_manager.close(popup);
            }
            Action::Edit(edit) => {
                self.project_manager
                    .edit(edit, self.cursor(), &mut self.selection)?;

                self.renderer.restart(
                    self.project_manager.project(),
                    self.audio_config.sample_rate()?,
                )?;
            }
            Action::EnterEditMode => self.edit_mode = true,
            Action::Exit => self.ui.exit(),
            Action::ExitEditMode => self.edit_mode = false,
            Action::ExportProject => {
                // TODO: read the project path from the app
                let project_directory = current_dir()?;

                let project_name = self.project_manager.project().name();

                let file_name = if project_name.is_empty() {
                    DEFAULT_EXPORT_FILE_NAME
                } else {
                    &project_name
                };

                let mut export_file = project_directory.join(file_name);

                // Currently only wav files are supported for export since symphonia only does decoding.
                export_file.set_extension("wav");

                self.renderer.export_when_finished(export_file)?;
            }
            Action::LetGo => self.held_object = None,
            Action::MoveCursor(instant) => {
                self.cursor = instant;

                if self.audio_config.is_player_playing() {
                    self.take(Action::Play);
                } else {
                    self.audio_config.pause_player();
                }
            }
            Action::MoveHeldObject(to) => self.move_held_object(to),
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
                self.popup_manager.open(&popup);
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
            Action::Select(item) => {
                self.selection.push(item);
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

    fn move_held_object(&mut self, to: Point) {
        let Some(object) = self.held_object else {
            return;
        };

        match object {
            Holdable::PianoRollHandle { y } => {
                self.piano_roll
                    .set_content_height(self.ui.size().height - to.y + y - Length::PIXEL);
            }
            Holdable::Popup { id, point } => {
                self.popup_manager.transform_popup(id, |area| Rectangle {
                    position: to - point,
                    size: area.size,
                });
            }
            Holdable::PopupSide { popup, side } => self
                .popup_manager
                .transform_popup(popup, |area| side.resize(area, to)),
            // These are processed when they are dropped.
            Holdable::Clip(_) | Holdable::NoteCreation { .. } | Holdable::SelectionBox { .. } => (),
        }
    }
}
