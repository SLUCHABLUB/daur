//! Conversion function between [daur] types and [ratatui] types.

use daur::ui::Colour;
use daur::ui::Length;
use daur::ui::Point;
use daur::ui::Rectangle;
use daur::ui::Size;
use ratatui::layout::Position;
use ratatui::layout::Rect;
use ratatui::layout::Size as RatatuiSize;
use ratatui::style::Color;
use saturating_cast::SaturatingCast as _;

/// Converts a ratatui distance (per characters) to a [`Length`].
pub(crate) const fn to_length(chars: u16) -> Length {
    Length { pixels: chars }
}

/// Converts a [`Length`] to a ratatui distance (per characters).
pub(crate) fn from_length(length: Length) -> u16 {
    length.pixels.saturating_cast()
}

/// Converts a ratatui (position)[Position] to a [point](Point).
pub(crate) fn to_point(position: Position) -> Point {
    Point {
        x: to_length(position.x),
        y: to_length(position.y),
    }
}

/// Converts a ratatui [size](RatatuiSize) to a [size](Size).
pub(crate) fn to_size(size: RatatuiSize) -> Size {
    Size {
        width: to_length(size.width),
        height: to_length(size.height),
    }
}

/// Converts a ratatui "[rect](Rect)" to a [rectangle](Rectangle).
pub(crate) fn to_rectangle(rect: Rect) -> Rectangle {
    Rectangle {
        position: to_point(rect.as_position()),
        size: to_size(rect.as_size()),
    }
}

/// Converts a [rectangle](Rectangle) to a ratatui "[rect](Rect)".
pub(crate) fn from_rectangle(rectangle: Rectangle) -> Rect {
    Rect {
        x: from_length(rectangle.position.x),
        y: from_length(rectangle.position.y),
        width: from_length(rectangle.size.width),
        height: from_length(rectangle.size.height),
    }
}

/// Approximates a [colour](Colour) with a ratatui "[color](Color)".
pub(crate) fn approximate_colour(colour: Colour) -> Color {
    // TODO: support lower bit-depth colours
    let [red, green, blue] = colour.to_srgb();
    Color::Rgb(red, green, blue)
}
