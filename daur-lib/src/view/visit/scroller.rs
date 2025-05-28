use crate::HoldableObject;
use crate::app::{Action, Actions};
use crate::ui::{Colour, Length, Point, Rectangle, ThemeColour, Vector};
use crate::view::context::Menu;
use crate::view::visit::Visitor;
use crate::view::{Alignment, DropAction, OnClick, Painter, SelectableItem};
use std::num::NonZeroU64;

/// A visitor that scrolls (moves) objects.
#[derive(Debug)]
pub struct Scroller<'actions> {
    /// The position of the mouse when the view was scrolled.
    position: Point,
    actions: &'actions mut Actions,
    /// The offset by which the scrolled view(s) should be moved.
    offset: Vector,
}

impl<'actions> Scroller<'actions> {
    /// Constructs a new scroller that scrolls (moves) views at a position by an offset.  
    pub fn new(position: Point, offset: Vector, actions: &'actions mut Actions) -> Self {
        Scroller {
            position,
            actions,
            offset,
        }
    }
}

impl Visitor for Scroller<'_> {
    fn visit_border(&mut self, _: Rectangle, _: bool) {}

    fn visit_canvas(&mut self, _: Rectangle, _: Colour, _: &Painter) {}

    fn visit_clickable(&mut self, _: Rectangle, _: &OnClick) {}

    fn visit_contextual(&mut self, _: Rectangle, _: &Menu) {}

    fn visit_cursor_window(&mut self, _: Rectangle, _: Length) {}

    fn visit_grabbable(&mut self, _: Rectangle, _: HoldableObject) {}

    fn visit_object_acceptor(&mut self, _: Rectangle, _: &DropAction) {}

    fn visit_rule(&mut self, _: Rectangle, _: isize, _: NonZeroU64) {}

    fn visit_selectable(&mut self, _: Rectangle, _: SelectableItem) {}

    fn visit_selection_box(&mut self, _: Rectangle) {}

    fn visit_scrollable(&mut self, area: Rectangle, action: fn(Vector) -> Action) {
        if area.contains(self.position) {
            self.actions.push(action(self.offset));
        }
    }

    fn visit_solid(&mut self, _: Rectangle, _: ThemeColour) {}

    fn visit_text(&mut self, _: Rectangle, _: &str, _: Alignment) {}

    fn visit_titled(&mut self, _: Rectangle, _: &str, _: bool) {}
}
