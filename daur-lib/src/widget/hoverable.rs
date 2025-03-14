use crate::ui::{Length, Point, Rectangle, Size};
use crate::widget::{HasSize, Widget};
use crate::Action;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;

/// A widget that whose appearance changes when hovered.
#[derive(Debug)]
pub struct Hoverable<Default, Hovered = Default> {
    /// The widget to use when not hovered.
    pub default: Default,
    /// The widget to use when hovered.
    pub hovered: Hovered,
}

impl<Default: Widget, Hovered: Widget> Widget for Hoverable<Default, Hovered> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        if area.contains(mouse_position) {
            self.hovered.render(area, buffer, mouse_position);
        } else {
            self.default.render(area, buffer, mouse_position);
        }
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        if area.contains(position) {
            self.hovered.click(area, button, position, actions);
        } else {
            self.default.click(area, button, position, actions);
        }
    }
}

impl<Default: HasSize, Hovered: HasSize> HasSize for Hoverable<Default, Hovered> {
    fn size(&self) -> Size {
        let default = self.default.size();
        let hovered = self.hovered.size();

        Size {
            width: Length::max(default.width, hovered.width),
            height: Length::max(default.height, hovered.height),
        }
    }
}
