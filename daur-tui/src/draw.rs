use crate::canvas::Context;
use crate::convert::{approximate_colour, from_rectangle, to_rectangle};
use crate::tui::Tui;
use ascii::{AsciiChar, AsciiStr};
use daur::app::Action;
use daur::ui::{Colour, Length, Offset, Rectangle, Size, Theme, ThemeColour, Vector};
use daur::view::context::Menu;
use daur::view::visit::Visitor;
use daur::view::{Alignment, DropAction, OnClick, Painter};
use daur::{App, Holdable, Selectable, UserInterface as _};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Stylize as _;
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::symbols::line::VERTICAL;
use ratatui::text::{Line, Text};
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Widget as _};
use ratatui::{DefaultTerminal, layout};
use saturating_cast::SaturatingCast as _;
use std::cmp::min;
use std::io;
use std::iter::repeat_n;
use std::num::{NonZeroU64, NonZeroUsize};

pub(crate) fn redraw(app: &mut App<Tui>, terminal: &mut DefaultTerminal) -> io::Result<()> {
    terminal
        .draw(|frame| {
            let area = to_rectangle(frame.area());
            let buffer = frame.buffer_mut();

            app.ui_mut().area = area;

            let ui = app.ui();

            app.view().accept::<Tui, _>(
                &mut Renderer {
                    buffer,
                    theme: app.theme(),
                },
                ui.render_area(),
            );
        })
        .map(|_| ())
}

struct Renderer<'buffer> {
    buffer: &'buffer mut Buffer,
    theme: Theme,
}

impl Visitor for Renderer<'_> {
    fn visit_border(&mut self, area: Rectangle, title: Option<&str>, thick: bool) {
        let area = from_rectangle(area);

        let set = if thick { THICK } else { PLAIN };

        Block::bordered()
            .border_set(set)
            .title_alignment(layout::Alignment::Center)
            .title(title.unwrap_or(""))
            .render(area, self.buffer);
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

    fn visit_grabbable(&mut self, _: Rectangle, _: Holdable) {}

    fn visit_object_acceptor(&mut self, _: Rectangle, _: &DropAction) {}

    fn visit_rule(
        &mut self,
        area: Rectangle,
        index: usize,
        cells: NonZeroU64,
        left_crop: Length,
        full_width: Length,
    ) {
        let cells = NonZeroUsize::try_from(cells).unwrap_or(NonZeroUsize::MAX);

        let first_line = format!("{index:<0$}\n", full_width.pixels as usize);
        // This should be infallible.
        let first_line = first_line
            .get((left_crop.pixels as usize)..)
            .unwrap_or(&first_line);

        let cell_width = usize::from(full_width.pixels).div_ceil(cells.get());
        // The spacing between cells.
        let cell_space = cell_width.saturating_sub(1);

        let mut second_line = Vec::with_capacity(full_width.pixels as usize);

        for _ in 0..cells.get() {
            second_line.push(AsciiChar::Dot);
            second_line.extend(repeat_n(AsciiChar::Space, cell_space));
        }

        if let Some(first) = second_line.get_mut(0) {
            *first = AsciiChar::VerticalBar;
        }

        let start_index = left_crop.pixels as usize;
        let end_index = (left_crop + area.size.width).pixels as usize;

        let second_line: &AsciiStr = second_line
            .get(start_index..end_index)
            .unwrap_or(&[])
            .as_ref();
        let second_line = second_line.as_str();

        Text::from(vec![Line::raw(first_line), Line::raw(second_line)])
            .render(from_rectangle(area), self.buffer);
    }

    fn visit_selectable(&mut self, _: Rectangle, _: Selectable) {}

    fn visit_selection_box(&mut self, area: Rectangle) {
        self.visit_border(area, None, false);
    }

    fn visit_scrollable(&mut self, _: Rectangle, _: fn(Vector) -> Action) {}

    fn visit_solid(&mut self, area: Rectangle, colour: ThemeColour) {
        let area = from_rectangle(area);

        Clear.render(area, self.buffer);
        Paragraph::default()
            .bg(approximate_colour(self.theme.resolve(colour)))
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

    fn visit_title_bar(&mut self, area: Rectangle, title: &str, highlighted: bool) {
        let area = from_rectangle(area);

        let set = if highlighted { THICK } else { PLAIN };

        let block = Block::bordered()
            .border_set(set)
            .title_alignment(layout::Alignment::Center);

        match area.height {
            0 => (),
            1 => block
                .borders(Borders::TOP)
                .title(title)
                .render(area, self.buffer),
            2 => block.title(title).render(area, self.buffer),
            3.. => {
                let inner_area = to_rectangle(block.inner(area));
                block.render(area, self.buffer);

                self.visit_text(inner_area, title, Alignment::Centre);
            }
        }
    }
}
