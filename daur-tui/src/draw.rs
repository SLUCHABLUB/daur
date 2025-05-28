use crate::canvas::Context;
use crate::convert::{approximate_colour, from_rectangle, to_rectangle};
use crate::tui::Tui;
use daur::ui::{Colour, Length, Offset, Rectangle, Size, Vector};
use daur::view::context::Menu;
use daur::view::visit::Visitor;
use daur::view::{Alignment, DropAction, OnClick, Painter, SelectableItem};
use daur::{Action, App, HoldableObject, UserInterface as _};
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

pub(crate) fn redraw(app: &mut App<Tui>, terminal: &mut DefaultTerminal) -> io::Result<()> {
    terminal
        .draw(|frame| {
            let area = to_rectangle(frame.area());
            let buffer = frame.buffer_mut();

            app.ui_mut().area = area;

            let ui = app.ui();

            app.view()
                .accept::<Tui, _>(&mut Renderer { buffer }, ui.render_area());
        })
        .map(|_| ())
}

type EmptyCanvas = Canvas<'static, fn(&mut ratatui::widgets::canvas::Context)>;

struct Renderer<'buffer> {
    buffer: &'buffer mut Buffer,
}

impl Visitor for Renderer<'_> {
    fn visit_border(&mut self, area: Rectangle, thick: bool) {
        self.visit_titled_bordered(area, area, "", thick, thick);
    }

    fn visit_canvas(&mut self, area: Rectangle, background: Colour, painter: &Painter) {
        let size = area.size;
        let area = from_rectangle(area);

        let width = f64::from(area.width);
        let height = f64::from(area.height);

        Canvas::default()
            .background_color(approximate_colour(background))
            .x_bounds([0.0, width])
            .y_bounds([0.0, height])
            .paint(|context| {
                painter(&mut Context::new(context, size));
            })
            .render(area, self.buffer);
    }

    fn visit_clickable(&mut self, _: Rectangle, _: &OnClick) {}

    fn visit_contextual(&mut self, _: Rectangle, _: &Menu) {}

    fn visit_cursor_window(&mut self, area: Rectangle, offset: Length) {
        if area.size.width <= offset {
            return;
        }

        let area = from_rectangle(Rectangle {
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

    fn visit_object_acceptor(&mut self, _: Rectangle, _: &DropAction) {}

    fn visit_rule(&mut self, area: Rectangle, index: isize, cells: NonZeroU64) {
        let area = from_rectangle(area);

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

    fn visit_selectable(&mut self, _: Rectangle, _: SelectableItem) {}

    fn visit_selection_box(&mut self, area: Rectangle) {
        self.visit_border(area, false);
    }

    fn visit_scrollable(&mut self, _: Rectangle, _: fn(Vector) -> Action) {}

    fn visit_solid(&mut self, area: Rectangle, colour: Colour) {
        let area = from_rectangle(area);

        EmptyCanvas::default()
            .background_color(approximate_colour(colour))
            .render(area, self.buffer);
    }

    fn visit_text(&mut self, area: Rectangle, string: &str, alignment: Alignment) {
        let area = from_rectangle(area);

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
        let area = from_rectangle(area);

        let set = if highlighted { THICK } else { PLAIN };

        Block::new()
            .borders(Borders::TOP)
            .border_set(set)
            .title(title)
            .render(area, self.buffer);
    }

    fn visit_window(&mut self, area: Rectangle) {
        Clear.render(from_rectangle(area), self.buffer);
    }

    fn visit_titled_bordered(
        &mut self,
        area: Rectangle,
        _titled_area: Rectangle,
        title: &str,
        highlighted: bool,
        thick: bool,
    ) {
        let area = from_rectangle(area);

        let set = if thick || highlighted { THICK } else { PLAIN };

        Block::bordered()
            .border_set(set)
            .title(title)
            .render(area, self.buffer);
    }
}
