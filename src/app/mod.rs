pub mod action;
mod atomic;
pub mod control;
mod draw;
mod error;
mod events;
mod reference;
pub mod ruler;
pub mod settings;
pub mod window;

use crate::app::action::Action;
use crate::app::control::DEFAULT_CONTROLS;
use crate::app::draw::spawn_draw_thread;
use crate::app::events::spawn_events_thread;
use crate::app::reference::AppShare;
use crate::app::settings::OverviewSettings;
use crate::clip::Clip;
use crate::columns::ScreenLength;
use crate::id::Id;
use crate::popup::Popup;
use crate::project::Project;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::track::Track;
use crate::widget::two_stack::TwoStack;
use crate::widget::Widget;
use crossterm::event::{KeyEvent, MouseButton};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Position, Rect};
use ratatui::DefaultTerminal;
use std::collections::HashMap;
use std::io;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct App {
    pub controls: HashMap<KeyEvent, Action>,
    pub project: Project,

    /// When playback started.
    /// `None` means that playback is paused.
    pub playback_start: Option<SystemTime>,

    pub popups: Vec<Popup>,

    pub project_bar_size: ScreenLength,
    pub track_settings_size: ScreenLength,

    pub selected_track: Id<Track>,
    pub selected_clip: Id<Clip>,
    pub cursor: Instant,

    pub overview_settings: OverviewSettings,
}

impl App {
    pub fn new() -> App {
        App {
            controls: HashMap::from(DEFAULT_CONTROLS),
            project: Project::default(),

            playback_start: None,

            popups: Vec::new(),

            project_bar_size: ScreenLength(5),
            track_settings_size: ScreenLength(20),

            selected_track: Id::nil(),
            selected_clip: Id::nil(),
            cursor: Instant::START,

            overview_settings: OverviewSettings::default(),
        }
    }

    pub fn run(self, terminal: DefaultTerminal) -> io::Result<()> {
        let app = AppShare::new(self);

        let (action_sender, actions) = channel();

        let draw_thread = spawn_draw_thread(Arc::clone(&app), terminal);
        let events_thread = spawn_events_thread(Arc::clone(&app), action_sender);

        // TODO: set redraw to false if not needed
        while !app.should_exit() && !draw_thread.is_finished() && !events_thread.is_finished() {
            let Ok(action) = actions.try_recv() else {
                continue;
            };

            action.take(&app);
        }

        draw_thread.join().expect("Drawing thread panicked")?;
        events_thread
            .join()
            .expect("Event-handler thread panicked")?;
        Ok(())
    }

    fn playback_position(&self) -> Instant {
        if let Some(playback_start) = self.playback_start {
            Period::from_real_time(
                self.cursor,
                &self.project.time_signature,
                &self.project.tempo,
                playback_start.elapsed().unwrap_or(Duration::ZERO),
            )
            .end()
        } else {
            self.cursor
        }
    }

    /// The app's main widget, behind any popups
    fn background(&self) -> impl Widget + use<'_> {
        TwoStack::vertical(
            (
                self.project.bar(self.playback_start.is_some()),
                self.project.workspace(
                    self.track_settings_size,
                    self.overview_settings,
                    self.selected_track,
                    self.selected_clip,
                    self.playback_position(),
                ),
            ),
            [
                Constraint::Length(self.project_bar_size.get()),
                Constraint::Fill(1),
            ],
        )
    }
}

impl Widget for App {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        self.background().render(area, buf, mouse_position);

        for popup in &self.popups {
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
        for popup in &self.popups {
            let area = popup.area_in_window(area);
            if area.contains(position) {
                popup.click(area, button, position, action_queue);
                return;
            } else if popup.unimportant() {
                action_queue.push(Action::ClosePopup(popup.info().id()));
            }
        }

        self.background()
            .click(area, button, position, action_queue);
    }
}
