mod action;

pub use action::Action;

use crate::observed::Observed;
use crate::time::{Instant, Mapping};
use crate::ui::{Grid, Length, Offset};
use crate::view::piano_roll::Settings;
use crate::view::{Direction, View, piano_roll};
use crate::{ArcCell, Cell, OptionArcCell, Project, UserInterface, popup, project, ui};
use derive_more::Debug;
use rodio::Device;
use rodio::cpal::traits::HostTrait as _;
use rodio::cpal::{Host, default_host};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

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
    pub popups: popup::Manager<Ui>,

    /// The height of the project bar.
    pub project_bar_height: Length,
    /// The width of the track settings.
    pub track_settings_width: Length,

    // TODO: find a semantically superior way to index tracks
    /// The index iof the currently selected track.
    /// Note that this may not index a valid track.
    pub selected_track_index: Cell<usize>,
    /// The index iof the currently selected clip.
    /// Note that this may not index a valid clip.
    pub selected_clip_index: Cell<usize>,

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

            project_bar_height: Length::PROJECT_BAR_HEIGHT,
            track_settings_width: Length::TRACK_SETTINGS_DEFAULT,

            selected_track_index: Cell::new(0),
            selected_clip_index: Cell::new(0),
            cursor: Cell::new(Instant::START),

            grid: Grid::default(),
            overview_offset: Cell::new(Offset::ZERO),
            piano_roll_settings: Cell::new(Settings::default()),
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
                    playback_start.elapsed().unwrap_or(Duration::ZERO),
                )
                .end()
        } else {
            self.cursor.get()
        }
    }

    /// The main view of the app behind any popups
    pub fn main_view(&self) -> View {
        View::Stack {
            direction: Direction::Down,
            elements: vec![
                self.project
                    .bar(self.is_playing())
                    .quotated(self.project_bar_height),
                self.project
                    .workspace(
                        self.track_settings_width,
                        self.grid,
                        self.overview_offset.get(),
                        self.selected_track_index.get(),
                        self.selected_clip_index.get(),
                        self.playback_position(),
                    )
                    .fill_remaining(),
                piano_roll::view(
                    self.project
                        .clip(
                            self.selected_track_index.get(),
                            self.selected_clip_index.get(),
                        )
                        .as_deref(),
                    ui::Mapping {
                        time_signature: self.project.time_signature(),
                        grid: self.grid,
                    },
                    self.piano_roll_settings.get(),
                    &self.project.key(),
                )
                .quotated(self.piano_roll_settings.get().height),
            ],
        }
    }
}
