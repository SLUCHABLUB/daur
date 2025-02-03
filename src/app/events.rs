use crate::app::action::Action;
use crate::app::reference::AppShare;
use crate::widget::Widget;
use crossterm::event;
use crossterm::event::{Event, MouseEventKind};
use ratatui::layout::{Position, Rect};
use std::io;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub fn spawn_events_thread(
    app: Arc<AppShare>,
    action_sender: Sender<Action>,
) -> JoinHandle<io::Result<()>> {
    spawn(move || {
        while !app.should_exit() {
            let result = event::read();

            let Ok(event) = result else {
                app.set_exit();
                return result.map(|_| ());
            };

            match event {
                Event::FocusGained | Event::FocusLost => (),
                Event::Key(event) => {
                    if let Some(action) = app.read_lock().controls.get(&event) {
                        let _ = action_sender.send(*action);
                    }
                }
                Event::Mouse(event) => {
                    app.set_mouse_position(Position::new(event.column, event.row));

                    match event.kind {
                        MouseEventKind::Down(button) => {
                            let mut action_queue = Vec::new();

                            app.read_lock().to_widget().click(
                                app.area(),
                                button,
                                app.mouse_position(),
                                &mut action_queue,
                            );

                            for action in action_queue {
                                let _ = action_sender.send(action);
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
                Event::Paste(string) => {
                    todo!("paste: {string}")
                }
                Event::Resize(width, height) => {
                    app.set_area(Rect {
                        x: 0,
                        y: 0,
                        width,
                        height,
                    });
                }
            }
        }
        Ok(())
    })
}
