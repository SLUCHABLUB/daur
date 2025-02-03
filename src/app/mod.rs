pub mod action;
mod atomic;
pub mod control;
mod draw;
mod events;
pub mod overview_settings;
mod reference;
pub mod ruler;
pub mod window;

use crate::app::action::Action;
use crate::app::control::DEFAULT_CONTROLS;
use crate::app::draw::spawn_draw_thread;
use crate::app::events::spawn_events_thread;
use crate::app::overview_settings::OverviewSettings;
use crate::app::reference::AppShare;
use crate::columns::ScreenLength;
use crate::project::Project;
use crate::time::instant::Instant;
use crate::time::period::Period;
use crate::widget::two_stack::TwoStack;
use crate::widget::Widget;
use crossterm::event::KeyEvent;
use ratatui::layout::Constraint;
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

    // TODO: popups
    pub project_bar_size: ScreenLength,
    pub track_settings_size: ScreenLength,

    pub selected_track: Option<usize>,
    pub selected_clip: Option<usize>,
    pub cursor: Instant,

    pub overview_settings: OverviewSettings,
}

impl App {
    pub fn new() -> App {
        App {
            controls: HashMap::from(DEFAULT_CONTROLS),
            project: Project::default(),

            playback_start: None,

            project_bar_size: ScreenLength(5),
            track_settings_size: ScreenLength(20),

            selected_track: None,
            selected_clip: None,
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

    fn to_widget(&self) -> impl Widget + use<'_> {
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
