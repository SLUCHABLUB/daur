use crate::ui::{Point, Rectangle, relative};

/// Information about the user interface that a reactive view may use.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub struct RenderArea {
    /// The area of the view.
    pub area: Rectangle,
    /// The position of the mouse cursor.
    pub mouse_position: Point,
}

impl RenderArea {
    /// Whether the area of the view contains the mouse.
    #[must_use]
    pub fn is_hovered(self) -> bool {
        self.area.contains(self.mouse_position)
    }

    /// Returns the position of the mouse cursor relative to the area.
    #[must_use]
    pub fn relative_mouse_position(self) -> Option<relative::Point> {
        self.is_hovered()
            .then_some(self.mouse_position.relative_to(self.area.position))
    }

    /// Returns a moved copy of the rendering area.
    #[must_use]
    pub fn with_area(mut self, area: Rectangle) -> RenderArea {
        self.area = area;
        self
    }
}
