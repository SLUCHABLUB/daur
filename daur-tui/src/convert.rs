use daur::ui::{Length, Point, Rectangle, Size};
use ratatui::layout::{Position, Rect, Size as RatatuiSize};

pub fn position_to_point(position: Position) -> Point {
    Point {
        x: Length::new(position.x),
        y: Length::new(position.y),
    }
}

pub fn ratatui_to_size(size: RatatuiSize) -> Size {
    Size {
        width: Length::new(size.width),
        height: Length::new(size.height),
    }
}

pub fn rect_to_rectangle(rect: Rect) -> Rectangle {
    Rectangle {
        position: position_to_point(rect.as_position()),
        size: ratatui_to_size(rect.as_size()),
    }
}

pub fn rectangle_to_rect(rectangle: Rectangle) -> Rect {
    Rect {
        x: rectangle.position.x.inner(),
        y: rectangle.position.y.inner(),
        width: rectangle.size.width.inner(),
        height: rectangle.size.height.inner(),
    }
}
