use daur::Colour;
use daur::ui::{Length, Point, Rectangle, Size};
use ratatui::layout::{Position, Rect, Size as RatatuiSize};
use ratatui::style::Color;

pub fn position_to_point(position: Position) -> Point {
    Point {
        x: Length::new(position.x),
        y: Length::new(position.y),
    }
}

pub fn point_to_position(point: Point) -> Position {
    Position {
        x: point.x.inner(),
        y: point.y.inner(),
    }
}

pub fn ratatui_to_size(size: RatatuiSize) -> Size {
    Size {
        width: Length::new(size.width),
        height: Length::new(size.height),
    }
}

pub fn size_to_ratatui(size: Size) -> RatatuiSize {
    RatatuiSize {
        width: size.width.inner(),
        height: size.height.inner(),
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

pub fn approximate_colour(colour: Colour) -> Color {
    // TODO: support lower bit-depth colours
    Color::Rgb(colour.red, colour.green, colour.blue)
}
