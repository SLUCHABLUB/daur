use crate::HoldableObject;
use crate::app::{Action, Actions};
use crate::ui::{Colour, Length, Point, Rectangle, Vector};
use crate::view::context::Menu;
use crate::view::visit::Visitor;
use crate::view::{Alignment, DropAction, OnClick, Painter, SelectableItem};
use std::num::NonZeroU64;

/// A visitor that grabs objects.
#[derive(Debug)]
pub struct Grabber<'actions> {
    actions: &'actions mut Actions,
    position: Point,
}

impl<'actions> Grabber<'actions> {
    /// Constructs a new grabber.
    pub fn new(position: Point, actions: &'actions mut Actions) -> Self {
        Grabber { actions, position }
    }
}

impl Visitor for Grabber<'_> {
    fn reverse_order() -> bool {
        true
    }

    fn visit_border(&mut self, _: Rectangle, _: bool) {}

    fn visit_canvas(&mut self, _: Rectangle, _: Colour, _: &Painter) {}

    fn visit_clickable(&mut self, _: Rectangle, _: &OnClick) {}

    fn visit_contextual(&mut self, _: Rectangle, _: &Menu) {}

    fn visit_cursor_window(&mut self, _: Rectangle, _: Length) {}

    fn visit_grabbable(&mut self, area: Rectangle, object: HoldableObject) {
        if area.contains(self.position) {
            self.actions.push(Action::PickUp(object));
        }
    }

    fn visit_object_acceptor(&mut self, _: Rectangle, _: &DropAction) {}

    fn visit_rule(&mut self, _: Rectangle, _: isize, _: NonZeroU64) {}

    fn visit_selectable(&mut self, _: Rectangle, _: SelectableItem) {}

    fn visit_selection_box(&mut self, _: Rectangle) {}

    fn visit_scrollable(&mut self, _: Rectangle, _: fn(Vector) -> Action) {}

    fn visit_solid(&mut self, _: Rectangle, _: Colour) {}

    fn visit_text(&mut self, _: Rectangle, _: &str, _: Alignment) {}

    fn visit_titled(&mut self, _: Rectangle, _: &str, _: bool) {}
}
