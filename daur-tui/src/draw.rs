use crate::canvas::Context;
use crate::convert::{
    approximate_colour, length_to_u16, ratatui_to_size, rect_to_rectangle, rectangle_to_rect,
};
use crate::tui::Tui;
use daur::view::{Alignment, Painter, View};
use daur::{App, Colour};
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::symbols::line::VERTICAL;
use ratatui::text::{Line, Text};
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::{Block, Clear, Paragraph, Widget as _};
use ratatui::{DefaultTerminal, layout};
use ratatui_explorer::{FileExplorer, Theme};
use saturating_cast::SaturatingCast as _;
use std::cmp::min;
use std::io;
use std::iter::zip;
use std::num::{NonZeroU64, NonZeroUsize};
use std::sync::Arc;
use std::thread::{JoinHandle, spawn};

pub fn spawn_draw_thread(
    app: Arc<App<Tui>>,
    mut terminal: DefaultTerminal,
) -> JoinHandle<io::Error> {
    spawn(move || {
        loop {
            app.ui.should_redraw.wait_until();

            app.ui.should_redraw.set(app.is_playing());

            let result = terminal.draw(|frame| {
                let area = frame.area();
                let buffer = frame.buffer_mut();

                app.ui.window_area.set(area);

                render(&app.main_view(), area, buffer, app.ui.mouse_position.get());

                let popups = app.ui.popups.read();

                for (_id, area, view) in popups.iter() {
                    Clear.render(*area, buffer);
                    render(view, *area, buffer, app.ui.mouse_position.get());
                }

                drop(popups);

                if let Some(menu) = app.ui.context_menu.get() {
                    let (area, view) = &*menu;

                    Clear.render(*area, buffer);
                    render(view, *area, buffer, app.ui.mouse_position.get());
                }
            });

            if let Err(error) = result {
                return error;
            }
        }
    })
}

type EmptyCanvas = Canvas<'static, fn(&mut ratatui::widgets::canvas::Context)>;

fn render(view: &View, area: Rect, buffer: &mut Buffer, mouse_position: Position) {
    match view {
        View::Bordered {
            title,
            thick,
            content,
        } => {
            let set = if *thick { THICK } else { PLAIN };

            let block = Block::bordered().border_set(set).title(title.as_str());

            let inner = block.inner(area);

            block.render(area, buffer);

            render(content, inner, buffer, mouse_position);
        }
        View::Button {
            on_click: _,
            content,
        } => render(content, area, buffer, mouse_position),
        View::Canvas {
            background,
            painter,
        } => {
            render_canvas(*background, painter, area, buffer);
        }
        View::Contextual { menu: _, view } | View::Sized { view, .. } => {
            render(view, area, buffer, mouse_position);
        }
        View::CursorWindow { offset } => {
            let offset = length_to_u16(*offset);

            if area.width <= offset {
                return;
            }

            let cursor_area = Rect {
                x: area.x.saturating_add(offset),
                y: area.y,
                width: 1,
                height: area.height,
            };

            let line_count = area.height.saturating_cast();

            Text::from(vec![Line::raw(VERTICAL); line_count]).render(cursor_area, buffer);
        }
        View::Empty => (),
        View::FileSelector { selected_file } => {
            let theme = Theme::new()
                .with_block(Block::bordered())
                .add_default_title()
                .with_highlight_symbol("> ");

            let Ok(mut explorer) = FileExplorer::with_theme(theme) else {
                return;
            };

            let Ok(()) = explorer.set_cwd(&*selected_file.get()) else {
                return;
            };

            explorer.widget().render(area, buffer);
        }
        View::Generator(generator) => render(&generator(), area, buffer, mouse_position),
        View::Hoverable { default, hovered } => render(
            if area.contains(mouse_position) {
                hovered
            } else {
                default
            },
            area,
            buffer,
            mouse_position,
        ),
        View::Layers(layers) => {
            for layer in layers {
                render(layer, area, buffer, mouse_position);
            }
        }
        View::Rule { index, cells } => render_rule(*index, *cells, area, buffer),
        View::SizeInformed(generator) => {
            render(
                &generator(ratatui_to_size(area.as_size())),
                area,
                buffer,
                mouse_position,
            );
        }
        View::Solid(colour) => EmptyCanvas::default()
            .background_color(approximate_colour(*colour))
            .render(area, buffer),
        View::Stack {
            direction,
            elements,
        } => {
            let quota: Vec<_> = elements.iter().map(|quotated| quotated.quotum).collect();
            let rectangles = rect_to_rectangle(area)
                .split(*direction, &quota)
                .map(rectangle_to_rect);

            for (area, quoted) in zip(rectangles, elements) {
                render(&quoted.view, area, buffer, mouse_position);
            }
        }
        View::Text { string, alignment } => render_text(string, *alignment, area, buffer),
    }
}

fn render_canvas(background: Colour, painter: &Painter, area: Rect, buffer: &mut Buffer) {
    let width = f64::from(area.width);
    let height = f64::from(area.height);

    Canvas::default()
        .background_color(approximate_colour(background))
        .x_bounds([0.0, width])
        .y_bounds([0.0, height])
        .paint(|context| {
            painter(&mut Context {
                context,
                size: ratatui_to_size(area.as_size()),
            });
        })
        .render(area, buffer);
}

fn render_rule(index: isize, cells: NonZeroU64, area: Rect, buffer: &mut Buffer) {
    let width = usize::from(area.width);
    let cells = NonZeroUsize::try_from(cells).unwrap_or(NonZeroUsize::MAX);

    if index < 0 {
        Text::raw(format!("{index:<width$}\n{:><width$}", "|"))
    } else {
        let first_row = format!("{index:<width$}\n");
        let cell_width = width / cells;
        let first_cell = format!("{:<cell_width$}", "|");

        let standard_cells = cells.get().saturating_sub(1);
        let standard_cell = format!("{:<cell_width$}", ".");

        Text::raw(first_row + &*first_cell + &*standard_cell.repeat(standard_cells))
    }
    .render(area, buffer);
}

fn render_text(string: &str, alignment: Alignment, area: Rect, buffer: &mut Buffer) {
    let paragraph_alignment = match alignment {
        Alignment::TopLeft | Alignment::Left | Alignment::BottomLeft => layout::Alignment::Left,
        Alignment::Top | Alignment::Centre | Alignment::Bottom => layout::Alignment::Center,
        Alignment::TopRight | Alignment::Right | Alignment::BottomRight => layout::Alignment::Right,
    };

    let paragraph = Paragraph::new(string).alignment(paragraph_alignment);

    let height = min(
        paragraph.line_count(area.width).saturating_cast(),
        area.height,
    );

    #[expect(clippy::integer_division, reason = "favour top by rounding down")]
    let y_offset = match alignment {
        Alignment::TopLeft | Alignment::Top | Alignment::TopRight => 0,
        Alignment::Left | Alignment::Centre | Alignment::Right => {
            area.height.saturating_sub(height) / 2
        }
        Alignment::BottomLeft | Alignment::Bottom | Alignment::BottomRight => {
            area.height.saturating_sub(height)
        }
    };

    let area = Rect {
        x: area.x,
        y: area.y.saturating_add(y_offset),
        width: area.width,
        height,
    };

    paragraph.render(area, buffer);
}
