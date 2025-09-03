use crate::app::{Action, Actions};
use crate::ui::{Colour, Length, Point, Rectangle, ThemeColour, Vector};
use crate::view::context::Menu;
use crate::view::visit::Visitor;
use crate::view::{Alignment, DropAction, OnClick, Painter, RenderArea};
use crate::{Holdable, Selectable};
use std::num::NonZeroU64;

/// A [visitor](Visitor) for dropping an [object](Holdable).
#[derive(Debug)]
pub struct Dropper<'actions> {
    actions: &'actions mut Actions,
    object: Holdable,
    position: Point,
}

impl<'actions> Dropper<'actions> {
    /// Constructs a new dropper.
    pub fn new(object: Holdable, position: Point, actions: &'actions mut Actions) -> Self {
        actions.push(Action::LetGo);

        Dropper {
            actions,
            object,
            position,
        }
    }
}

impl Visitor for Dropper<'_> {
    fn visit_border(&mut self, _: Rectangle, _: Option<&str>, _: bool) {}

    fn visit_canvas(&mut self, _: Rectangle, _: Colour, _: &Painter) {}

    fn visit_clickable(&mut self, _: Rectangle, _: &OnClick) {}

    fn visit_contextual(&mut self, _: Rectangle, _: &Menu) {}

    fn visit_cursor_window(&mut self, _: Rectangle, _: Length) {}

    fn visit_grabbable(&mut self, _: Rectangle, _: Holdable) {}

    fn visit_object_acceptor(&mut self, area: Rectangle, action: &DropAction) {
        if area.contains(self.position)
            && let Some(action) = action(
                self.object,
                RenderArea {
                    area,
                    mouse_position: self.position,
                },
            )
        {
            self.actions.push(action);
        }
    }

    fn visit_rule(&mut self, _: Rectangle, _: usize, _: NonZeroU64, _: Length, _: Length) {}

    fn visit_selectable(&mut self, area: Rectangle, item: Selectable) {
        let Holdable::SelectionBox { start } = self.object else {
            return;
        };
        let end = self.position;

        let selection_box = Rectangle::containing_both(start, end);

        if selection_box.intersects(area) {
            self.actions.push(Action::Select(item));
        }
    }

    fn visit_selection_box(&mut self, _: Rectangle) {}

    fn visit_scrollable(&mut self, _: Rectangle, _: fn(Vector) -> Action) {}

    fn visit_solid(&mut self, _: Rectangle, _: ThemeColour) {}

    fn visit_text(&mut self, _: Rectangle, _: &str, _: Alignment) {}

    fn visit_title_bar(&mut self, _: Rectangle, _: &str, _: bool) {}
}
