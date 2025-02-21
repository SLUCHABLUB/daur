use crate::app::Action;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::length::size::Size;
use crate::popup::Popup;
use crate::widget::has_size::HasSize;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use educe::Educe;
use ratatui::buffer::Buffer;
use std::sync::Weak;

/// A simple button
#[derive(Clone, Educe)]
#[educe(Eq, PartialEq)]
pub struct Terminating<Child> {
    pub child: Child,
    /// The id of the containing popup
    #[educe(Eq(ignore))]
    pub popup: Weak<Popup>,
}

impl<Child> Terminating<Child> {
    pub fn popup(&self) -> Weak<Popup> {
        Weak::clone(&self.popup)
    }
}

// TODO: use injective
impl<Child: Widget> Widget for Terminating<Child> {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        self.child.render(area, buf, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        self.child.click(area, button, position, actions);
        if button == MouseButton::Left {
            actions.push(Action::ClosePopup(self.popup()));
        }
    }
}

impl<Child: Widget> Widget for &Terminating<Child> {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        (*self).render(area, buf, mouse_position);
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        (*self).click(area, button, position, actions);
    }
}

impl<Child: HasSize> HasSize for Terminating<Child> {
    fn size(&self) -> Size {
        self.child.size()
    }
}

impl<Child: HasSize> HasSize for &Terminating<Child> {
    fn size(&self) -> Size {
        (*self).size()
    }
}
