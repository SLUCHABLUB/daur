pub mod action;
pub mod control;
mod error;
mod events;
pub mod ruler;
pub mod settings;
pub mod window;

use crate::app::action::Action;
use crate::app::control::default;
use crate::app::events::spawn_events_thread;
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
use rodio::cpal::traits::HostTrait;
use rodio::cpal::{default_host, Host};
use rodio::{Device, DeviceTrait, OutputStream, OutputStreamHandle, Sink};
use std::collections::HashMap;
use std::panic::resume_unwind;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime};

macro_rules! popup_error {
    ($error:expr, $app:ident) => {{
        let popup = Popup::from($error);
        $app.popups.push(popup);
        return Default::default();
    }};
}

macro_rules! or_popup {
    ($result:expr, $app:ident) => {
        match $result {
            Ok(ok) => ok,
            Err(error) => popup_error!(error, $app),
        }
    };
}

use {or_popup, popup_error};

// TODO: make adjustable and read from device
const PLAYBACK_SAMPLE_RATE: u32 = 44_100;

pub struct App {
    controls: HashMap<KeyEvent, Action>,
    project: Project,

    /// When playback started.
    /// `None` means that playback is paused.
    playback_start: Option<SystemTime>,
    // TODO: allow changing
    host: Host,
    device: Option<Device>,
    output_stream: Option<(OutputStream, OutputStreamHandle)>,
    sink: Option<Rc<Sink>>,

    popups: Vec<Popup>,

    project_bar_size: ScreenLength,
    track_settings_size: ScreenLength,

    selected_track: Id<Track>,
    selected_clip: Id<Clip>,
    cursor: Instant,

    overview_settings: OverviewSettings,

    cached_mouse_position: Position,
    cached_area: Rect,
    should_redraw: bool,
    should_exit: bool,
}

impl App {
    pub fn new() -> App {
        let host = default_host();
        let device = host.default_output_device();

        App {
            controls: default(),
            project: Project::default(),

            playback_start: None,
            host,
            device,
            output_stream: None,
            sink: None,

            popups: Vec::new(),

            project_bar_size: ScreenLength(5),
            track_settings_size: ScreenLength(20),

            selected_track: Id::nil(),
            selected_clip: Id::nil(),
            cursor: Instant::START,

            overview_settings: OverviewSettings::default(),

            cached_mouse_position: Position::default(),
            cached_area: Rect::default(),
            should_redraw: true,
            should_exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) {
        let (event_sender, events) = channel();

        let events_thread = spawn_events_thread(event_sender);

        while !self.should_exit {
            if let Ok(event) = events.try_recv() {
                self.handle_event(&event);
            };

            if self.should_redraw {
                self.draw(terminal);
                self.should_redraw = self.is_playing();
            }
        }

        if events_thread.is_finished() {
            let Ok(()) = events_thread.join().map_err(resume_unwind);
        }
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) {
        or_popup!(
            terminal.draw(|frame| {
                let area = frame.area();
                let buf = frame.buffer_mut();
                let mouse_position = self.cached_mouse_position;

                self.cached_area = area;

                self.render(area, buf, mouse_position);
            }),
            self
        );
    }

    fn is_playing(&self) -> bool {
        self.playback_start.is_some()
    }

    pub fn start_playback(&mut self) {
        let Some(sink) = self.sink() else {
            return;
        };
        sink.clear();
        sink.append(self.project.to_source(PLAYBACK_SAMPLE_RATE, self.cursor));
        sink.play();
        self.playback_start = Some(SystemTime::now());
    }

    pub fn stop_playback(&mut self) {
        self.cursor = self.playback_position();
        self.playback_start = None;

        if let Some(sink) = self.sink.as_ref() {
            sink.clear();
        }
    }

    fn sink(&mut self) -> Option<Rc<Sink>> {
        if self.sink.is_some() {
            return self.sink.as_ref().map(Rc::clone);
        }
        if let Some((_, handle)) = self.output_stream.as_ref() {
            self.sink = Some(Rc::new(or_popup!(Sink::try_new(handle), self)));
            return self.sink();
        }
        if let Some(device) = self.device.as_ref() {
            self.output_stream = Some(or_popup!(OutputStream::try_from_device(device), self));
            return self.sink();
        }

        let devices = or_popup!(self.host.output_devices(), self);
        self.popups.push(Popup::buttons(devices.map(|device| {
            (
                device.name().unwrap_or_else(|error| error.to_string()),
                Action::SetDevice(device),
            )
        })));

        None
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
                self.project.bar(self.is_playing()),
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
            }

            if popup.info().unimportant {
                action_queue.push(Action::ClosePopup(popup.info().id()));
            }
        }

        self.background()
            .click(area, button, position, action_queue);
    }
}
