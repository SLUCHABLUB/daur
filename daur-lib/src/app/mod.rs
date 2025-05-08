mod action;
mod holdable;

pub use action::Action;
pub use holdable::HoldableObject;

use crate::audio::Config;
use crate::time::{Instant, NonZeroDuration};
use crate::ui::{Grid, Length, NonZeroLength, Offset};
use crate::view::context::MenuInstance;
use crate::view::piano_roll::Settings;
use crate::view::{ToText as _, View, piano_roll};
use crate::{Clip, Project, Ratio, Track, UserInterface, popup, project, ui};
use arcstr::{ArcStr, literal};
use derive_more::Debug;
use getset::{CloneGetters, Getters, MutGetters};
use std::sync::Weak;

const SPLASH: ArcStr = literal!("DAUR - A DAW");

// TODO: remove internal mutability
/// A running instance of the DAW.
#[derive(Debug, Getters, MutGetters, CloneGetters)]
pub struct App<Ui: UserInterface> {
    /// The user interface used by the app.
    // TODO: remove getter
    #[getset(get = "pub", get_mut = "pub")]
    ui: Ui,

    /// The view of the app.
    ///
    /// This includes popups and the context menu.
    /// The view may need to be reacquired if an action is taken on the app.
    #[getset(get = "pub")]
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
    hand: Option<HoldableObject>,

    // TODO: move to temporary settings
    /// The height of the project bar.
    project_bar_height: NonZeroLength,
    track_settings_width: NonZeroLength,

    #[get_clone = "pub(crate)"]
    selected_track: Weak<Track>,
    #[get_clone = "pub(crate)"]
    selected_clip: Weak<Clip>,

    /// The position of the musical cursor.
    ///
    /// If audio is playing, this may not reflect the actual position,
    /// but the position of the cursor at the time when audio playback started.
    cursor: Instant,

    /// The settings for the overview grid.
    // TODO: move to temporary settings
    grid: Grid,
    // TODO: move to temporary settings
    /// How far to the left the overview has been moved.
    overview_offset: Length,
    // TODO: move to temporary settings
    /// The settings regarding the piano roll.
    piano_roll_settings: Settings,
}

impl<Ui: UserInterface> App<Ui> {
    /// Creates a new instance
    #[must_use]
    pub fn new(ui: Ui) -> App<Ui> {
        let height = ui.size().height;

        App {
            ui,
            view: SPLASH.centred(),

            project_manager: project::Manager::new(Project::default()),
            renderer: project::Renderer::default(),

            audio_config: Config::default(),

            popup_manager: popup::Manager::new(),
            context_menu: None,
            hand: None,

            project_bar_height: Ui::PROJECT_BAR_HEIGHT,
            track_settings_width: Ui::TRACK_SETTINGS_WITH,

            selected_track: Weak::new(),
            selected_clip: Weak::new(),
            cursor: Instant::START,

            grid: Grid {
                cell_duration: NonZeroDuration::QUARTER,
                cell_width: Ui::CELL_WIDTH,
            },
            overview_offset: Length::ZERO,
            piano_roll_settings: Settings {
                x_offset: Length::ZERO,
                y_offset: Offset::ZERO,
                content_height: height * Ratio::HALF,
                open: false,
                key_width: Ui::KEY_WIDTH,
                piano_depth: Ui::PIANO_DEPTH,
                black_key_depth: Ui::BLACK_KEY_DEPTH,
            },
        }
    }

    /// Returns the position of the musical cursor.
    fn cursor(&self) -> Instant {
        if let Some(position) = self.audio_config.player_position() {
            self.project_manager
                .project()
                .time_mapping()
                .musical(position)
        } else {
            self.cursor
        }
    }

    /// The main view of the app behind any popups
    fn main_view(&self) -> View {
        View::y_stack([
            self.project_manager
                .project()
                .bar::<Ui>(self.audio_config.is_player_playing())
                .quotated(self.project_bar_height.get()),
            self.project_manager
                .project()
                .workspace::<Ui>(
                    self.track_settings_width,
                    self.grid,
                    self.overview_offset,
                    &self.selected_track,
                    &self.selected_clip,
                    self.cursor(),
                    self.audio_config.try_player(),
                )
                .fill_remaining(),
            piano_roll::view::<Ui>(
                &self.selected_clip,
                ui::Mapping {
                    time_signature: self.project_manager.project().time_signature(),
                    grid: self.grid,
                },
                self.piano_roll_settings,
                &self.project_manager.project().key(),
            ),
        ])
    }

    fn render_view(&self) -> View {
        let mut layers = vec![self.main_view()];

        for instance in self.popup_manager.popups() {
            layers.push(instance.view());
        }

        if let Some(instance) = self.context_menu() {
            layers.push(instance.into_view());
        }

        View::Layers(layers)
    }
}
