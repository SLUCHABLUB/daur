use crate::ui::{Point, Rectangle};
use crate::widget::Widget;
use crate::Action;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;

/// A type that can be converted to a widget
pub trait ToWidget {
    /// The widget type
    type Widget<'widget>: Widget
    where
        Self: 'widget;

    /// Returns the widget
    fn to_widget(&self) -> Self::Widget<'_>;
}

impl<T: ToWidget> Widget for T {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        self.to_widget().render(area, buffer, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        self.to_widget().click(area, button, position, actions);
    }
}
