use crate::canvas::Context;
use crate::convert::{
    approximate_colour, position_to_point, rect_to_rectangle, rectangle_to_rect, to_size,
};
use crate::tui::Tui;
use daur::ui::{Length, Offset, Rectangle, Size, Vector};
use daur::view::context::Menu;
use daur::view::{Alignment, OnClick, Painter, Visitor};
use daur::{App, Colour, HoldableObject};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::symbols::line::VERTICAL;
use ratatui::text::{Line, Text};
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget as _};
use ratatui::{DefaultTerminal, layout};
use saturating_cast::SaturatingCast as _;
use std::cmp::min;
use std::io;
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

                app.view().accept(
                    &mut Renderer { buffer },
                    rect_to_rectangle(area),
                    position_to_point(app.ui.mouse_position.get()),
                );
            });

            if let Err(error) = result {
                return error;
            }
        }
    })
}

type EmptyCanvas = Canvas<'static, fn(&mut ratatui::widgets::canvas::Context)>;

pub struct Renderer<'buffer> {
    pub buffer: &'buffer mut Buffer,
}

impl Visitor for Renderer<'_> {
    type Ui = Tui;

    fn visit_border(&mut self, area: Rectangle, thick: bool) {
        self.visit_titled_bordered(area, area, "", thick, thick);
    }

    fn visit_canvas(&mut self, area: Rectangle, background: Colour, painter: &Painter) {
        let area = rectangle_to_rect(area);

        let width = f64::from(area.width);
        let height = f64::from(area.height);

        Canvas::default()
            .background_color(approximate_colour(background))
            .x_bounds([0.0, width])
            .y_bounds([0.0, height])
            .paint(|context| {
                painter(&mut Context {
                    context,
                    size: to_size(area.as_size()),
                });
            })
            .render(area, self.buffer);
    }

    fn visit_clickable(&mut self, _: Rectangle, _: &OnClick) {}

    fn visit_contextual(&mut self, _: Rectangle, _: &Menu) {}

    fn visit_cursor_window(&mut self, area: Rectangle, offset: Length) {
        if area.size.width <= offset {
            return;
        }

        let area = rectangle_to_rect(Rectangle {
            position: area.position + Vector::from_x(Offset::positive(offset)),
            size: Size {
                width: Length::PIXEL,
                height: area.size.height,
            },
        });

        let line_count = area.height.saturating_cast();

        Text::from(vec![Line::raw(VERTICAL); line_count]).render(area, self.buffer);
    }

    fn visit_grabbable(&mut self, _: Rectangle, _: HoldableObject) {}

    fn visit_rule(&mut self, area: Rectangle, index: isize, cells: NonZeroU64) {
        let area = rectangle_to_rect(area);

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
        .render(area, self.buffer);
    }

    fn visit_solid(&mut self, area: Rectangle, colour: Colour) {
        let area = rectangle_to_rect(area);

        EmptyCanvas::default()
            .background_color(approximate_colour(colour))
            .render(area, self.buffer);
    }

    fn visit_text(&mut self, area: Rectangle, string: &str, alignment: Alignment) {
        let area = rectangle_to_rect(area);

        let paragraph_alignment = match alignment {
            Alignment::TopLeft | Alignment::Left | Alignment::BottomLeft => layout::Alignment::Left,
            Alignment::Top | Alignment::Centre | Alignment::Bottom => layout::Alignment::Center,
            Alignment::TopRight | Alignment::Right | Alignment::BottomRight => {
                layout::Alignment::Right
            }
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

        paragraph.render(area, self.buffer);
    }

    fn visit_titled(&mut self, area: Rectangle, title: &str, highlighted: bool) {
        let area = rectangle_to_rect(area);

        let set = if highlighted { THICK } else { PLAIN };

        Block::new()
            .borders(Borders::TOP)
            .border_set(set)
            .title(title)
            .render(area, self.buffer);
    }

    fn visit_window(&mut self, area: Rectangle) {
        Clear.render(rectangle_to_rect(area), self.buffer);
    }

    fn visit_titled_bordered(
        &mut self,
        area: Rectangle,
        _titled_area: Rectangle,
        title: &str,
        highlighted: bool,
        thick: bool,
    ) {
        let area = rectangle_to_rect(area);

        let set = if thick || highlighted { THICK } else { PLAIN };

        Block::bordered()
            .border_set(set)
            .title(title)
            .render(area, self.buffer);
    }
}
