use crate::app::{or_popup, App};
use crate::keyboard::Key;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::widget::Widget as _;
use crossterm::event;
use crossterm::event::{Event, MouseEventKind};
use never::Never;
use ratatui::layout::{Position, Rect};
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub fn spawn_events_thread(app: Arc<App>) -> JoinHandle<Never> {
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

                let Some(key) = Key::from_event(event) else {
                    return;
                };

                let mut event_captured = false;

                if let Some(popup) = self.popups.top() {
                    let mut actions = Vec::new();

                    event_captured = popup.handle_key(key, &mut actions);

                    for action in actions {
                        action.take(self);
                    }
                }

                if !event_captured {
                    if let Some(action) = self.controls.get(&key).cloned() {
                        action.take(self);
                    }
                }
            }
            Event::Mouse(event) => {
                self.should_redraw.set(true);

                let new_position = Point::from_position(Position::new(event.column, event.row));
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

                self.cached_area
                    .set(Rectangle::from_rect(Rect::new(0, 0, width, height)));
            }
        }
    }
}
