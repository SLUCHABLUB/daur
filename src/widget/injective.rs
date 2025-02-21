use crate::app::Action;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::length::size::Size;
use crate::widget::has_size::HasSize;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;

pub trait Injective {
    type Visual: Widget;

    fn visual(&self) -> Self::Visual;

    fn inject(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    );
}

impl<T: Injective> Widget for T {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        self.visual().render(area, buf, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        self.inject(area, button, position, actions);
        self.visual().click(area, button, position, actions);
    }
}

impl<T: Injective> HasSize for T
where
    T::Visual: HasSize,
{
    fn size(&self) -> Size {
        self.visual().size()
    }
}
