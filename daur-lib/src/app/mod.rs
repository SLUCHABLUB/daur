mod action;

pub use action::Action;

use crate::cell::{CloneCell, WeakCell};
use crate::context::MenuInstance;
use crate::observed::Observed;
use crate::time::real::Duration;
use crate::time::{Instant, Mapping, NonZeroDuration};
use crate::ui::{Grid, Length, NonZeroLength, Offset};
use crate::view::piano_roll::Settings;
use crate::view::{Direction, OnClick, Quotated, View, piano_roll};
use crate::{
    ArcCell, Cell, Clip, OptionArcCell, Project, Track, UserInterface, popup, project, ui,
};
use derive_more::Debug;
use rodio::Device;
use rodio::cpal::traits::HostTrait as _;
use rodio::cpal::{Host, default_host};
use std::collections::HashMap;
use std::sync::Weak;
use std::time::SystemTime;

/// A running instance of the DAW.
#[derive(Debug)]
pub struct App<Ui: UserInterface> {
    /// The user interface.
    pub ui: Ui,

    /// The keybinds.
    /// How the keys are interpreted is based on the UI implementation.
    pub controls: ArcCell<HashMap<String, Action>>,
    /// The project manager.
    pub project: project::Manager,

    /// When playback started.
    /// [`None`] means that playback is paused.
    pub playback_start: Observed<Option<SystemTime>>,
    /// The playback-audio host.
    #[debug(ignore)]
    pub host: ArcCell<Host>,
    /// The playback-audio device.
    #[debug(ignore)]
    pub device: OptionArcCell<Device>,

    /// The popup manager.
    pub popups: popup::Manager,
    /// The context menu.
    pub context_menu: CloneCell<Option<MenuInstance>>,

    /// The height of the project bar.
    pub project_bar_height: NonZeroLength,
    /// The width of the track settings.
    pub track_settings_width: NonZeroLength,

    /// The currently selected track.
    pub selected_track: WeakCell<Track>,
    /// The currently selected clip.
    pub selected_clip: WeakCell<Clip>,

    /// The position of the musical cursor.
    pub cursor: Cell<Instant>,

    /// The settings for the overview grid.
    pub grid: Grid,
    /// How far to the right the overview is offset.
    pub overview_offset: Cell<Offset>,
    /// The settings regarding the piano roll.
    pub piano_roll_settings: Cell<Settings>,
}

impl<Ui: UserInterface> App<Ui> {
    /// Creates a new instance
    #[must_use]
    pub fn new(ui: Ui) -> App<Ui> {
        let host = default_host();
        let device = OptionArcCell::from_value(host.default_output_device());
        let host = ArcCell::from_value(host);

        App {
            ui,

            controls: ArcCell::from_value(HashMap::new()),
            project: project::Manager::new(Project::default()),

            playback_start: Observed::new(None),
            host,
            device,

            popups: popup::Manager::new(),
            context_menu: CloneCell::new(None),

            project_bar_height: Ui::PROJECT_BAR_HEIGHT,
            track_settings_width: Ui::TRACK_SETTINGS_WITH,

            selected_track: WeakCell::new(Weak::new()),
            selected_clip: WeakCell::new(Weak::new()),
            cursor: Cell::new(Instant::START),

            grid: Grid {
                cell_duration: NonZeroDuration::QUARTER,
                cell_width: Ui::CELL_WIDTH,
            },
            overview_offset: Cell::new(Offset::ZERO),
            piano_roll_settings: Cell::new(Settings {
                x_offset: Length::ZERO,
                y_offset: Offset::ZERO,
                height: None,
                key_width: Ui::KEY_WIDTH,
                piano_depth: Ui::PIANO_DEPTH,
                black_key_depth: Ui::BLACK_KEY_DEPTH,
            }),
        }
    }

    /// Whether the app is currently playing audio
    #[must_use]
    pub fn is_playing(&self) -> bool {
        self.playback_start.get().is_some()
    }

    /// Starts playing the audio
    pub fn start_playback(&self) {
        self.playback_start.set(Some(SystemTime::now()));
    }

    /// Stops playing the audio
    pub fn stop_playback(&self) {
        self.cursor.set(self.playback_position());
        self.playback_start.set(None);
    }

    fn playback_position(&self) -> Instant {
        let mapping = Mapping {
            tempo: self.project.tempo(),
            time_signature: self.project.time_signature(),
        };

        if let Some(playback_start) = self.playback_start.get() {
            mapping
                .period(
                    self.cursor.get(),
                    playback_start
                        .elapsed()
                        .map(Duration::from)
                        .unwrap_or(Duration::ZERO),
                )
                .end()
        } else {
            self.cursor.get()
        }
    }

    /// The main view of the app behind any popups
    fn main_view(&self) -> View {
        View::Stack {
            direction: Direction::Down,
            elements: vec![
                self.project
                    .bar::<Ui>(self.is_playing())
                    .quotated(self.project_bar_height.get()),
                self.project
                    .workspace::<Ui>(
                        self.track_settings_width,
                        self.grid,
                        self.overview_offset.get(),
                        &self.selected_track.get(),
                        &self.selected_clip.get(),
                        self.playback_position(),
                    )
                    .fill_remaining(),
                self.piano_roll_settings
                    .get()
                    .height
                    .map_or(Quotated::EMPTY, |height| {
                        piano_roll::view::<Ui>(
                            &self.selected_clip.get(),
                            ui::Mapping {
                                time_signature: self.project.time_signature(),
                                grid: self.grid,
                            },
                            self.piano_roll_settings.get(),
                            &self.project.key(),
                        )
                        .quotated(height.get())
                    }),
            ],
        }
    }

    fn view_with_popups(&self) -> View {
        let mut layers = vec![self.main_view()];

        for instance in self.popups.to_vec() {
            layers.push(instance.into_view());
        }

        View::Layers(layers)
    }

    /// The view of the app.
    ///
    /// This includes popups and the context menu.
    pub fn view(&self) -> View {
        if let Some(instance) = self.context_menu.get() {
            View::Layers(vec![
                self.view_with_popups()
                    .on_click(OnClick::from(Action::CloseContextMenu)),
                instance.into_view(),
            ])
        } else {
            self.view_with_popups()
        }
    }
}
