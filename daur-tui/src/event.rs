use crate::convert::{position_to_point, rect_to_rectangle, rectangle_to_rect, to_size};
use crate::tui::Tui;
use crossterm::event::{
    Event, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind, read,
};
use daur::ui::{Length, NonZeroLength, Vector};
use daur::view::{Direction, View};
use daur::{Action, App, ArcCell, Receiver as _};
use ratatui::layout::{Position, Rect};
use ratatui::widgets::{Block, Borders};
use ratatui_explorer::FileExplorer;
use std::io;
use std::iter::zip;
use std::path::Path;
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
            let mut actions = Vec::new();

            click(
                &app.view(),
                button,
                app.ui.window_area.get(),
                app.ui.mouse_position.get(),
                &mut actions,
            );

            for action in actions {
                action.take(app);
            }
        }
        MouseEventKind::Up(button) => {
            if button != MouseButton::Left {
                return;
            }

            release_mouse(
                &app.view(),
                app.ui.window_area.get(),
                app.ui.mouse_position.get(),
            );
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

fn titled_content_area(content: &View, area: Rect) -> Rect {
    // shrink the content area if the title is not part of a border
    if matches!(content, View::Bordered { .. }) {
        area
    } else {
        Block::bordered().borders(Borders::TOP).inner(area)
    }
}

// TODO: capture click when clicking a window
fn click(
    view: &View,
    button: MouseButton,
    area: Rect,
    position: Position,
    actions: &mut Vec<Action>,
) {
    match view {
        // clicking the border counts as clicking the content
        View::Bordered { thick: _, content } => click(
            content,
            button,
            Block::bordered().inner(area),
            position,
            actions,
        ),
        View::Button { on_click, content } => {
            let relative_position = position_to_point(position) - rect_to_rectangle(area).position;

            if button == MouseButton::Left {
                on_click.run(to_size(area.as_size()), relative_position.point(), actions);
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
                actions.send(Action::OpenContextMenu {
                    menu: menu.clone(),
                    position: position_to_point(position),
                });
            }

            click(view, button, area, position, actions);
        }
        View::FileSelector { selected_file } => click_file_explorer(selected_file, position),
        View::Generator(generator) => {
            click(&generator(), button, area, position, actions);
        }
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
        View::Sized { view, .. } => click(view, button, area, position, actions),
        View::SizeInformed(generator) => click(
            &generator(to_size(area.as_size())),
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
        View::Titled {
            title: _,
            highlighted: _,
            view,
        } => click(
            view,
            button,
            titled_content_area(view, area),
            position,
            actions,
        ),
        View::Window {
            area: window_area,
            view,
        } => {
            //  Offset the window area.
            let area = *window_area + position_to_point(area.as_position()).position();
            let area = rectangle_to_rect(area);

            click(view, button, area, position, actions);
        }
    }
}

fn click_file_explorer(selected_file: &ArcCell<Path>, position: Position) {
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

fn release_mouse(view: &View, area: Rect, position: Position) {
    match view {
        View::Bordered { content, .. } => {
            release_mouse(content, Block::bordered().inner(area), position);
        }
        View::Button { content, .. } => release_mouse(content, area, position),
        View::Canvas { .. }
        | View::CursorWindow { .. }
        | View::Empty
        | View::FileSelector { .. }
        | View::Rule { .. }
        | View::Solid(_)
        | View::Text { .. } => (),
        View::Contextual { view, .. } | View::Sized { view, .. } => {
            release_mouse(view, area, position);
        }
        View::Generator(generator) => release_mouse(&generator(), area, position),
        View::Hoverable { default, hovered } => release_mouse(
            if area.contains(position) {
                hovered
            } else {
                default
            },
            area,
            position,
        ),
        View::Layers(layers) => layers
            .iter()
            .rev()
            .for_each(|layer| release_mouse(layer, area, position)),
        View::SizeInformed(generator) => {
            release_mouse(&generator(to_size(area.as_size())), area, position);
        }
        View::Stack {
            direction,
            elements,
        } => {
            let quota: Vec<_> = elements.iter().map(|quotated| quotated.quotum).collect();
            let rectangles = rect_to_rectangle(area)
                .split(*direction, &quota)
                .map(rectangle_to_rect);

            for (area, quoted) in zip(rectangles, elements) {
                release_mouse(&quoted.view, area, position);
            }
        }
        View::Titled {
            title: _,
            highlighted: _,
            view,
        } => {
            release_mouse(view, titled_content_area(view, area), position);
        }
        View::Window {
            area: window_area,
            view,
        } => {
            //  Offset the window area.
            let area = *window_area + position_to_point(area.as_position()).position();
            let area = rectangle_to_rect(area);

            release_mouse(view, area, position);
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
