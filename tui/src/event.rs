use crate::Key;
use crate::Tui;
use crate::convert::to_point;
use crate::convert::to_size;
use crossterm::event::Event;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use crossterm::event::MouseButton;
use crossterm::event::MouseEvent;
use crossterm::event::MouseEventKind;
use daur::App;
use daur::UserInterface as _;
use daur::View;
use daur::app::Action;
use daur::app::Actions;
use daur::ui::Direction;
use daur::ui::Length;
use daur::ui::Point;
use daur::ui::Rectangle;
use daur::view::visit::Clicker;
use daur::view::visit::Dropper;
use daur::view::visit::Grabber;
use daur::view::visit::Scroller;
use ratatui::layout::Position;
use ratatui::layout::Size;

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
        Event::Resize(width, height) => app.ui().area.set(Rectangle {
            position: Point::ZERO,
            size: to_size(Size { width, height }),
        }),
    }
}

fn handle_key_event(event: KeyEvent, ui: &Tui, actions: &mut Actions) {
    if event.kind != KeyEventKind::Press {
        return;
    }

    let key = Key::from(event);

    if let Some(action) = ui.configuration.key_map.get(&key) {
        actions.push(action.clone());
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
    app.ui()
        .mouse_position
        .set(to_point(Position::new(column, row)));

    match kind {
        MouseEventKind::Down(button) => {
            app.ui().last_mouse_button_down.set(button);
            app.ui().mouse_movement_since_mouse_down.set(false);

            if button != MouseButton::Left {
                return;
            }

            let mut grabber = Grabber::new(app.ui().mouse_position.get(), actions);

            app.view()
                .accept::<Tui, _>(&mut grabber, app.ui().render_area());
        }
        MouseEventKind::Up(_button) => {
            let button = app.ui().last_mouse_button_down.get();
            let position = app.ui().mouse_position.get();

            // click

            if !app.ui().mouse_movement_since_mouse_down.get() {
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
            app.ui().mouse_movement_since_mouse_down.set(true);
        }
        MouseEventKind::Drag(_) => {
            actions.push(Action::MoveHeldObject(app.ui().mouse_position.get()));

            app.ui().mouse_movement_since_mouse_down.set(true);
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

    let mut scroller = Scroller::new(ui.mouse_position.get(), offset, actions);

    view.accept::<Tui, _>(&mut scroller, ui.render_area());
}
