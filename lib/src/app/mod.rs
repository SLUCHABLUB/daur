//! Items pertaining to [`App`].

mod action;
mod actions;
mod view;

pub use action::Action;
pub use actions::Actions;
use std::sync::Arc;

use crate::Holdable;
use crate::PianoRoll;
use crate::UserInterface;
use crate::View;
use crate::app::view::view;
use crate::audio::Config;
use crate::metre::Instant;
use crate::metre::NonZeroDuration;
use crate::metre::Quantisation;
use crate::popup;
use crate::project;
use crate::select::Selection;
use crate::ui;
use crate::ui::Theme;
use crate::view::context::MenuInstance;
use derive_more::Debug;
use getset::CloneGetters;
use getset::CopyGetters;
use getset::Getters;
use getset::MutGetters;

/// A running instance of the DAW.
#[derive(Debug, Getters, MutGetters, CopyGetters, CloneGetters)]
pub struct App<Ui: UserInterface> {
    /// The user interface used by the app.
    #[get_copy = "pub"]
    ui: &'static Ui,
    /// Settings for the user interface.
    ui_settings: ui::Settings,

    /// The view of the app.
    ///
    /// This includes popups and the context menu.
    /// The view may need to be reacquired if an action is taken on the app.
    #[get = "pub"]
    view: View,

    /// The project manager (tracks history).
    project_manager: project::Manager,
    /// The project renderer.
    #[debug(skip)]
    renderer: project::Renderer,

    /// The audio configuration (volatile audio settings).
    #[debug(skip)]
    audio_config: Config,
    /// The selected colour theme.
    #[get_copy = "pub"]
    theme: Theme,

    /// The currently open context menu.
    #[get_clone = "pub(crate)"]
    context_menu: Option<MenuInstance>,
    /// The currently held object.
    #[get_copy = "pub"]
    held_object: Option<Holdable>,
    /// The popup manage (allows synchronised opening of popups).
    popup_manager: Arc<popup::Manager>,

    /// The position of the musical cursor.
    ///
    /// If audio is playing, this may not reflect the actual position,
    /// but the position of the cursor at the time when audio playback started.
    cursor: Instant,
    /// The selection (what clips, tracks, notes, &c. are selected).
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
    /// Creates a new instance.
    #[must_use]
    pub fn new(ui: &'static Ui) -> Self {
        let popup_manager = Arc::new(popup::Manager::new());

        let mut app = App {
            ui,
            ui_settings: ui::Settings::default_in::<Ui>(),

            view: View::Empty,

            project_manager: project::Manager::default(),
            renderer: project::Renderer::new(Arc::clone(&popup_manager)),

            audio_config: Config::default(),

            popup_manager,
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
            piano_roll: PianoRoll::default_in::<Ui>(),
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

    /// Rerenders the cached view.
    fn rerender(&mut self) {
        self.view = view(self);
    }
}
