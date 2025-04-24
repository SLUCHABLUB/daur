use daur::Colour;
use daur::ui::{Length, Point, Rectangle, Size};
use ratatui::layout::{Position, Rect, Size as RatatuiSize};
use ratatui::style::Color;
use saturating_cast::SaturatingCast as _;

pub const fn to_length(chars: u16) -> Length {
    Length { pixels: chars }
}

pub fn from_length(length: Length) -> u16 {
    length.pixels.saturating_cast()
}

pub fn to_point(position: Position) -> Point {
    Point {
        x: to_length(position.x),
        y: to_length(position.y),
    }
}

pub fn to_size(size: RatatuiSize) -> Size {
    Size {
        width: to_length(size.width),
        height: to_length(size.height),
    }
}

pub fn to_rectangle(rect: Rect) -> Rectangle {
    Rectangle {
        position: to_point(rect.as_position()),
        size: to_size(rect.as_size()),
    }
}

pub fn from_rectangle(rectangle: Rectangle) -> Rect {
    Rect {
        x: from_length(rectangle.position.x),
        y: from_length(rectangle.position.y),
        width: from_length(rectangle.size.width),
        height: from_length(rectangle.size.height),
    }
}

pub fn approximate_colour(colour: Colour) -> Color {
    // TODO: support lower bit-depth colours
    Color::Rgb(colour.red, colour.green, colour.blue)
}
