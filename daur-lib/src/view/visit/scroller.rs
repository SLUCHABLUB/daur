use crate::ui::{Colour, Length, Point, Rectangle, Vector};
use crate::view::context::Menu;
use crate::view::visit::Visitor;
use crate::view::{Alignment, OnClick, Painter};
use crate::{Action, HoldableObject};
use std::num::NonZeroU64;

/// A visitor that scrolls (moves) objects.
#[must_use = "run `Scroller::actions`"]
#[derive(Debug)]
pub struct Scroller {
    /// The position of the mouse when the view was scrolled.
    position: Point,
    actions: Vec<Action>,
    /// The offset by which the scrolled view(s) should be moved.
    offset: Vector,
}

impl Scroller {
    /// Constructs a new scroller that scrolls (moves) views at a position by an offset.  
    pub fn new(position: Point, offset: Vector) -> Scroller {
        Scroller {
            position,
            actions: Vec::new(),
            offset,
        }
    }

    /// Extracts the actions accumulated by the scroller.
    #[must_use]
    pub fn actions(self) -> impl IntoIterator<Item = Action> {
        self.actions
    }
}

impl Visitor for Scroller {
    fn visit_border(&mut self, _: Rectangle, _: bool) {}

    fn visit_canvas(&mut self, _: Rectangle, _: Colour, _: &Painter) {}

    fn visit_clickable(&mut self, _: Rectangle, _: &OnClick) {}

    fn visit_contextual(&mut self, _: Rectangle, _: &Menu) {}

    fn visit_cursor_window(&mut self, _: Rectangle, _: Length) {}

    fn visit_grabbable(&mut self, _: Rectangle, _: HoldableObject) {}

    fn visit_rule(&mut self, _: Rectangle, _: isize, _: NonZeroU64) {}

    fn visit_scrollable(&mut self, area: Rectangle, action: fn(Vector) -> Action) {
        if area.contains(self.position) {
            self.actions.push(action(self.offset));
        }
    }

    fn visit_solid(&mut self, _: Rectangle, _: Colour) {}

    fn visit_text(&mut self, _: Rectangle, _: &str, _: Alignment) {}

    fn visit_titled(&mut self, _: Rectangle, _: &str, _: bool) {}

    fn visit_window(&mut self, _: Rectangle) {}
}
