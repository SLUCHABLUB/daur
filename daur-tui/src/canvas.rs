use daur::ui::{Point, Rectangle, Size};
use daur::view;
use ratatui::style::Color;
use ratatui::widgets::canvas;
use ratatui::widgets::canvas::Points;

pub struct Context<'reference, 'context> {
    pub context: &'reference mut canvas::Context<'context>,
    pub size: Size,
}

impl view::Context for Context<'_, '_> {
    fn size(&self) -> Size {
        self.size
    }

    fn draw_point(&mut self, point: Point, colour: Color) {
        let point = (f64::from(point.x.inner()), f64::from(point.y.inner()));
        self.context.draw(&Points {
            coords: &[point],
            color: colour,
        });
    }

    fn draw_rectangle(&mut self, rectangle: Rectangle, colour: Color) {
        // TODO: does this fill the rectangle?
        self.context.draw(&canvas::Rectangle {
            x: f64::from(rectangle.position.x.inner()),
            y: f64::from(rectangle.position.y.inner()),
            width: f64::from(rectangle.size.width.inner()),
            height: f64::from(rectangle.size.height.inner()),
            color: colour,
        });
    }
}
