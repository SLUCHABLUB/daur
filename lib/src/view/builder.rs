//! Implementations of builder methods for [`View`].

use crate::Holdable;
use crate::Selectable;
use crate::View;
use crate::app::Action;
use crate::ui::Length;
use crate::ui::Vector;
use crate::ui::relative;
use crate::view::OnClick;
use crate::view::RenderArea;
use crate::view::context::Menu;
use arcstr::ArcStr;

impl View {
    /// Puts a border around the view.
    pub fn bordered(self) -> View {
        View::Bordered {
            title: None,
            thick: false,
            view: Box::new(self),
        }
    }

    /// Puts a border around the view.
    pub fn bordered_with_thickness(self, thickness: bool) -> View {
        View::Bordered {
            title: None,
            thick: thickness,
            view: Box::new(self),
        }
    }

    /// Puts a border around the view.
    pub fn bordered_with_title(self, title: ArcStr) -> View {
        View::Bordered {
            title: Some(title),
            thick: false,
            view: Box::new(self),
        }
    }

    /// Puts a border around the view.
    pub fn bordered_with_title_and_thickness(self, title: ArcStr, thickness: bool) -> View {
        View::Bordered {
            title: Some(title),
            thick: thickness,
            view: Box::new(self),
        }
    }

    /// Turns the view into a button.
    pub fn on_click(self, on_click: OnClick) -> View {
        View::Clickable {
            on_click,
            view: Box::new(self),
        }
    }

    /// Adds a context menu to the widget.
    pub fn contextual(self, menu: Menu) -> View {
        View::Contextual {
            menu,
            view: Box::new(self),
        }
    }

    /// Adds a grabbable object to the view.
    pub fn grabbable<F: Fn(RenderArea) -> Option<Holdable> + Send + Sync + 'static>(
        self,
        generator: F,
    ) -> View {
        View::Grabbable {
            object: Box::new(generator),
            view: Box::new(self),
        }
    }

    /// Adds a function that accepts droppable objects to the view.
    pub fn object_accepting<
        F: Fn(Holdable, RenderArea) -> Option<Action> + Send + Sync + 'static,
    >(
        self,
        dropper: F,
    ) -> View {
        View::ObjectAcceptor {
            drop: Box::new(dropper),
            view: Box::new(self),
        }
    }

    /// Positions the view in a rectangle.
    pub fn positioned(self, at: relative::Rectangle) -> View {
        self.quoted_2d(at.size).positioned(at.position)
    }

    /// Offsets the view along the x-axis.
    pub fn x_positioned(self, offset: Length) -> View {
        self.fill_remaining().x_positioned(offset)
    }

    /// Makes the view scrollable.
    pub fn scrollable(self, action: fn(Vector) -> Action) -> View {
        View::Scrollable {
            action,
            view: Box::new(self),
        }
    }

    /// Makes the view selectable.
    pub fn selectable(self, item: Selectable) -> View {
        View::Selectable {
            item,
            view: Box::new(self),
        }
    }
}
