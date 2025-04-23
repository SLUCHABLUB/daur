//! Types pertaining to [`View`].

pub mod context;
pub mod multi;
pub mod piano_roll;
pub mod single;

mod alignment;
mod button;
mod canvas;
mod clicker;
mod cursor;
mod direction;
mod feed;
mod minimum_size;
mod quotum;
mod ruler;
mod text;
mod visit;

pub use alignment::Alignment;
pub use button::OnClick;
pub use canvas::Context;
pub use clicker::Clicker;
pub use cursor::cursor_window;
pub use direction::Direction;
pub use feed::feed;
pub use quotum::{Quotated, Quotum};
pub use ruler::ruler;
pub use text::ToText;
pub use visit::Visitor;

use crate::ui::{Length, Rectangle, Size};
use crate::view::context::Menu;
use crate::view::minimum_size::minimum_size;
use crate::{ArcCell, Colour, Ratio, UserInterface};
use arcstr::ArcStr;
use derive_more::Debug;
use itertools::Itertools as _;
use std::cmp::max;
use std::num::NonZeroU64;
use std::path::Path;
use std::sync::Arc;

/// A function for painting a canvas.
pub type Painter = dyn Fn(&mut dyn Context) + Send + Sync;
/// A function for generating a view.
pub type Generator = dyn Fn() -> View + Send + Sync;

/// A UI element.
#[doc(hidden)]
#[must_use = "A view must be processed in some way"]
#[derive(Debug, Default)]
pub enum View {
    /// A view with a border.
    Bordered {
        /// Whether the border is **thick**.
        thick: bool,
        /// The bordered view.
        content: Box<Self>,
    },
    /// A clickable view.
    Button {
        /// The action to take when the button is clicked
        on_click: OnClick,
        /// The default label for the button
        content: Box<Self>,
    },
    /// A canvas on which stuff can be drawn.
    /// See [`Context`].
    Canvas {
        /// The background colour.
        background: Colour,
        /// The function that paints the canvas.
        #[debug(skip)]
        painter: Box<Painter>,
    },
    /// A view with a custom context-menu.
    Contextual {
        /// The context menu.
        menu: Menu,
        /// The view.
        view: Box<View>,
    },
    /// A view with a musical context.
    CursorWindow {
        /// How far from the left the cursor is positioned.
        offset: Length,
    },
    /// An empty (transparent) view.
    #[default]
    Empty,
    /// A view into the file system.
    FileSelector {
        /// The currently selected file.
        selected_file: Arc<ArcCell<Path>>,
    },
    /// A function that generates a view.
    Generator(#[debug(skip)] Box<Generator>),
    /// A view that whose appearance changes when hovered.
    Hoverable {
        /// The view to use when not hovered.
        default: Box<Self>,
        /// The view to use when hovered.
        hovered: Box<Self>,
    },
    /// Multiple views layered on each other.
    Layers(Vec<Self>),
    /// A rule of a ruler.
    Rule {
        /// The display-index of the rule.
        index: isize,
        /// The number of cells (the number of markings - 1).
        cells: NonZeroU64,
    },
    /// A view with a custom minimum size
    Sized { view: Box<View>, minimum_size: Size },
    /// A view that needs to know its container's size.
    SizeInformed(#[debug(skip)] Box<dyn Fn(Size) -> View + Send + Sync>),
    /// A solid colour.
    Solid(Colour),
    /// A stack of views.
    Stack {
        /// The direction in which the elements are laid out.
        direction: Direction,
        /// The stacked views.
        elements: Vec<Quotated>,
    },
    /// Some text.
    Text {
        /// The text.
        string: ArcStr,
        /// How the text should be aligned.
        alignment: Alignment,
    },
    /// A view with a title bar.
    Titled {
        /// The title.
        title: ArcStr,
        /// Whether the title is highlighted.
        highlighted: bool,
        /// The view.
        view: Box<View>,
    },
    /// A window which takes up part of the screen.
    Window {
        /// The area of the window.
        area: Rectangle,
        /// The view of the window.
        view: Arc<View>,
    },
}

impl View {
    /// Puts a border around the view.
    pub fn bordered(self) -> Self {
        View::Bordered {
            thick: false,
            content: Box::new(self),
        }
    }

    /// Puts a titled border around the view.
    pub fn titled(self, title: ArcStr) -> Self {
        View::Titled {
            title,
            highlighted: false,
            view: Box::new(self),
        }
    }

    /// Sets the border thickness if the view is [bordered](View::Bordered).
    ///
    /// Also sets highlights the title if the view is [titled](View::Titled).
    pub fn with_thickness(self, thickness: bool) -> Self {
        if let View::Bordered { thick: _, content } = self {
            View::Bordered {
                thick: thickness,
                content,
            }
        } else if let View::Titled {
            title,
            highlighted: _,
            view,
        } = self
        {
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

    /// Constructs a new [canvas](View::Canvas).
    pub fn canvas<Painter>(background: Colour, painter: Painter) -> View
    where
        Painter: Fn(&mut dyn Context) + Send + Sync + 'static,
    {
        View::Canvas {
            background,
            painter: Box::new(painter),
        }
    }

    /// Adds a context menu to the widget.
    pub fn context(self, menu: Menu) -> View {
        View::Contextual {
            menu,
            view: Box::new(self),
        }
    }

    /// Constructs a new [hoverable](View::Hoverable) view.
    pub fn hoverable(default: View, hovered: View) -> Self {
        View::Hoverable {
            default: Box::new(default),
            hovered: Box::new(hovered),
        }
    }

    /// Constructs a new view from a [generator](View::Generator).
    pub fn generator<F: Fn() -> View + Send + Sync + 'static>(generator: F) -> Self {
        View::Generator(Box::new(generator))
    }

    /// Constructs a new [size-informed](View::SizeInformed) view.
    pub fn size_informed<F: Fn(Size) -> View + Send + Sync + 'static>(generator: F) -> Self {
        View::SizeInformed(Box::new(generator))
    }

    /// Constructs a new [stack](View::Stack) where all views are quotated equally.
    pub fn balanced_stack<Ui: UserInterface, E: IntoIterator<Item = Self>>(
        direction: Direction,
        elements: E,
    ) -> Self {
        let iter = elements.into_iter();
        let mut elements = Vec::new();
        let mut minimum_size = Size::ZERO;
        let mut count: u64 = 0;

        for element in iter {
            let size = element.minimum_size::<Ui>();
            minimum_size.width = max(minimum_size.width, size.width);
            minimum_size.height = max(minimum_size.height, size.height);
            count = count.saturating_add(1);

            elements.push(element.fill_remaining());
        }

        let count = Ratio::integer(count);

        match direction {
            Direction::Up | Direction::Down => minimum_size.height *= count,
            Direction::Left | Direction::Right => minimum_size.width *= count,
        }

        View::Sized {
            view: Box::new(View::Stack {
                direction,
                elements,
            }),
            minimum_size,
        }
    }

    /// Constructs a new [stack](View::Stack) where elements are quotated with their minimum size and spread out evenly.
    pub fn spaced_stack<Ui: UserInterface, E: IntoIterator<Item = Self>>(
        direction: Direction,
        elements: E,
    ) -> Self {
        View::Stack {
            direction,
            elements: elements
                .into_iter()
                .map(View::quotated_minimally::<Ui>)
                .intersperse_with(|| View::Empty.fill_remaining())
                .collect(),
        }
    }

    /// Returns the minimum size required to fit the entire view.
    #[must_use]
    pub fn minimum_size<Ui: UserInterface>(&self) -> Size {
        minimum_size::<Ui>(self)
    }
}
