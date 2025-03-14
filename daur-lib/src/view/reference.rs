use crate::ui::{Point, Rectangle, Size};
use crate::view::{HasSize, View};
use crate::Action;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use std::ops::Deref;

// TODO: document why we cant use &T
/// A reference to a view.
#[derive(Copy, Clone, Debug)]
pub struct Ref<'lifetime, T> {
    reference: &'lifetime T,
}

impl<'lifetime, T> From<&'lifetime T> for Ref<'lifetime, T> {
    fn from(reference: &'lifetime T) -> Self {
        Ref { reference }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.reference
    }
}

impl<T: View> View for Ref<'_, T> {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        self.reference.render(area, buffer, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        self.reference.click(area, button, position, actions);
    }
}

impl<T: HasSize> HasSize for Ref<'_, T> {
    fn size(&self) -> Size {
        self.reference.size()
    }
}
