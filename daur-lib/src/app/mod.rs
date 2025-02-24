mod action;
mod audio;
mod control;
mod draw;
mod events;
mod macros;

pub use action::Action;

use crate::app::audio::spawn_audio_thread;
use crate::app::control::default;
use crate::app::draw::spawn_draw_thread;
use crate::app::events::spawn_events_thread;
use crate::app::macros::or_popup;
use crate::cell::Cell;
use crate::keyboard::Key;
use crate::piano_roll::PianoRoll;
use crate::popup::Popups;
use crate::project::manager::Manager;
use crate::project::Project;
use crate::time::{Instant, Mapping};
use crate::ui::{Grid, Length, Offset, Point, Rectangle, Size};
use crate::widget::heterogeneous::ThreeStack;
use crate::widget::Widget;
use crate::{ui, PianoRollSettings};
use crossterm::event::MouseButton;
use educe::Educe;
use ratatui::buffer::Buffer;
use ratatui::layout::Constraint;
use ratatui::DefaultTerminal;
use rodio::cpal::traits::HostTrait as _;
use rodio::cpal::{default_host, Host};
use rodio::Device;
use std::collections::HashMap;
use std::hint::spin_loop;
use std::panic::resume_unwind;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

/// A running instance of the DAW
#[derive(Educe)]
#[educe(Debug)]
pub struct App {
    controls: HashMap<Key, Action>,
    project: Manager,

    /// When playback started.
    /// `None` means that playback is paused.
    playback_start: Cell<Option<SystemTime>>,
    // TODO: allow changing
    #[educe(Debug(ignore))]
    host: Host,
    #[educe(Debug(ignore))]
    device: Cell<Option<Device>>,

    popups: Popups,

    project_bar_size: Length,
    track_settings_size: Length,

    // Note that this may not actually index a valid track
    selected_track_index: Cell<usize>,
    // Note that this may not actually index a valid clip
    selected_clip_index: Cell<usize>,

    cursor: Cell<Instant>,

    grid: Grid,
    overview_offset: Cell<Offset>,
    piano_roll_settings: Cell<PianoRollSettings>,

    las_mouse_position: Cell<Point>,
    last_size: Cell<Size>,
    should_redraw: Cell<bool>,
    should_exit: Cell<bool>,
}

impl App {
    /// Creates a new instance
    #[must_use]
    pub fn new() -> App {
        let host = default_host();
        let device = Cell::new(host.default_output_device());

        App {
            controls: default(),
            project: Manager::new(Project::default()),

            playback_start: Cell::new(None),
            host,
            device,

            popups: Popups::new(),

            project_bar_size: Length::PROJECT_BAR_MINIMUM,
            track_settings_size: Length::TRACK_SETTINGS_DEFAULT,

            selected_track_index: Cell::new(0),
            selected_clip_index: Cell::new(0),
            cursor: Cell::new(Instant::START),

            grid: Grid::default(),
            overview_offset: Cell::new(Offset::ZERO),
            piano_roll_settings: Cell::new(PianoRollSettings::default()),

            las_mouse_position: Cell::default(),
            last_size: Cell::default(),
            should_redraw: Cell::new(true),
            should_exit: Cell::new(false),
        }
    }

    /// Runs the app
    pub fn run(self, terminal: DefaultTerminal) {
        let app = Arc::new(self);

        let audio_thread = spawn_audio_thread(Arc::clone(&app));
        let draw_thread = spawn_draw_thread(Arc::clone(&app), terminal);
        let events_thread = spawn_events_thread(Arc::clone(&app));

        while !app.should_exit.get() {
            spin_loop();
        }

        // TODO: save

        if audio_thread.is_finished() {
            let Err(error) = audio_thread.join();
            resume_unwind(error)
        }

        if draw_thread.is_finished() {
            let Err(error) = draw_thread.join();
            resume_unwind(error)
        }

        if events_thread.is_finished() {
            let Err(error) = events_thread.join();
            resume_unwind(error)
        }
    }

    fn is_playing(&self) -> bool {
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

    fn last_rectangle(&self) -> Rectangle {
        Rectangle {
            position: Point::ZERO,
            size: self.last_size.get(),
        }
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

    /// The app's main widget, behind any popups
    fn background(&self) -> impl Widget + use<'_> {
        ThreeStack::vertical(
            (
                self.project.bar(self.is_playing()),
                self.project.workspace(
                    self.track_settings_size,
                    self.grid,
                    self.overview_offset.get(),
                    self.selected_track_index.get(),
                    self.selected_clip_index.get(),
                    self.playback_position(),
                ),
                PianoRoll {
                    clip: self.project.clip(
                        self.selected_track_index.get(),
                        self.selected_clip_index.get(),
                    ),
                    mapping: ui::Mapping {
                        time_signature: self.project.time_signature(),
                        grid: self.grid,
                    },
                    settings: self.piano_roll_settings.get(),

                    key: self.project.key(),
                },
            ),
            [
                self.project_bar_size.constraint(),
                Constraint::Fill(1),
                self.piano_roll_settings.get().height.constraint(),
            ],
        )
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl Widget for App {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        self.background().render(area, buffer, mouse_position);

        for popup in self.popups.to_stack() {
            let area = popup.area_in_window(area);
            popup.render(area, buffer, mouse_position);
        }
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        for popup in self.popups.to_stack() {
            let area = popup.area_in_window(area);
            if area.contains(position) {
                popup.click(area, button, position, actions);
                return;
            }

            if popup.info().unimportant {
                actions.push(Action::ClosePopup(popup.info().this()));
            }
        }

        self.background().click(area, button, position, actions);
    }
}
