use crate::ui::{Point, Rectangle, Vector};
use crate::view::context::Menu;
use crate::{Action, HoldableObject, UserInterface, View};
use arcstr::ArcStr;
use std::cmp::max;

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
    pub fn grabbable<F: Fn(Rectangle, Point) -> Option<HoldableObject> + Send + Sync + 'static>(
        self,
        generator: F,
    ) -> Self {
        View::Grabbable {
            object: Box::new(generator),
            view: Box::new(self),
        }
    }

    /// Makes the view scrollable.
    pub fn scrollable(self, action: fn(Vector) -> Action) -> Self {
        View::Scrollable {
            action,
            view: Box::new(self),
        }
    }

    /// Puts a title on the view.
    pub fn titled(self, title: ArcStr) -> Self {
        View::Titled {
            title,
            highlighted: false,
            view: Box::new(self),
        }
    }

    /// Puts a title on the view where the title influences the [minimum size](View::minimum_size).
    pub fn hard_titled<Ui: UserInterface>(self, title: ArcStr) -> Self {
        let mut minimum_size = self.minimum_size::<Ui>();
        minimum_size.width = max(minimum_size.width, Ui::title_width(&title, &self));
        minimum_size.height += Ui::title_height(&title, &self);

        View::Sized {
            minimum_size,
            view: Box::new(self.titled(title)),
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
        } else if let View::Titled { title, view, .. } = self {
            View::Titled {
                title,
                highlighted: thickness,
                view,
            }
        } else {
            // TODO: log that nothing happened
            self
        }
    }
}
