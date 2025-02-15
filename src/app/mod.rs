pub mod action;
mod audio;
pub mod control;
mod draw;
mod error;
mod events;
mod macros;
pub mod ruler;
mod settings;
pub mod window;

pub use settings::OverviewSettings;

use crate::app::action::Action;
use crate::app::audio::spawn_audio_thread;
use crate::app::control::default;
use crate::app::draw::spawn_draw_thread;
use crate::app::events::spawn_events_thread;
use crate::app::macros::{or_popup, popup_error};
use crate::cell::Cell;
use crate::clip::Clip;
use crate::locked_vec::LockedVec;
use crate::popup::Popup;
use crate::project::Project;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::widget::heterogeneous_stack::TwoStack;
use crate::widget::Widget;
use crossterm::event::{KeyEvent, MouseButton};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Position, Rect};
use ratatui::DefaultTerminal;
use rodio::cpal::traits::HostTrait;
use rodio::cpal::{default_host, Host};
use rodio::Device;
use std::collections::HashMap;
use std::hint::spin_loop;
use std::panic::resume_unwind;
use std::sync::{Arc, Weak};
use std::time::{Duration, SystemTime};

pub struct App {
    controls: HashMap<KeyEvent, Action>,
    project: Project,

    /// When playback started.
    /// `None` means that playback is paused.
    playback_start: Cell<Option<SystemTime>>,
    // TODO: allow changing
    host: Host,
    device: Cell<Option<Device>>,

    popups: LockedVec<Arc<Popup>>,

    // TODO: add a sematic type for these and implement project-wide
    project_bar_size: u16,
    track_settings_size: u16,

    // Note that this may not actually index a valid track
    selected_track_index: Cell<usize>,
    selected_clip: Weak<Clip>,
    cursor: Cell<Instant>,

    overview_settings: OverviewSettings,

    cached_mouse_position: Cell<Position>,
    cached_area: Cell<Rect>,
    should_redraw: Cell<bool>,
    should_exit: Cell<bool>,
}

impl App {
    pub fn new() -> Arc<App> {
        let host = default_host();
        let device = Cell::new(host.default_output_device());

        Arc::new(App {
            controls: default(),
            project: Project::default(),

            playback_start: Cell::new(None),
            host,
            device,

            popups: LockedVec::new(),

            project_bar_size: 5,
            track_settings_size: 20,

            selected_track_index: Cell::new(0),
            selected_clip: Weak::new(),
            cursor: Cell::new(Instant::START),

            overview_settings: OverviewSettings::default(),

            cached_mouse_position: Cell::default(),
            cached_area: Cell::default(),
            should_redraw: Cell::new(true),
            should_exit: Cell::new(false),
        })
    }

    pub fn run(self: Arc<Self>, terminal: DefaultTerminal) {
        let audio_thread = spawn_audio_thread(Arc::clone(&self));
        let draw_thread = spawn_draw_thread(Arc::clone(&self), terminal);
        let events_thread = spawn_events_thread(Arc::clone(&self));

        while !self.should_exit.get() {
            spin_loop();
        }

        // TODO: save

        if audio_thread.is_finished() {
            let Ok(()) = audio_thread.join().map_err(resume_unwind);
        }

        if draw_thread.is_finished() {
            let Ok(()) = draw_thread.join().map_err(resume_unwind);
        }

        if events_thread.is_finished() {
            let Ok(()) = events_thread.join().map_err(resume_unwind);
        }
    }

    fn is_playing(&self) -> bool {
        self.playback_start.get().is_some()
    }

    pub fn start_playback(&self) {
        self.playback_start.set(Some(SystemTime::now()));
    }

    pub fn stop_playback(&self) {
        self.cursor.set(self.playback_position());
        self.playback_start.set(None);
    }

    fn playback_position(&self) -> Instant {
        if let Some(playback_start) = self.playback_start.get() {
            Period::from_real_time(
                self.cursor.get(),
                &self.project.time_signature,
                &self.project.tempo,
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
            [
                Constraint::Length(self.project_bar_size),
                Constraint::Fill(1),
            ],
        )
    }
}

impl Widget for App {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        self.background().render(area, buf, mouse_position);

        for popup in self.popups.iter() {
            let area = popup.area_in_window(area);
            popup.render(area, buf, mouse_position);
        }
    }

    fn click(
        &self,
        area: Rect,
        button: MouseButton,
        position: Position,
        action_queue: &mut Vec<Action>,
    ) {
        for popup in self.popups.iter() {
            let area = popup.area_in_window(area);
            if area.contains(position) {
                popup.click(area, button, position, action_queue);
                return;
            }

            if popup.info().unimportant {
                action_queue.push(Action::ClosePopup(popup.info().this()));
            }
        }

        self.background()
            .click(area, button, position, action_queue);
    }
}
