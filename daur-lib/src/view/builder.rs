use crate::app::Action;
use crate::ui::{Vector, relative};
use crate::view::context::Menu;
use crate::view::{RenderArea, SelectableItem};
use crate::{HoldableObject, View};
use arcstr::ArcStr;

impl View {
    /// Puts a border around the view.
    pub fn bordered(self) -> Self {
        View::Bordered {
            thick: false,
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
    pub fn grabbable<F: Fn(RenderArea) -> Option<HoldableObject> + Send + Sync + 'static>(
        self,
        generator: F,
    ) -> Self {
        View::Grabbable {
            object: Box::new(generator),
            view: Box::new(self),
        }
    }

    /// Adds a function that accepts droppable objects to the view.
    pub fn object_accepting<
        F: Fn(HoldableObject, RenderArea) -> Option<Action> + Send + Sync + 'static,
    >(
        self,
        dropper: F,
    ) -> Self {
        View::ObjectAcceptor {
            drop: Box::new(dropper),
            view: Box::new(self),
        }
    }

    /// Positions the view in a rectangle.
    pub fn positioned(self, at: relative::Rectangle) -> Self {
        self.quotated_2d(at.size).positioned(at.position)
    }

    /// Makes the view scrollable.
    pub fn scrollable(self, action: fn(Vector) -> Action) -> Self {
        View::Scrollable {
            action,
            view: Box::new(self),
        }
    }

    /// Makes the view selectable.
    pub fn selectable(self, item: SelectableItem) -> Self {
        View::Selectable {
            item,
            view: Box::new(self),
        }
    }

    /// Puts a title on the view.
    pub fn titled(self, title: ArcStr) -> Self {
        View::Titled {
            title,
            highlighted: false,
            croppable: true,
            view: Box::new(self),
        }
    }

    /// Puts a title on the view where the title influences the [minimum size](View::minimum_size).
    pub fn titled_non_cropping(self, title: ArcStr) -> Self {
        View::Titled {
            title,
            highlighted: false,
            croppable: false,
            view: Box::new(self),
        }
    }

    /// Sets the border thickness if the view is [bordered](View::Bordered).
    ///
    /// Also sets highlights the title if the view is [titled](View::Titled).
    pub fn with_thickness(self, thickness: bool) -> Self {
        if let View::Bordered { view, .. } = self {
            View::Bordered {
                thick: thickness,
                view,
            }
        } else if let View::Titled {
            title,
            view,
            croppable,
            ..
        } = self
        {
            View::Titled {
                title,
                highlighted: thickness,
                view,
                croppable,
            }
        } else {
            // TODO: log that nothing happened
            self
        }
    }
}
