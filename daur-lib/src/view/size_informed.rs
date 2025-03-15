use crate::app::Action;
use crate::ui::{Point, Rectangle, Size};
use crate::view::View;
use crossterm::event::MouseButton;
use derive_more::Debug;
use ratatui::buffer::Buffer;

/// A size-informed view, i.e. one that need information about its size before rendering
#[derive(Debug)]
pub struct SizeInformed<'generator, Child> {
    #[debug(ignore)]
    /// The function for generating the view
    pub generator: Box<dyn Fn(Size) -> Child + 'generator>,
}

impl<Child> SizeInformed<'_, Child> {
    /// Constructs a new size-informed view
    pub fn new<'generator, F: Fn(Size) -> Child + 'generator>(
        f: F,
    ) -> SizeInformed<'generator, Child> {
        SizeInformed {
            generator: Box::new(f),
        }
    }
}

impl<Child: View> View for SizeInformed<'_, Child> {
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
