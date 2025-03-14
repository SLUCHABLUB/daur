use crate::ui::{Point, Rectangle};
use crate::view::View;
use crate::Action;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;

/// A type that can be converted to a view
pub trait Composition {
    /// The underlying view
    type Body<'view>: View
    where
        Self: 'view;

    /// Returns the view
    fn body(&self) -> Self::Body<'_>;
}

impl<T: Composition> View for T {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        self.body().render(area, buffer, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        self.body().click(area, button, position, actions);
    }
}
