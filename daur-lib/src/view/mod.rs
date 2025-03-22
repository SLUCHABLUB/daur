//! The UI of daur is based on views, based on the system used by the `ratatui` crate

mod alignment;
mod button;
mod canvas;
mod cursor;
mod direction;
mod feed;
mod quotum;
mod ruler;
mod text;

pub mod multi;
pub mod piano_roll;
pub mod single;

pub use alignment::Alignment;
pub use button::OnClick;
pub use canvas::Context;
pub use cursor::cursor_window;
pub use direction::Direction;
pub use feed::feed;
pub use quotum::{Quotated, Quotum};
pub use ruler::ruler;
pub use text::ToText;

use crate::context::Menu;
use crate::ui::{Length, Size};
use crate::{ArcCell, Colour};
use arcstr::ArcStr;
use derive_more::Debug;
use itertools::Itertools as _;
use std::cmp::max;
use std::num::NonZeroU32;
use std::path::Path;
use std::sync::Arc;

type Painter = dyn Fn(&mut dyn Context) + Send + Sync;

/// A UI element.
#[doc(hidden)]
#[must_use = "A view must be processed in some way"]
#[derive(Debug, Default)]
pub enum View {
    /// A view with a border and optional title.
    Bordered {
        /// The title.
        title: ArcStr,
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
    Generator(#[debug(skip)] Box<dyn Fn() -> View + Send + Sync>),
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
        cells: NonZeroU32,
    },
    /// A view that needs to know its containers size.
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
}

impl View {
    /// Puts a border around `self`.
    pub fn bordered(self) -> Self {
        self.titled(ArcStr::new())
    }

    /// Puts a titled border around `self`.
    pub fn titled(self, title: ArcStr) -> Self {
        View::Bordered {
            title,
            thick: false,
            content: Box::new(self),
        }
    }

    /// Sets the thickness if `self` matches [`View::Bordered`].
    pub fn with_thickness(self, thickness: bool) -> Self {
        if let View::Bordered {
            title,
            thick: _,
            content,
        } = self
        {
            View::Bordered {
                title,
                thick: thickness,
                content,
            }
        } else {
            self
        }
    }

    /// Constructs a [`View::Canvas`].
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

    /// Constructs a [`View::Hoverable`].
    pub fn hoverable(default: View, hovered: View) -> Self {
        View::Hoverable {
            default: Box::new(default),
            hovered: Box::new(hovered),
        }
    }

    /// Constructs a [`View::Generator`].
    pub fn generator<F: Fn() -> View + Send + Sync + 'static>(generator: F) -> Self {
        View::Generator(Box::new(generator))
    }

    /// Constructs a [`View::SizeInformed`].
    pub fn size_informed<F: Fn(Size) -> View + Send + Sync + 'static>(generator: F) -> Self {
        View::SizeInformed(Box::new(generator))
    }

    /// Constructs a stack where all views are quotated equally.
    pub fn balanced_stack<E>(direction: Direction, elements: E) -> Self
    where
        E: IntoIterator<Item = Self>,
    {
        View::Stack {
            direction,
            elements: elements.into_iter().map(View::fill_remaining).collect(),
        }
    }

    /// A stack where elements are quotated with their minimum size and spread out evenly.
    pub fn spaced_stack<E>(direction: Direction, elements: E) -> Self
    where
        E: IntoIterator<Item = Self>,
    {
        View::Stack {
            direction,
            elements: elements
                .into_iter()
                .map(View::quotated_minimally)
                .intersperse_with(|| View::Empty.fill_remaining())
                .collect(),
        }
    }

    /// Returns the minimum size required to fit the entire view.
    #[must_use]
    pub fn minimum_size(&self) -> Size {
        match self {
            // TODO: this may depend on thickness
            View::Bordered {
                title: _,
                thick: _,
                content,
            } => {
                let mut size = content.minimum_size();
                size.height += Length::DOUBLE_BORDER;
                size.width += Length::DOUBLE_BORDER;
                size
            }
            View::Button {
                on_click: _,
                content,
            } => content.minimum_size(),
            View::Canvas { .. }
            | View::CursorWindow { .. }
            | View::Empty
            | View::FileSelector { .. }
            | View::SizeInformed(_)
            | View::Solid(_) => Size::ZERO,
            View::Contextual { menu: _, view } => view.minimum_size(),
            View::Generator(generator) => generator().minimum_size(),
            View::Hoverable { default, hovered } => {
                let default = default.minimum_size();
                let hovered = hovered.minimum_size();

                Size {
                    width: max(default.width, hovered.width),
                    height: max(default.height, hovered.height),
                }
            }
            View::Layers(layers) => {
                let mut size = Size::ZERO;

                for layer in layers {
                    let layer_size = layer.minimum_size();
                    size.width = max(size.width, layer_size.width);
                    size.height = max(size.height, layer_size.height);
                }

                size
            }
            View::Rule { .. } => Size {
                width: Length::ZERO,
                height: Length::new(2),
            },
            View::Stack {
                direction,
                elements,
            } => {
                let mut parallel = Length::ZERO;
                let mut orthogonal = Length::ZERO;

                for quoted in elements {
                    let child = quoted.view.minimum_size();
                    parallel += child.parallel_to(*direction);
                    orthogonal = max(orthogonal, child.orthogonal_to(*direction));
                }

                Size::from_parallel_orthogonal(parallel, orthogonal, *direction)
            }
            View::Text {
                string,
                alignment: _,
            } => {
                let mut size = Size::ZERO;

                for line in string.lines() {
                    size.width = max(size.width, Length::string_width(line));
                }

                size.height = Length::string_height(string);

                size
            }
        }
    }
}
