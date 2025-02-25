use crate::app::Action;
use crate::ui::{Point, Rectangle};
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::style::Color;
use ratatui::widgets;
use ratatui::widgets::canvas::{Canvas, Context};

/// A solid colour
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Solid {
    /// The colour
    pub colour: Color,
}

impl Solid {
    /// A solid black widget
    pub const BLACK: Solid = Solid {
        colour: Color::Black,
    };

    /// A solid white widget
    pub const WHITE: Solid = Solid {
        colour: Color::White,
    };
}

impl Widget for Solid {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, _: Point) {
        widgets::Widget::render(
            Canvas::<fn(&mut Context)>::default().background_color(self.colour),
            area.to_rect(),
            buffer,
        );
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {}
}
