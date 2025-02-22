mod action;
mod audio;
pub mod control;
mod draw;
mod events;
mod macros;
mod settings;
pub mod window;

pub use action::Action;
pub use settings::OverviewSettings;

use crate::app::audio::spawn_audio_thread;
use crate::app::control::default;
use crate::app::draw::spawn_draw_thread;
use crate::app::events::spawn_events_thread;
use crate::app::macros::or_popup;
use crate::cell::Cell;
use crate::clip::Clip;
use crate::keyboard::Key;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::length::Length;
use crate::popup::Popups;
use crate::project::manager::Manager;
use crate::project::Project;
use crate::time::{Instant, Period};
use crate::widget::heterogeneous::TwoStack;
use crate::widget::Widget;
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
use std::sync::{Arc, Weak};
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
    selected_clip: Weak<Clip>,
    cursor: Cell<Instant>,

    overview_settings: OverviewSettings,

    cached_mouse_position: Cell<Point>,
    cached_area: Cell<Rectangle>,
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
            selected_clip: Weak::new(),
            cursor: Cell::new(Instant::START),

            overview_settings: OverviewSettings::default(),

            cached_mouse_position: Cell::default(),
            cached_area: Cell::default(),
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

    fn playback_position(&self) -> Instant {
        if let Some(playback_start) = self.playback_start.get() {
            Period::from_real_time(
                self.cursor.get(),
                &self.project.time_signature(),
                &self.project.tempo(),
                playback_start.elapsed().unwrap_or(Duration::ZERO),
            )
            .end()
        } else {
            self.cursor.get()
        }
    }

    /// The app's main widget, behind any popups
    fn background(&self) -> impl Widget + use<'_> {
        TwoStack::vertical(
            (
                self.project.bar(self.is_playing()),
                self.project.workspace(
                    self.track_settings_size,
                    self.overview_settings,
                    self.selected_track_index.get(),
                    &self.selected_clip,
                    self.playback_position(),
                ),
            ),
            [self.project_bar_size.constraint(), Constraint::Fill(1)],
        )
    }
}

impl Default for App {
    fn default() -> Self {
        App::new()
    }
}

impl Widget for App {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        self.background().render(area, buf, mouse_position);

        for popup in self.popups.to_stack() {
            let area = popup.area_in_window(area);
            popup.render(area, buf, mouse_position);
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
