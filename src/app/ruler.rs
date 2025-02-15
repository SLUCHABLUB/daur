use crate::app::action::Action;
use crate::app::settings::OverviewSettings;
use crate::app::window::Window;
use crate::project::changing::Changing;
use crate::time::TimeSignature;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

#[derive(Copy, Clone)]
pub struct Ruler<'a> {
    pub time_signature: &'a Changing<TimeSignature>,
    pub overview_settings: OverviewSettings,
}

impl Widget for Ruler<'_> {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        let window = Window {
            time_signature: self.time_signature,
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

            let area = Rect {
                x,
                y: area.y,
                width,
                height: area.height,
            };

            segment(
                index,
                self.overview_settings.cell_width as usize,
                width as usize,
            )
            .render(area, buf, mouse_position);
        }
    }

    fn click(&self, _: Rect, _: MouseButton, _: Position, _: &mut Vec<Action>) {
        // TODO: move or scale overview
    }
}

fn segment(index: usize, cell_width: usize, bar_width: usize) -> impl Widget {
    let cell_count = bar_width.div_ceil(cell_width);
    let mut cell = vec![b' '; cell_width];
    cell[0] = b'.';
    let mut cells = cell.repeat(cell_count);
    cells[0] = b'|';

    Paragraph::new(vec![
        Line::raw(index.to_string()),
        Line::raw(String::from_utf8(cells).unwrap()),
    ])
}
