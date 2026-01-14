//! Items pertaining to [canvases](View::Canvas).

use crate::ui::Colour;
use crate::ui::Length;
use crate::ui::Point;
use crate::ui::Rectangle;
use crate::ui::Size;

/// A [canvas](crate::View::Canvas) context, used for drawing.
pub trait Context {
    /// Returns the width and height of the canvas.
    fn size(&self) -> Size;

    /// Draws a single point / pixel on the canvas.
    /// If the point is not within the bound of the canvas, the call should be ignored.
    fn draw_point(&mut self, point: Point, colour: Colour);

    /// Draws a (filled in) rectangle on the canvas.
    /// If any part of the rectangle is not withing the bounds of the canvas,
    /// the rectangle should be cropped.
    fn draw_rectangle(&mut self, rectangle: Rectangle, colour: Colour) {
        let x_range =
            rectangle.position.x.pixels..(rectangle.position.x + rectangle.size.width).pixels;
        let y_range =
            rectangle.position.y.pixels..(rectangle.position.y + rectangle.size.height).pixels;

        for x in x_range {
            for y in y_range.clone() {
                let x = Length { pixels: x };
                let y = Length { pixels: y };

                self.draw_point(Point { x, y }, colour);
            }
        }
    }
}
