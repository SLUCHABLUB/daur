use crate::app::Action;
use crate::ui::{Length, Mapping, NonZeroLength, Offset, Point, Rectangle};
use crate::widget::{feed, Text, Widget};
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::Direction;
use saturating_cast::SaturatingCast as _;

#[derive(Clone)]
pub struct Ruler {
    pub mapping: Mapping,
    /// How far along the overview has been scrolled
    pub offset: Offset,
}

impl Widget for Ruler {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        feed(Direction::Horizontal, self.offset, area.width, |index| {
            let Ok(index) = usize::try_from(index) else {
                let first = self.mapping.time_signature.bar_n(0);
                let width = self.mapping.bar_width(first);
                let rule = negative_rule(index, width);
                return (rule, width);
            };

            let bar = self.mapping.time_signature.bar_n(index);

            let width = self.mapping.bar_width(bar);

            let rule = rule(index, self.mapping.grid.cell_width, width);

            (rule, width)
        })
        .render(area, buffer, mouse_position);
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {
        // TODO: move or scale overview
    }
}

type Rule = Text;

fn rule(index: usize, cell_width: NonZeroLength, bar_width: Length) -> Rule {
    let cell_count = (bar_width / cell_width).ceil() as usize;
    let spaces_per_cell = cell_width.get() / NonZeroLength::CHAR_WIDTH;
    let spaces_per_cell = spaces_per_cell.round().saturating_cast();

    let mut cell = vec![b' '; spaces_per_cell];
    if let Some(first) = cell.first_mut() {
        *first = b'.';
    }
    let mut cells = cell.repeat(cell_count);
    if let Some(first) = cells.first_mut() {
        *first = b'|';
    }

    let string = index.to_string() + "\n" + &*String::from_utf8_lossy(&cells);

    // TODO: right align
    Text::left_aligned(ArcStr::from(string))
}

fn negative_rule(index: isize, bar_width: Length) -> Rule {
    let arrow_count = bar_width / NonZeroLength::CHAR_WIDTH;
    let string =
        index.to_string() + "\n" + "|" + &*">".repeat(arrow_count.round().saturating_cast());

    // TODO: right align
    Text::left_aligned(ArcStr::from(string))
}
