use crate::colour::Colour;
use crate::ui::{Point, Rectangle, Size};

/// A [canvas](crate::View::Canvas) context, used for drawing.
pub trait Context {
    /// Returns the width and height of the canvas.
    fn size(&self) -> Size;

    /// Draws a single point / pixel on the canvas.
    /// If the point is not within the bound of the canvas the call should be ignored.
    fn draw_point(&mut self, point: Point, colour: Colour);

    /// Draws a (filled in) rectangle on the canvas.
    /// If any part of the rectangle is not withing the bounds of the canvas,
    /// the rectangle should be cropped.
    fn draw_rectangle(&mut self, rectangle: Rectangle, colour: Colour);
}
