mod action;
mod holdable;

pub use action::Action;
pub use holdable::HoldableObject;

use crate::audio::Config;
use crate::time::{Instant, NonZeroDuration};
use crate::ui::{Grid, Length, NonZeroLength, Offset};
use crate::view::context::MenuInstance;
use crate::view::piano_roll::Settings;
use crate::view::{View, piano_roll};
use crate::{
    Cell, Clip, CloneCell, Project, Ratio, Track, UserInterface, WeakCell, popup, project, ui,
};
use derive_more::Debug;
use getset::Getters;
use std::sync::Weak;

// TODO: remove internal mutability
/// A running instance of the DAW.
#[derive(Debug, Getters)]
pub struct App<Ui: UserInterface> {
    /// The user interface used by the app.
    // TODO: remove getter
    #[get = "pub"]
    ui: Ui,

    project: project::Manager,
    #[debug(skip)]
    renderer: project::Renderer,

    #[debug(skip)]
    audio_config: Config,

    popups: popup::Manager,
    context_menu: CloneCell<Option<MenuInstance>>,
    /// The currently held object.
    hand: Cell<Option<HoldableObject>>,

    // TODO: move to temporary settings
    #[get = "pub"]
    /// The height of the project bar.
    project_bar_height: NonZeroLength,
    track_settings_width: NonZeroLength,

    selected_track: WeakCell<Track>,
    selected_clip: WeakCell<Clip>,

    /// The position of the musical cursor.
    ///
    /// If audio is playing, this may not reflect the actual position,
    /// but the position of the cursor at the time when audio playback started.
    cursor: Cell<Instant>,

    /// The settings for the overview grid.
    // TODO: move to temporary settings
    grid: Grid,
    // TODO: move to temporary settings
    /// How far to the left the overview has been moved.
    #[get = "pub"]
    overview_offset: Cell<Length>,
    // TODO: move to temporary settings
    /// The settings regarding the piano roll.
    #[get = "pub"]
    piano_roll_settings: Cell<Settings>,
}

impl<Ui: UserInterface> App<Ui> {
    /// Creates a new instance
    #[must_use]
    pub fn new(ui: Ui) -> App<Ui> {
        let height = ui.size().height;

        App {
            ui,

            project: project::Manager::new(Project::default()),
            renderer: project::Renderer::default(),

            audio_config: Config::default(),

            popups: popup::Manager::new(),
            context_menu: CloneCell::new(None),
            hand: Cell::new(None),

            project_bar_height: Ui::PROJECT_BAR_HEIGHT,
            track_settings_width: Ui::TRACK_SETTINGS_WITH,

            selected_track: WeakCell::new(Weak::new()),
            selected_clip: WeakCell::new(Weak::new()),
            cursor: Cell::new(Instant::START),

            grid: Grid {
                cell_duration: NonZeroDuration::QUARTER,
                cell_width: Ui::CELL_WIDTH,
            },
            overview_offset: Cell::new(Length::ZERO),
            piano_roll_settings: Cell::new(Settings {
                x_offset: Length::ZERO,
                y_offset: Offset::ZERO,
                content_height: height * Ratio::HALF,
                open: false,
                key_width: Ui::KEY_WIDTH,
                piano_depth: Ui::PIANO_DEPTH,
                black_key_depth: Ui::BLACK_KEY_DEPTH,
            }),
        }
    }

    /// Returns the position of the musical cursor.
    fn cursor(&self) -> Instant {
        if let Some(position) = self.audio_config.player_position() {
            self.project.time_mapping().musical(position)
        } else {
            self.cursor.get()
        }
    }

    /// The main view of the app behind any popups
    fn main_view(&self) -> View {
        View::y_stack([
            self.project
                .bar::<Ui>(self.audio_config.is_player_playing())
                .quotated(self.project_bar_height.get()),
            self.project
                .workspace::<Ui>(
                    self.track_settings_width,
                    self.grid,
                    self.overview_offset.get(),
                    &self.selected_track.get(),
                    &self.selected_clip.get(),
                    self.cursor(),
                    self.audio_config.player().ok().as_ref(),
                )
                .fill_remaining(),
            piano_roll::view::<Ui>(
                &self.selected_clip.get(),
                ui::Mapping {
                    time_signature: self.project.time_signature(),
                    grid: self.grid,
                },
                self.piano_roll_settings.get(),
                &self.project.key(),
            ),
        ])
    }

    /// The view of the app.
    ///
    /// This includes popups and the context menu.
    /// The view may need to be recomputed if an action is taken on the app.
    pub fn view(&self) -> View {
        let mut layers = vec![self.main_view()];

        for instance in self.popups.to_vec() {
            layers.push(instance.into_view());
        }

        if let Some(instance) = self.context_menu.get() {
            layers.push(instance.into_view());
        }

        View::Layers(layers)
    }
}
