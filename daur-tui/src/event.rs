use crate::convert::{to_point, to_size};
use crate::tui::Tui;
use crossterm::event::{
    Event, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind, read,
};
use daur::App;
use daur::ui::{Length, Point, Rectangle};
use daur::view::Direction;
use daur::view::visit::{Clicker, Grabber};
use ratatui::layout::{Position, Size};
use std::io;
use std::sync::Arc;
use std::thread::{JoinHandle, spawn};

pub fn spawn_events_thread(app: Arc<App<Tui>>) -> JoinHandle<io::Error> {
    spawn(move || {
        loop {
            let event = match read() {
                Ok(event) => event,
                Err(error) => return error,
            };

            match event {
                Event::FocusGained | Event::FocusLost | Event::Paste(_) => (),
                Event::Key(event) => handle_key_event(event, &app),
                Event::Mouse(event) => handle_mouse_event(event, &app),
                Event::Resize(width, height) => app.ui.window_area.set(Rectangle {
                    position: Point::ZERO,
                    size: to_size(Size { width, height }),
                }),
            }

            app.ui.should_redraw.set(true);
        }
    })
}

fn handle_key_event(
    KeyEvent {
        code,
        modifiers,
        kind,
        state: _,
    }: KeyEvent,
    app: &App<Tui>,
) {
    if kind != KeyEventKind::Press {
        return;
    }

    let key_name = format!("{modifiers} + {code}");

    if let Some(action) = app.controls.get().get(&key_name) {
        action.clone().take(app);
    }
}

fn handle_mouse_event(
    MouseEvent {
        kind,
        column,
        row,
        modifiers: _,
    }: MouseEvent,
    app: &App<Tui>,
) {
    app.ui
        .mouse_position
        .set(to_point(Position::new(column, row)));

    let area = app.ui.window_area.get();
    let position = app.ui.mouse_position.get();

    if let Some(object) = app.hand.get() {
        object.update(app, position);
    }

    match kind {
        MouseEventKind::Down(button) => {
            if button != MouseButton::Left {
                return;
            }

            let mut grabber = Grabber::<Tui>::new(position);

            app.view().accept(&mut grabber, area, position);

            app.hand.set(grabber.object());
        }
        MouseEventKind::Up(button) => {
            // click

            let mut clicker = match button {
                MouseButton::Left => Clicker::left_click(position),
                MouseButton::Right => Clicker::right_click(position),
                MouseButton::Middle => return,
            };

            app.view().accept(&mut clicker, area, position);

            clicker.take_actions(app);

            // let go

            if let Some(object) = app.hand.replace(None) {
                object.let_go(app);
            }
        }
        MouseEventKind::Moved | MouseEventKind::Drag(_) => (),
        MouseEventKind::ScrollDown => {
            scroll(app, Direction::Down);
        }
        MouseEventKind::ScrollUp => {
            scroll(app, Direction::Up);
        }
        MouseEventKind::ScrollLeft => {
            scroll(app, Direction::Left);
        }
        MouseEventKind::ScrollRight => {
            scroll(app, Direction::Right);
        }
    }
}

// TODO: break into parts and move into the library
fn scroll(app: &App<Tui>, direction: Direction) {
    // the screen is moved in the opposite direction of the mouse movement
    let offset = -(direction * Length::PIXEL);

    let mouse_position = app.ui.mouse_position.get();
    let area = app.ui.window_area.get();

    let piano_roll_start = area.size.height - app.piano_roll_settings.get().content_height;

    if mouse_position.y < app.project_bar_height.get() {
        // scroll the project bar (do nothing)
    } else if mouse_position.y < piano_roll_start {
        // scroll the track overview
        let new_offset = app.overview_offset.get() + offset.x;
        app.overview_offset.set(new_offset);
        // TODO: scroll tracks vertically
    } else {
        // scroll the piano roll

        let mut settings = app.piano_roll_settings.get();

        // The x offset is to the right
        settings.x_offset -= offset.x;
        settings.y_offset += offset.y;

        app.piano_roll_settings.set(settings);
    }
}
