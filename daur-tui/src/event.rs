use crate::convert::{to_point, to_size};
use crate::tui::Tui;
use crossterm::event::{Event, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind};
use daur::ui::{Direction, Length, Point, Rectangle};
use daur::view::visit::{Clicker, Grabber, Scroller};
use daur::{Action, App, View};
use ratatui::layout::{Position, Size};

pub(crate) fn handle_events(events: &[Event], app: &App<Tui>) {
    let mut actions = Vec::new();
    let view = app.ui().view(app);

    for event in events {
        handle_event(event, app.ui(), &view, &mut actions);
    }

    app.take_actions(actions);
}

pub fn handle_event(event: &Event, ui: &Tui, view: &View, actions: &mut Vec<Action>) {
    match *event {
        Event::FocusGained | Event::FocusLost | Event::Paste(_) => (),
        Event::Key(event) => handle_key_event(event, ui, actions),
        Event::Mouse(event) => handle_mouse_event(event, ui, view, actions),
        Event::Resize(width, height) => ui.set_area(Rectangle {
            position: Point::ZERO,
            size: to_size(Size { width, height }),
        }),
    }
}

fn handle_key_event(
    KeyEvent {
        code,
        modifiers,
        kind,
        state: _,
    }: KeyEvent,
    ui: &Tui,
    actions: &mut Vec<Action>,
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
        modifiers: _,
    }: MouseEvent,
    ui: &Tui,
    view: &View,
    actions: &mut Vec<Action>,
) {
    ui.set_mouse_position(to_point(Position::new(column, row)));

    let area = ui.area();
    let position = ui.mouse_position();

    actions.push(Action::MoveHand(position));

    match kind {
        MouseEventKind::Down(button) => {
            if button != MouseButton::Left {
                return;
            }

            let mut grabber = Grabber::new(position);

            view.accept::<Tui, _>(&mut grabber, area, position);

            actions.extend(grabber.actions());
        }
        MouseEventKind::Up(button) => {
            // click

            let mut clicker = match button {
                MouseButton::Left => Clicker::left_click(position),
                MouseButton::Right => Clicker::right_click(position),
                MouseButton::Middle => return,
            };

            view.accept::<Tui, _>(&mut clicker, area, position);

            actions.extend(clicker.actions());

            // let go

            actions.push(Action::LetGo);
        }
        MouseEventKind::Moved | MouseEventKind::Drag(_) => (),
        MouseEventKind::ScrollDown => {
            scroll(Direction::Down, ui, view, actions);
        }
        MouseEventKind::ScrollUp => {
            scroll(Direction::Up, ui, view, actions);
        }
        MouseEventKind::ScrollLeft => {
            scroll(Direction::Left, ui, view, actions);
        }
        MouseEventKind::ScrollRight => {
            scroll(Direction::Right, ui, view, actions);
        }
    }
}

fn scroll(direction: Direction, ui: &Tui, view: &View, actions: &mut Vec<Action>) {
    let offset = -direction * Length::PIXEL;

    let mouse_position = ui.mouse_position();
    let area = ui.area();

    let mut scroller = Scroller::new(mouse_position, offset);

    view.accept::<Tui, _>(&mut scroller, area, mouse_position);

    actions.extend(scroller.actions());
}
