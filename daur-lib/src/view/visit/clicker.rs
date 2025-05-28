use crate::app::HoldableObject;
use crate::ui::{Colour, Length, Point, Rectangle, Vector};
use crate::view::context::Menu;
use crate::view::visit::Visitor;
use crate::view::{Alignment, DropAction, OnClick, Painter, RenderArea, SelectableItem};
use crate::{Action, Actions};
use std::num::NonZeroU64;

/// A visitor for clicking a view.
#[derive(Debug)]
pub struct Clicker<'actions> {
    position: Point,
    actions: &'actions mut Actions,
    right_click: bool,
    captured: bool,
}

impl<'actions> Clicker<'actions> {
    /// A clicker using the left mouse button.
    pub fn left_click(position: Point, actions: &'actions mut Actions) -> Self {
        Clicker {
            position,
            actions,
            right_click: false,
            captured: false,
        }
    }

    /// A clicker using the right mouse button.
    pub fn right_click(position: Point, actions: &'actions mut Actions) -> Self {
        Clicker {
            position,
            actions,
            right_click: true,
            captured: false,
        }
    }

    fn should_click(&self, area: Rectangle) -> bool {
        !self.captured && area.contains(self.position)
    }
}

impl Visitor for Clicker<'_> {
    fn reverse_order() -> bool {
        true
    }

    fn visit_border(&mut self, _: Rectangle, _: bool) {}

    fn visit_canvas(&mut self, _: Rectangle, _: Colour, _: &Painter) {}

    fn visit_clickable(&mut self, area: Rectangle, on_click: &OnClick) {
        if !self.right_click && self.should_click(area) {
            on_click.run(
                RenderArea {
                    area,
                    mouse_position: self.position,
                },
                self.actions,
            );
        }
    }

    fn visit_contextual(&mut self, area: Rectangle, menu: &Menu) {
        if self.right_click && self.should_click(area) {
            self.actions.push(Action::OpenContextMenu {
                menu: menu.clone(),
                position: self.position,
            });
        }
    }

    fn visit_cursor_window(&mut self, _: Rectangle, _: Length) {}

    fn visit_grabbable(&mut self, _: Rectangle, _: HoldableObject) {
        // a click triggers on release whilst a grab triggers on press
    }

    fn visit_object_acceptor(&mut self, _: Rectangle, _: &DropAction) {}

    fn visit_rule(&mut self, _: Rectangle, _: isize, _: NonZeroU64) {}

    // TODO: select
    fn visit_selectable(&mut self, _: Rectangle, _: SelectableItem) {}

    fn visit_selection_box(&mut self, _: Rectangle) {}

    fn visit_scrollable(&mut self, _: Rectangle, _: fn(Vector) -> Action) {}

    fn visit_solid(&mut self, _: Rectangle, _: Colour) {}

    fn visit_text(&mut self, _: Rectangle, _: &str, _: Alignment) {}

    fn visit_titled(&mut self, _: Rectangle, _: &str, _: bool) {}

    fn visit_window(&mut self, area: Rectangle) {
        if area.contains(self.position) {
            self.captured = true;
        }
    }
}
