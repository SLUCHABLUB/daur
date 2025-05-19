mod action;
mod actions;
mod holdable;
mod selection;
mod view;

pub use action::Action;
pub use actions::Actions;
pub use holdable::HoldableObject;
pub use selection::Selection;

use crate::app::view::view;
use crate::audio::Config;
use crate::metre::{Instant, NonZeroDuration};
use crate::ui::{Grid, Length, NonZeroLength, Offset};
use crate::view::context::MenuInstance;
use crate::{PianoRoll, Ratio, UserInterface, View, popup, project};
use derive_more::Debug;
use getset::{CloneGetters, CopyGetters, Getters, MutGetters};

/// A running instance of the DAW.
#[derive(Debug, Getters, MutGetters, CopyGetters, CloneGetters)]
pub struct App<Ui: UserInterface> {
    /// The user interface used by the app.
    #[get = "pub"]
    #[get_mut = "pub"]
    ui: Ui,

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

    popup_manager: popup::Manager,
    #[get_clone = "pub(crate)"]
    context_menu: Option<MenuInstance>,
    /// The currently held object.
    #[get_copy = "pub"]
    held_object: Option<HoldableObject>,

    // TODO: move to temporary settings
    /// The height of the project bar.
    project_bar_height: NonZeroLength,
    track_settings_width: NonZeroLength,

    selection: Selection,

    /// The position of the musical cursor.
    ///
    /// If audio is playing, this may not reflect the actual position,
    /// but the position of the cursor at the time when audio playback started.
    cursor: Instant,

    // TODO: move to temporary settings
    /// Whether _edit mode_ is enabled.
    edit_mode: bool,
    /// The settings for the overview grid.
    // TODO: move to temporary settings
    grid: Grid,
    // TODO: move to temporary settings
    /// How far to the left the overview has been moved.
    negative_overview_offset: Length,
    // TODO: move to temporary settings
    /// The settings regarding the piano roll.
    piano_roll: PianoRoll,
}

impl<Ui: UserInterface> App<Ui> {
    /// Creates a new instance
    #[must_use]
    pub fn new(ui: Ui) -> App<Ui> {
        let height = ui.size().height;

        let mut app = App {
            ui,
            view: View::Empty,

            project_manager: project::Manager::default(),
            renderer: project::Renderer::default(),

            audio_config: Config::default(),

            popup_manager: popup::Manager::new(),
            context_menu: None,
            held_object: None,

            project_bar_height: Ui::PROJECT_BAR_HEIGHT,
            track_settings_width: Ui::TRACK_SETTINGS_WITH,

            selection: Selection::default(),
            cursor: Instant::START,

            edit_mode: false,
            grid: Grid {
                cell_duration: NonZeroDuration::QUARTER,
                cell_width: Ui::CELL_WIDTH,
            },
            negative_overview_offset: Length::ZERO,
            piano_roll: PianoRoll {
                negative_x_offset: Length::ZERO,
                y_offset: Offset::ZERO,
                content_height: height * Ratio::HALF,
                is_open: false,
                key_width: Ui::KEY_WIDTH,
                piano_depth: Ui::PIANO_DEPTH,
                black_key_depth: Ui::BLACK_KEY_DEPTH,
            },
        };

        app.rerender();

        app
    }

    /// Returns the position of the musical cursor.
    fn cursor(&self) -> Instant {
        if let Some(position) = self.audio_config.player_position() {
            position.to_metre(self.project_manager.project().settings())
        } else {
            self.cursor
        }
    }

    fn rerender(&mut self) {
        self.view = view(self);
    }
}
