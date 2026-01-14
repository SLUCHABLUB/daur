//! Items pertaining to [`Context`].

use crate::convert::approximate_colour;
use daur::ui::Colour;
use daur::ui::Point;
use daur::ui::Rectangle;
use daur::ui::Size;
use daur::view;
use ratatui::widgets::canvas;
use ratatui::widgets::canvas::Points;

/// A wrapper of [ratatui's canvas context](canvas::Context) to implement [daur's canvas context](view::Context).
pub(crate) struct Context<'reference, 'context> {
    /// The underlying ratatui context.
    pub context: &'reference mut canvas::Context<'context>,
    /// The size of the canvas.
    pub size: Size,
}

impl view::Context for Context<'_, '_> {
    fn size(&self) -> Size {
        self.size
    }

    fn draw_point(&mut self, point: Point, colour: Colour) {
        let point = (f64::from(point.x.pixels), f64::from(point.y.pixels));
        self.context.draw(&Points {
            coords: &[point],
            color: approximate_colour(colour),
        });
    }

    fn draw_rectangle(&mut self, rectangle: Rectangle, colour: Colour) {
        // TODO: does this fill the rectangle?
        self.context.draw(&canvas::Rectangle {
            x: f64::from(rectangle.position.x.pixels),
            y: f64::from(rectangle.position.y.pixels),
            width: f64::from(rectangle.size.width.pixels),
            height: f64::from(rectangle.size.height.pixels),
            color: approximate_colour(colour),
        });
    }
}
