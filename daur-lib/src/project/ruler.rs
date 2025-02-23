use crate::app::Action;
use crate::ui::{Length, Mapping, NonZeroLength, Point, Rectangle};
use crate::widget::{Text, Widget};
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use saturating_cast::SaturatingCast as _;

#[derive(Clone)]
pub struct Ruler {
    pub mapping: Mapping,
}

impl Widget for Ruler {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let mut started = false;
        for (index, bar) in self.mapping.time_signature.bars().enumerate() {
            let x = match self.mapping.offset_in_range(bar.start, area.width) {
                Some(x) => x + area.x,
                None if started => break,
                None => continue,
            };
            started = true;

            let width = self.mapping.bar_width(bar).min(area.x + area.width - x);

            let area = Rectangle {
                x,
                y: area.y,
                width,
                height: area.height,
            };

            segment(index, self.mapping.grid.cell_width, width).render(
                area,
                buffer,
                mouse_position,
            );
        }
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {
        // TODO: move or scale overview
    }
}

// TODO: implement this with a stack rather than a paragraph
fn segment(index: usize, cell_width: NonZeroLength, bar_width: Length) -> impl Widget {
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

    Text::left_aligned(ArcStr::from(string))
}
