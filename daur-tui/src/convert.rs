use daur::Colour;
use daur::ui::{Length, Point, Rectangle, Size};
use ratatui::layout::{Position, Rect, Size as RatatuiSize};
use ratatui::style::Color;
use saturating_cast::SaturatingCast as _;

fn u16_to_length(chars: u16) -> Length {
    Length { pixels: chars }
}

pub fn length_to_u16(length: Length) -> u16 {
    length.pixels.saturating_cast()
}

pub fn position_to_point(position: Position) -> Point {
    Point {
        x: u16_to_length(position.x),
        y: u16_to_length(position.y),
    }
}

pub fn to_size(size: RatatuiSize) -> Size {
    Size {
        width: u16_to_length(size.width),
        height: u16_to_length(size.height),
    }
}

pub fn rect_to_rectangle(rect: Rect) -> Rectangle {
    Rectangle {
        position: position_to_point(rect.as_position()),
        size: to_size(rect.as_size()),
    }
}

pub fn rectangle_to_rect(rectangle: Rectangle) -> Rect {
    Rect {
        x: length_to_u16(rectangle.position.x),
        y: length_to_u16(rectangle.position.y),
        width: length_to_u16(rectangle.size.width),
        height: length_to_u16(rectangle.size.height),
    }
}

pub fn approximate_colour(colour: Colour) -> Color {
    // TODO: support lower bit-depth colours
    Color::Rgb(colour.red, colour.green, colour.blue)
}
