use crate::app::Action;
use crate::ui::{Point, Rectangle, Size};
use crate::widget::Widget;
use crossterm::event::MouseButton;
use educe::Educe;
use ratatui::buffer::Buffer;

/// A size-informed widget, i.e. one that need information about its size before rendering
#[derive(Educe)]
#[educe(Debug)]
pub struct SizeInformed<'generator, Child> {
    #[educe(Debug(ignore))]
    /// The function for generating the widget
    pub generator: Box<dyn Fn(Size) -> Child + 'generator>,
}

impl<Child> SizeInformed<'_, Child> {
    /// Constructs a new size-informed widget
    pub fn new<'generator, F: Fn(Size) -> Child + 'generator>(
        f: F,
    ) -> SizeInformed<'generator, Child> {
        SizeInformed {
            generator: Box::new(f),
        }
    }
}

impl<Child: Widget> Widget for SizeInformed<'_, Child> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        (self.generator)(area.size).render(area, buffer, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        (self.generator)(area.size).click(area, button, position, actions);
    }
}
