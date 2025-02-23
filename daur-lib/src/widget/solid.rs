use crate::app::Action;
use crate::ui::{Point, Rectangle};
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::widgets::WidgetRef as _;

/// A solid colour
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Solid {
    pub colour: Color,
}

impl Solid {
    pub const BLACK: Solid = Solid {
        colour: Color::Black,
    };
    pub const WHITE: Solid = Solid {
        colour: Color::White,
    };
}

impl Widget for Solid {
    fn render(&self, area: Rectangle, buf: &mut Buffer, _: Point) {
        Canvas::<fn(&mut Context)>::default()
            .background_color(self.colour)
            .render_ref(area.to_rect(), buf);
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {}
}
