use crate::app::{or_popup, App};
use crate::widget::Widget;
use crossterm::event;
use crossterm::event::{Event, MouseEventKind};
use ratatui::layout::{Position, Rect};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub fn spawn_events_thread(app: Arc<App>) -> JoinHandle<()> {
    spawn(move || loop {
        app.handle_event();
    })
}

impl App {
    fn handle_event(&self) {
        match or_popup!(event::read(), self) {
            // We will implement pasting manually
            Event::FocusGained | Event::FocusLost | Event::Paste(_) => (),
            Event::Key(event) => {
                self.should_redraw.set(true);

                if let Some(action) = self.controls.get(&event).cloned() {
                    action.take(self);
                }
            }
            Event::Mouse(event) => {
                self.should_redraw.set(true);

                let new_position = Position::new(event.column, event.row);
                if new_position != self.cached_mouse_position.get() {
                    self.cached_mouse_position.set(new_position);
                }

                match event.kind {
                    MouseEventKind::Down(button) => {
                        let mut action_queue = Vec::new();

                        self.click(
                            self.cached_area.get(),
                            button,
                            self.cached_mouse_position.get(),
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
                self.should_redraw.set(true);

                self.cached_area.set(Rect {
                    x: 0,
                    y: 0,
                    width,
                    height,
                });
            }
        }
    }
}
