use crate::app::window::Window;
use crate::app::Action;
use crate::app::OverviewSettings;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::length::Length;
use crate::project::changing::Changing;
use crate::time::TimeSignature;
use crate::widget::text::Text;
use crate::widget::Widget;
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use saturating_cast::SaturatingCast as _;
use std::sync::Arc;

#[derive(Clone)]
pub struct Ruler {
    pub time_signature: Arc<Changing<TimeSignature>>,
    pub overview_settings: OverviewSettings,
}

impl Widget for Ruler {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        let time_signature = Arc::clone(&self.time_signature);

        let window = Window {
            time_signature,
            overview_settings: self.overview_settings,
            x: area.x,
            width: area.width,
        };

        let mut started = false;
        for (index, bar) in self.time_signature.bars().enumerate() {
            let x = match window.instant_to_column(bar.start) {
                Some(x) => x,
                None if started => break,
                None => continue,
            };
            started = true;

            let width = bar
                .column_width(self.overview_settings)
                .min(area.x + area.width - x);

            let area = Rectangle {
                x,
                y: area.y,
                width,
                height: area.height,
            };

            segment(index, self.overview_settings.cell_width, width).render(
                area,
                buf,
                mouse_position,
            );
        }
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {
        // TODO: move or scale overview
    }
}

// TODO: implement this with a stack rather than a paragraph
fn segment(index: usize, cell_width: Length, bar_width: Length) -> impl Widget {
    let cell_count = (bar_width / cell_width).ceil() as usize;
    let spaces_per_cell = cell_width / Length::CHAR_WIDTH;
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
