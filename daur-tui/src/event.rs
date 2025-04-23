use crate::convert::{position_to_point, rect_to_rectangle};
use crate::tui::Tui;
use crossterm::event::{
    Event, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind, read,
};
use daur::App;
use daur::ui::{Length, NonZeroLength, Vector};
use daur::view::{Clicker, Direction};
use ratatui::layout::{Position, Rect};
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
                Event::Resize(width, height) => app.ui.window_area.set(Rect {
                    x: 0,
                    y: 0,
                    width,
                    height,
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
    app.ui.mouse_position.set(Position::new(column, row));

    match kind {
        MouseEventKind::Down(button) => {
            let area = rect_to_rectangle(app.ui.window_area.get());
            let position = position_to_point(app.ui.mouse_position.get());

            let mut clicker = match button {
                MouseButton::Left => Clicker::left_click(position),
                MouseButton::Right => Clicker::right_click(position),
                MouseButton::Middle => return,
            };

            app.view().accept(&mut clicker, area, position);

            clicker.take_actions(app);
        }
        MouseEventKind::Up(_) => {
            // TODO: clear the hand
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
    let offset = -Vector::directed(Length::PIXEL, direction);

    let mouse_position = position_to_point(app.ui.mouse_position.get());
    let area = rect_to_rectangle(app.ui.window_area.get());

    let piano_roll_start = area.size.height
        - app
            .piano_roll_settings
            .get()
            .height
            .map_or(Length::ZERO, NonZeroLength::get);

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
