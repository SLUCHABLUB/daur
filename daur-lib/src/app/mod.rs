//! Types pertaining to [`App`].

mod action;
mod actions;
mod view;

pub use action::Action;
pub use actions::Actions;

use crate::app::view::view;
use crate::audio::Config;
use crate::metre::{Instant, NonZeroDuration, Quantisation};
use crate::select::Selection;
use crate::ui::Theme;
use crate::view::context::MenuInstance;
use crate::{Holdable, PianoRoll, UserInterface, View, popup, project, ui};
use derive_more::Debug;
use getset::{CloneGetters, CopyGetters, Getters, MutGetters};

/// A running instance of the DAW.
#[cfg_attr(doc, doc(hidden))]
#[derive(Debug, Getters, MutGetters, CopyGetters, CloneGetters)]
pub struct App<Ui: UserInterface> {
    /// The user interface used by the app.
    #[get = "pub"]
    #[get_mut = "pub"]
    ui: Ui,
    ui_settings: ui::Settings,

    /// The view of the app.
    ///
    /// This includes popups and the context menu.
    /// The view may need to be reacquired if an action is taken on the app.
    #[get = "pub"]
    view: View,

    project_manager: project::Manager,
    #[debug(skip)]
    renderer: project::Renderer,

    #[debug(skip)]
    audio_config: Config,
    /// The colour theme.
    #[get_copy = "pub"]
    theme: Theme,

    #[get_clone = "pub(crate)"]
    context_menu: Option<MenuInstance>,
    /// The currently held object.
    #[get_copy = "pub"]
    held_object: Option<Holdable>,
    popup_manager: popup::Manager,

    /// The position of the musical cursor.
    ///
    /// If audio is playing, this may not reflect the actual position,
    /// but the position of the cursor at the time when audio playback started.
    cursor: Instant,
    selection: Selection,
    /// The settings quantisation.
    quantisation: Quantisation,

    /// Whether _edit mode_ is enabled.
    edit_mode: bool,
    /// The settings regarding the piano roll.
    #[get_mut = "pub(crate)"]
    piano_roll: PianoRoll,
}

impl<Ui: UserInterface> App<Ui> {
    /// Creates a new instance
    #[must_use]
    pub fn new(ui: Ui) -> App<Ui> {
        let mut app = App {
            ui,
            ui_settings: ui::Settings::default_in::<Ui>(),

            view: View::Empty,

            project_manager: project::Manager::default(),
            renderer: project::Renderer::new(),

            audio_config: Config::default(),

            popup_manager: popup::Manager::new(),
            context_menu: None,
            held_object: None,

            // TODO: load from file
            theme: Theme::default(),

            selection: Selection::default(),
            cursor: Instant::START,

            edit_mode: false,
            quantisation: Quantisation {
                cell_duration: NonZeroDuration::QUARTER,
                cell_width: Ui::CELL_WIDTH,
            },
            piano_roll: PianoRoll::new_in::<Ui>(),
        };

        app.rerender();

        app
    }

    /// Returns the position of the musical cursor.
    fn cursor(&self) -> Instant {
        if let Some(position) = self.audio_config.player_position() {
            position / &self.project_manager.project().time_context()
        } else {
            self.cursor
        }
    }

    fn rerender(&mut self) {
        self.view = view(self);
    }
}
