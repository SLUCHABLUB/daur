use crate::convert::approximate_colour;
use daur::ui::{Colour, Point, Rectangle, Size};
use daur::view;
use ratatui::widgets::canvas;
use ratatui::widgets::canvas::Points;

pub(crate) struct Context<'reference, 'context> {
    context: &'reference mut canvas::Context<'context>,
    size: Size,
}

impl<'reference, 'context> Context<'reference, 'context> {
    pub(crate) fn new(context: &'reference mut canvas::Context<'context>, size: Size) -> Self {
        Context { context, size }
    }
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
