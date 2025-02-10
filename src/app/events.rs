use crate::app::App;
use crate::widget::Widget;
use crossterm::event;
use crossterm::event::{Event, MouseEventKind};
use ratatui::layout::Position;
use std::sync::mpsc::Sender;
use std::thread::{spawn, JoinHandle};

pub fn spawn_events_thread(events: Sender<Event>) -> JoinHandle<()> {
    spawn(move || {
        loop {
            // TODO: show error
            let Ok(event) = event::read() else { break };
            let Ok(()) = events.send(event) else {
                break;
            };
        }
    })
}

impl App {
    pub fn handle_event(&mut self, event: &Event) {
        match event {
            // We will implement pasting manually
            Event::FocusGained | Event::FocusLost | Event::Paste(_) => (),
            Event::Key(event) => {
                if let Some(action) = self.controls.get(event).cloned() {
                    action.take(self);
                }
            }
            Event::Mouse(event) => {
                let new_position = Position::new(event.column, event.row);
                if new_position != self.cached_mouse_position {
                    self.cached_mouse_position = new_position;
                    // Some widgets change appearance when hovered
                    self.should_redraw = true;
                }

                match event.kind {
                    MouseEventKind::Down(button) => {
                        let mut action_queue = Vec::new();

                        self.click(
                            self.cached_area,
                            button,
                            self.cached_mouse_position,
                            &mut action_queue,
                        );

                        for action in action_queue {
                            action.take(self);
                        }
                    }
                    MouseEventKind::Up(_) => {
                        // TODO: drop held item
                        // TODO: stop selection
                    }
                    MouseEventKind::Moved
                    | MouseEventKind::Drag(_)
                    | MouseEventKind::ScrollDown
                    | MouseEventKind::ScrollUp
                    | MouseEventKind::ScrollLeft
                    | MouseEventKind::ScrollRight => (),
                }
            }
            Event::Resize(width, height) => {
                self.cached_area.width = *width;
                self.cached_area.height = *height;
            }
        }
    }
}
