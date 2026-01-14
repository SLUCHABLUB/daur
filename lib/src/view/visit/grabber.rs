//! Items pertaining to [`Grabber`].

use crate::Holdable;
use crate::Selectable;
use crate::app::Action;
use crate::app::Actions;
use crate::ui::Colour;
use crate::ui::Length;
use crate::ui::Point;
use crate::ui::Rectangle;
use crate::ui::ThemeColour;
use crate::ui::Vector;
use crate::view::Alignment;
use crate::view::DropAction;
use crate::view::OnClick;
use crate::view::Painter;
use crate::view::context::Menu;
use crate::view::visit::Visitor;
use std::num::NonZeroU64;

/// A visitor that grabs objects.
#[derive(Debug)]
pub struct Grabber<'actions> {
    /// The action queue to add actions to.
    actions: &'actions mut Actions,
    /// The position at which the view is to be grabbed.
    position: Point,
}

impl<'actions> Grabber<'actions> {
    /// Constructs a new grabber.
    pub fn new(position: Point, actions: &'actions mut Actions) -> Self {
        Grabber { actions, position }
    }
}

impl Visitor for Grabber<'_> {
    fn reverse_layer_order() -> bool {
        true
    }

    fn visit_border(&mut self, _: Rectangle, _: Option<&str>, _: bool) {}

    fn visit_canvas(&mut self, _: Rectangle, _: Colour, _: &Painter) {}

    fn visit_clickable(&mut self, _: Rectangle, _: &OnClick) {}

    fn visit_contextual(&mut self, _: Rectangle, _: &Menu) {}

    fn visit_cursor_window(&mut self, _: Rectangle, _: Length) {}

    fn visit_grabbable(&mut self, area: Rectangle, object: Holdable) {
        if area.contains(self.position) {
            self.actions.push(Action::PickUp(object));
        }
    }

    fn visit_object_acceptor(&mut self, _: Rectangle, _: &DropAction) {}

    fn visit_rule(&mut self, _: Rectangle, _: usize, _: NonZeroU64, _: Length, _: Length) {}

    fn visit_selectable(&mut self, _: Rectangle, _: Selectable) {}

    fn visit_selection_box(&mut self, _: Rectangle) {}

    fn visit_scrollable(&mut self, _: Rectangle, _: fn(Vector) -> Action) {}

    fn visit_solid(&mut self, _: Rectangle, _: ThemeColour) {}

    fn visit_text(&mut self, _: Rectangle, _: &str, _: Alignment) {}

    fn visit_title_bar(&mut self, _: Rectangle, _: &str, _: bool) {}
}
