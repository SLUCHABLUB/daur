use crate::app::Action;
use crate::ui::{Point, Rectangle};
use crate::view::View;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Canvas, Context};
use ratatui::widgets::Widget;

/// A solid colour
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Solid {
    /// The colour
    pub colour: Color,
}

impl Solid {
    /// An empty view
    pub const EMPTY: Solid = Solid {
        colour: Color::Reset,
    };

    /// A solid black view
    pub const BLACK: Solid = Solid {
        colour: Color::Black,
    };

    /// A solid white view
    pub const WHITE: Solid = Solid {
        colour: Color::White,
    };
}

impl View for Solid {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, _: Point) {
        Widget::render(
            Canvas::<fn(&mut Context)>::default().background_color(self.colour),
            area.to_rect(),
            buffer,
        );
    }

    fn click(&self, _: Rectangle, _: MouseButton, _: Point, _: &mut Vec<Action>) {}
}
