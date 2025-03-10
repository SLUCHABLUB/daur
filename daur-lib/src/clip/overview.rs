use crate::clip::Painter;
use crate::time::{NonZeroDuration, Period};
use crate::ui::{Point, Rectangle};
use crate::widget::Widget;
use crate::{Action, Clip};
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::Alignment;
use ratatui::prelude::Style;
use ratatui::symbols::border::{PLAIN, THICK};
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::{Block, Borders};
use std::sync::Arc;

pub struct Overview {
    pub clip: Arc<Clip>,
    pub selected: bool,
    pub track_index: usize,
    pub index: usize,
    pub period: Period,
    pub visible_period: Period,
}

impl Widget for Overview {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        let set = if self.selected { THICK } else { PLAIN };

        let painter: Painter = Box::new(|context| self.clip.content.paint_overview(context));

        let Some(clip_duration) = NonZeroDuration::from_duration(self.period.duration) else {
            return;
        };

        let [mut x, y] = self.clip.content.full_overview_viewport();
        let viewport_width = x[1] - x[0];

        let left_cutoff = self.visible_period.start.since_start;
        let right_cutoff = self.period.end() - self.visible_period.end();

        let left_cutoff_fraction = left_cutoff / clip_duration;
        let right_cutoff_fraction = right_cutoff / clip_duration;

        let left_cutoff = left_cutoff_fraction.to_float() * viewport_width;
        let right_cutoff = right_cutoff_fraction.to_float() * viewport_width;

        x[0] += left_cutoff;
        x[1] -= right_cutoff;

        Canvas::default()
            .background_color(self.clip.colour)
            .paint(painter)
            .block(
                Block::bordered()
                    .borders(Borders::TOP)
                    .title_alignment(Alignment::Center)
                    .border_set(set)
                    .title(self.clip.name.as_str())
                    .style(Style::new().bg(self.clip.colour)),
            )
            .x_bounds(x)
            .y_bounds(y)
            .render(area, buffer, mouse_position);
    }

    fn click(
        &self,
        _area: Rectangle,
        _button: MouseButton,
        _position: Point,
        actions: &mut Vec<Action>,
    ) {
        actions.push(Action::SelectClip {
            track_index: self.track_index,
            index: self.index,
        });
    }
}
