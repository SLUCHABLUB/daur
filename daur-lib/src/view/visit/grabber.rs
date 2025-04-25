use crate::UserInterface;
use crate::app::HoldableObject;
use crate::ui::{Colour, Length, Point, Rectangle};
use crate::view::context::Menu;
use crate::view::visit::Visitor;
use crate::view::{Alignment, OnClick, Painter};
use std::marker::PhantomData;
use std::num::NonZeroU64;

/// A visitor that grabs objects.
#[must_use = "run `Grabber::object`"]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Grabber<Ui> {
    object: Option<HoldableObject>,
    position: Point,
    phantom: PhantomData<Ui>,
}

impl<Ui> Grabber<Ui> {
    /// Constructs a new grabber.
    pub fn new(position: Point) -> Self {
        Grabber {
            object: None,
            position,
            phantom: PhantomData,
        }
    }

    /// Grabs the grabbers grabbed object.
    #[must_use]
    pub fn object(self) -> Option<HoldableObject> {
        self.object
    }
}

impl<Ui: UserInterface> Visitor for Grabber<Ui> {
    type Ui = Ui;

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
            self.object = self.object.or(Some(object));
        }
    }

    fn visit_rule(&mut self, _: Rectangle, _: isize, _: NonZeroU64) {}

    fn visit_solid(&mut self, _: Rectangle, _: Colour) {}

    fn visit_text(&mut self, _: Rectangle, _: &str, _: Alignment) {}

    fn visit_titled(&mut self, _: Rectangle, _: &str, _: bool) {}

    fn visit_window(&mut self, _: Rectangle) {}
}
