use crate::SHOULD_EXIT;
use crate::convert::{
    position_to_point, ratatui_to_size, rect_to_rectangle, rectangle_to_rect, size_to_ratatui,
};
use crate::draw::{SHOULD_REDRAW, WINDOW_AREA};
use crossterm::event::{
    Event, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind, read,
};
use daur::ui::{Length, Vector};
use daur::view::{Direction, View};
use daur::{Action, App, Cell, OptionArcCell};
use ratatui::layout::{Position, Rect};
use ratatui::widgets::Block;
use ratatui_explorer::FileExplorer;
use std::io;
use std::iter::zip;
use std::sync::Arc;
use std::thread::{JoinHandle, spawn};

pub static MOUSE_POSITION: Cell<Position> = Cell::new(Position::ORIGIN);
pub static CONTEXT_MENU: OptionArcCell<(Rect, View)> = OptionArcCell::none();

pub fn spawn_events_thread(app: Arc<App>) -> JoinHandle<io::Error> {
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
                Event::Resize(width, height) => WINDOW_AREA.set(Rect {
                    x: 0,
                    y: 0,
                    width,
                    height,
                }),
            }

            SHOULD_REDRAW.set(true);
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
    app: &App,
) {
    if kind != KeyEventKind::Press {
        return;
    }

    let key_name = format!("{modifiers} + {code}");

    if let Some(action) = app.controls.get().get(&key_name) {
        action.clone().take(app, || SHOULD_EXIT.set(true));
    }
}

fn handle_mouse_event(
    MouseEvent {
        kind,
        column,
        row,
        modifiers: _,
    }: MouseEvent,
    app: &App,
) {
    MOUSE_POSITION.set(Position::new(column, row));

    match kind {
        MouseEventKind::Down(button) => {
            let mut actions = Vec::new();

            let mut consumed = false;

            if let Some(menu) = CONTEXT_MENU.get() {
                let (area, view) = &*menu;

                if area.contains(MOUSE_POSITION.get()) {
                    click(view, button, *area, MOUSE_POSITION.get(), &mut actions);
                    consumed = true;
                } else {
                    CONTEXT_MENU.set(None);
                }
            }

            for popup in app.popups.to_stack().into_iter().rev() {
                if consumed {
                    break;
                }

                let area =
                    rectangle_to_rect(popup.area_in_window(rect_to_rectangle(WINDOW_AREA.get())));

                if area.contains(MOUSE_POSITION.get()) {
                    click(
                        &popup.view(),
                        button,
                        area,
                        MOUSE_POSITION.get(),
                        &mut actions,
                    );

                    consumed = true;
                }
            }

            if !consumed {
                click(
                    &app.main_view(),
                    button,
                    WINDOW_AREA.get(),
                    MOUSE_POSITION.get(),
                    &mut actions,
                );
            }

            for action in actions {
                action.take(app, || SHOULD_EXIT.set(true));
            }
        }
        MouseEventKind::Up(_) => {
            // TODO:
            //  - drop held item
            //  - stop selection
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

fn click(
    view: &View,
    button: MouseButton,
    area: Rect,
    position: Position,
    actions: &mut Vec<Action>,
) {
    match view {
        // clicking the border counts as clicking the content
        View::Bordered {
            title: _,
            thick: _,
            content,
        } => click(
            content,
            button,
            Block::bordered().inner(area),
            position,
            actions,
        ),
        View::Button { on_click, content } => {
            let relative_position = Position {
                x: position.x.saturating_sub(area.x),
                y: position.y.saturating_sub(area.y),
            };

            if button == MouseButton::Left {
                on_click.run(
                    ratatui_to_size(area.as_size()),
                    position_to_point(relative_position),
                    actions,
                );
            }

            click(content, button, area, position, actions);
        }
        View::Canvas { .. }
        | View::CursorWindow { .. }
        | View::Empty
        | View::Rule { .. }
        | View::Solid(_)
        | View::Text { .. } => (),
        View::Contextual { menu, view } => {
            if button == MouseButton::Right {
                let view = menu.view();

                let size = size_to_ratatui(view.minimum_size());
                let area = Rect::from((position, size));

                CONTEXT_MENU.set_some_value((area, menu.view()));
            }

            click(view, button, area, position, actions);
        }
        View::FileSelector { selected_file } => {
            let Ok(mut explorer) = FileExplorer::new() else {
                return;
            };

            let Ok(()) = explorer.set_cwd(&*selected_file.get()) else {
                return;
            };

            let index = usize::from(position.y);
            let Some(file) = explorer.files().get(index).or(explorer.files().last()) else {
                // Should be unreachable since `..` is always in the list
                return;
            };

            selected_file.set(Arc::from(file.path().as_path()));
        }
        View::Generator(generator) => click(&generator(), button, area, position, actions),
        View::Hoverable { default, hovered } => click(
            if area.contains(position) {
                hovered
            } else {
                default
            },
            button,
            area,
            position,
            actions,
        ),
        // .rev() so that the outermost layers get clicked first
        View::Layers(layers) => layers
            .iter()
            .rev()
            .for_each(|layer| click(layer, button, area, position, actions)),
        View::SizeInformed(generator) => click(
            &generator(ratatui_to_size(area.as_size())),
            button,
            area,
            position,
            actions,
        ),
        View::Stack {
            direction,
            elements,
        } => {
            let quota: Vec<_> = elements.iter().map(|quotated| quotated.quotum).collect();
            let rectangles = rect_to_rectangle(area)
                .split(*direction, &quota)
                .map(rectangle_to_rect);

            for (area, quoted) in zip(rectangles, elements) {
                if area.contains(position) {
                    click(&quoted.view, button, area, position, actions);
                    // Stack elements are non-overlapping
                    break;
                }
            }
        }
    }
}

// TODO: break into parts and move into the library
fn scroll(app: &App, direction: Direction) {
    let offset = -Vector::directed(Length::new(1), direction);

    let mouse_position = position_to_point(MOUSE_POSITION.get());
    let area = rect_to_rectangle(WINDOW_AREA.get());

    if mouse_position.y < app.project_bar_height {
        // scroll the project bar (do nothing)
    } else if mouse_position.y < area.size.height - app.piano_roll_settings.get().height {
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
