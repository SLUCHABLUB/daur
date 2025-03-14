use crate::app::{or_popup, App};
use crate::keyboard::Key;
use crate::ui::{Length, Offset, Point, Size, Vector};
use crate::widget::Widget as _;
use crate::Action;
use crossterm::event;
use crossterm::event::{Event, MouseEventKind};
use never::Never;
use ratatui::layout;
use ratatui::layout::Position;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

const UP: Vector = Vector::from_y(Offset::negative(Length::CHAR_HEIGHT));
const DOWN: Vector = Vector::from_y(Offset::positive(Length::CHAR_HEIGHT));
const LEFT: Vector = Vector::from_x(Offset::negative(Length::CHAR_WIDTH));
const RIGHT: Vector = Vector::from_x(Offset::positive(Length::CHAR_WIDTH));

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

                self.las_mouse_position
                    .set(Point::from_position(Position::new(event.column, event.row)));

                match event.kind {
                    MouseEventKind::Down(button) => {
                        let area = self.last_rectangle();
                        let position = self.las_mouse_position.get();
                        let mut actions = Vec::new();

                        let mut broke = false;

                        for popup in self.popups.to_stack().into_iter().rev() {
                            let area = popup.area_in_window(area);
                            if area.contains(position) {
                                popup.click(area, button, position, &mut actions);
                                broke = true;
                                break;
                            }

                            if popup.info().unimportant {
                                actions.push(Action::ClosePopup(popup.info().this()));
                            }
                        }

                        if !broke {
                            self.click(area, button, position, &mut actions);
                        }

                        for action in actions {
                            action.take(self);
                        }
                    }
                    MouseEventKind::Up(_) => {
                        // TODO: drop held item
                        // TODO: stop selection
                    }
                    MouseEventKind::Moved | MouseEventKind::Drag(_) => (),
                    MouseEventKind::ScrollDown => {
                        self.scroll(DOWN);
                    }
                    MouseEventKind::ScrollUp => {
                        self.scroll(UP);
                    }
                    MouseEventKind::ScrollLeft => {
                        self.scroll(LEFT);
                    }
                    MouseEventKind::ScrollRight => {
                        self.scroll(RIGHT);
                    }
                }
            }
            Event::Resize(width, height) => {
                self.should_redraw.set(true);

                self.last_size
                    .set(Size::from_size(layout::Size { width, height }));
            }
        }
    }

    fn scroll(&self, direction: Vector) {
        let offset = -direction;

        let mouse_position = self.las_mouse_position.get();
        let height = self.last_size.get().height;

        if mouse_position.y < self.project_bar_size {
            // scroll the project bar (do nothing)
        } else if mouse_position.y < height - self.piano_roll_settings.get().height {
            // scroll the track overview
            let new_offset = self.overview_offset.get() + offset.x;
            self.overview_offset.set(new_offset);
            // TODO: scroll tracks vertically
        } else {
            // scroll the piano roll

            let mut settings = self.piano_roll_settings.get();

            // The x offset is to the right
            settings.x_offset -= offset.x;
            settings.y_offset += offset.y;

            self.piano_roll_settings.set(settings);
        }
    }
}
