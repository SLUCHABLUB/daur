use crate::convert::{to_point, to_size};
use crate::tui::Tui;
use crossterm::event::{
    Event, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use daur::app::{Action, Actions};
use daur::ui::{Direction, Length, Point, Rectangle};
use daur::view::visit::{Clicker, Dropper, Grabber, Scroller};
use daur::{App, UserInterface as _, View};
use ratatui::layout::{Position, Size};

pub(crate) fn handle_events(events: &[Event], app: &mut App<Tui>) {
    let mut actions = Actions::new();

    for event in events {
        handle_event(event, app, &mut actions);
    }

    app.take_actions(actions);
}

pub fn handle_event(event: &Event, app: &mut App<Tui>, actions: &mut Actions) {
    match *event {
        Event::FocusGained | Event::FocusLost | Event::Paste(_) => (),
        Event::Key(event) => handle_key_event(event, app.ui(), actions),
        Event::Mouse(event) => handle_mouse_event(event, app, actions),
        Event::Resize(width, height) => {
            app.ui_mut().area = Rectangle {
                position: Point::ZERO,
                size: to_size(Size { width, height }),
            }
        }
    }
}

fn handle_key_event(
    KeyEvent {
        code,
        modifiers,
        kind,
        ..
    }: KeyEvent,
    ui: &Tui,
    actions: &mut Actions,
) {
    if kind != KeyEventKind::Press {
        return;
    }

    if let Some(action) = ui.key_action(modifiers, code) {
        actions.push(action);
    }
}

fn handle_mouse_event(
    MouseEvent {
        kind,
        column,
        row,
        modifiers,
    }: MouseEvent,
    app: &mut App<Tui>,
    actions: &mut Actions,
) {
    app.ui_mut().mouse_position = to_point(Position::new(column, row));

    match kind {
        MouseEventKind::Down(button) => {
            app.ui_mut().last_mouse_button_down = button;
            app.ui_mut().mouse_movement_since_mouse_down = false;

            if button != MouseButton::Left {
                return;
            }

            let mut grabber = Grabber::new(app.ui().mouse_position, actions);

            app.view()
                .accept::<Tui, _>(&mut grabber, app.ui().render_area());
        }
        MouseEventKind::Up(_button) => {
            let button = app.ui().last_mouse_button_down;
            let position = app.ui().mouse_position;

            // click

            if !app.ui().mouse_movement_since_mouse_down {
                // TODO: fix shift
                let shift = modifiers.contains(KeyModifiers::ALT);

                let mut clicker = match button {
                    MouseButton::Left => Clicker::left_click(position, !shift, actions),
                    MouseButton::Right => Clicker::right_click(position, actions),
                    MouseButton::Middle => return,
                };

                app.view()
                    .accept::<Tui, _>(&mut clicker, app.ui().render_area());
            }

            // let go

            if let Some(object) = app.held_object() {
                let mut dropper = Dropper::new(object, position, actions);

                app.view()
                    .accept::<Tui, _>(&mut dropper, app.ui().render_area());
            }
        }
        MouseEventKind::Moved => {
            app.ui_mut().mouse_movement_since_mouse_down = true;
        }
        MouseEventKind::Drag(_) => {
            actions.push(Action::MoveHeldObject(app.ui().mouse_position));

            app.ui_mut().mouse_movement_since_mouse_down = true;
        }
        MouseEventKind::ScrollDown => {
            scroll(Direction::Down, app.ui(), app.view(), actions);
        }
        MouseEventKind::ScrollUp => {
            scroll(Direction::Up, app.ui(), app.view(), actions);
        }
        MouseEventKind::ScrollLeft => {
            scroll(Direction::Left, app.ui(), app.view(), actions);
        }
        MouseEventKind::ScrollRight => {
            scroll(Direction::Right, app.ui(), app.view(), actions);
        }
    }
}

fn scroll(direction: Direction, ui: &Tui, view: &View, actions: &mut Actions) {
    let offset = -direction * Length::PIXEL;

    let mut scroller = Scroller::new(ui.mouse_position, offset, actions);

    view.accept::<Tui, _>(&mut scroller, ui.render_area());
}
