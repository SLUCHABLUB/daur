use crate::app::HoldableObject;
use crate::ui::{Colour, Length, Point, Rectangle};
use crate::view::context::Menu;
use crate::view::visit::Visitor;
use crate::view::{Alignment, OnClick, Painter};
use crate::{Action, App, UserInterface};
use std::marker::PhantomData;
use std::num::NonZeroU64;

/// A visitor for clicking a view.
///
/// It accumulates [actions](Action) when visiting a view.
/// These need to be processed, for example, by running [`take_actions`](Clicker::take_actions).
#[must_use = "run `Clicker::take_actions`"]
#[derive(Clone, Debug)]
pub struct Clicker<Ui> {
    position: Point,
    actions: Vec<Action>,
    right_click: bool,
    captured: bool,
    phantom: PhantomData<Ui>,
}

impl<Ui> Clicker<Ui> {
    /// A clicker using the left mouse button.
    pub fn left_click(position: Point) -> Self {
        Clicker {
            position,
            actions: Vec::new(),
            right_click: false,
            captured: false,
            phantom: PhantomData,
        }
    }

    /// A clicker using the right mouse button.
    pub fn right_click(position: Point) -> Self {
        Clicker {
            position,
            actions: Vec::new(),
            right_click: true,
            captured: false,
            phantom: PhantomData,
        }
    }

    fn should_click(&self, area: Rectangle) -> bool {
        !self.captured && area.contains(self.position)
    }
}

impl<Ui: UserInterface> Clicker<Ui> {
    /// Takes the actions stored in the clicker.
    pub fn take_actions(self, app: &App<Ui>) {
        for action in self.actions {
            action.take(app);
        }
    }
}

impl<Ui: UserInterface> Visitor for Clicker<Ui> {
    type Ui = Ui;

    fn reverse_order() -> bool {
        true
    }

    fn visit_border(&mut self, _: Rectangle, _: bool) {}

    fn visit_canvas(&mut self, _: Rectangle, _: Colour, _: &Painter) {}

    fn visit_clickable(&mut self, area: Rectangle, on_click: &OnClick) {
        if !self.right_click && self.should_click(area) {
            let position = self.position - area.position.position();
            on_click.run(area.size, position, &mut self.actions);
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

    fn visit_rule(&mut self, _: Rectangle, _: isize, _: NonZeroU64) {}

    fn visit_solid(&mut self, _: Rectangle, _: Colour) {}

    fn visit_text(&mut self, _: Rectangle, _: &str, _: Alignment) {}

    fn visit_titled(&mut self, _: Rectangle, _: &str, _: bool) {}

    fn visit_window(&mut self, area: Rectangle) {
        if area.contains(self.position) {
            self.captured = true;
        }
    }
}
