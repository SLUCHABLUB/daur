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
use crate::view::RenderArea;
use crate::view::context::Menu;
use crate::view::visit::Visitor;
use std::num::NonZeroU64;

// TODO: make private
/// A visitor for clicking a view.
#[derive(Debug)]
pub struct Clicker<'actions> {
    position: Point,
    actions: &'actions mut Actions,
    right_click: bool,
    captured: bool,
    clear_selection: bool,
}

impl<'actions> Clicker<'actions> {
    /// A clicker using the left mouse button.
    pub fn left_click(
        position: Point,
        clear_selection: bool,
        actions: &'actions mut Actions,
    ) -> Self {
        actions.push(Action::CloseContextMenu);

        Clicker {
            position,
            actions,
            right_click: false,
            captured: false,
            clear_selection,
        }
    }

    /// A clicker using the right mouse button.
    pub fn right_click(position: Point, actions: &'actions mut Actions) -> Self {
        actions.push(Action::CloseContextMenu);

        Clicker {
            position,
            actions,
            right_click: true,
            captured: false,
            clear_selection: false,
        }
    }

    fn should_click(&self, area: Rectangle) -> bool {
        !self.captured && area.contains(self.position)
    }
}

impl Visitor for Clicker<'_> {
    fn reverse_layer_order() -> bool {
        true
    }

    fn visit_border(&mut self, _: Rectangle, _: Option<&str>, _: bool) {}

    fn visit_canvas(&mut self, area: Rectangle, _: Colour, _: &Painter) {
        if area.contains(self.position) {
            self.captured = true;
        }
    }

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

    fn visit_grabbable(&mut self, _: Rectangle, _: Holdable) {
        // a click triggers on release whilst a grab triggers on press
    }

    fn visit_object_acceptor(&mut self, _: Rectangle, _: &DropAction) {}

    fn visit_rule(&mut self, _: Rectangle, _: usize, _: NonZeroU64, _: Length, _: Length) {}

    fn visit_selectable(&mut self, area: Rectangle, item: Selectable) {
        if self.should_click(area) {
            if self.clear_selection {
                self.actions.push(Action::ClearSelection);
            }

            self.actions.push(Action::Select(item));
        }
    }

    fn visit_selection_box(&mut self, _: Rectangle) {}

    fn visit_scrollable(&mut self, _: Rectangle, _: fn(Vector) -> Action) {}

    fn visit_solid(&mut self, area: Rectangle, _: ThemeColour) {
        if area.contains(self.position) {
            self.captured = true;
        }
    }

    fn visit_text(&mut self, _: Rectangle, _: &str, _: Alignment) {}

    fn visit_title_bar(&mut self, area: Rectangle, _: &str, _: bool) {
        if area.contains(self.position) {
            self.captured = true;
        }
    }
}
