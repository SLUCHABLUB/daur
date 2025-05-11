//! Types pertaining to [`View`].

pub mod context;
pub mod multi;
pub mod single;
pub mod visit;

mod alignment;
mod axis;
mod builder;
mod button;
mod canvas;
mod constructors;
mod cursor_window;
mod feed;
mod file_selector;
mod minimum_size;
mod quotum;
mod ruler;
mod text;

pub use alignment::Alignment;
pub use axis::Axis;
pub use button::OnClick;
pub use canvas::Context;
pub use cursor_window::CursorWindow;
pub use file_selector::file_selector;
pub use quotum::{Quotated, Quotum};
pub use ruler::ruler;
pub use text::ToText;

pub(crate) use feed::feed;

use crate::Action;
use crate::app::HoldableObject;
use crate::ui::{Colour, Point, Rectangle, Size, Vector};
use crate::view::context::Menu;
use alloc::sync::Arc;
use arcstr::ArcStr;
use core::num::NonZeroU64;
use derive_more::Debug;

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
        view: Box<Self>,
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
    /// A clickable view.
    Clickable {
        /// The action to take when the button is clicked
        on_click: OnClick,
        /// The default label for the button
        view: Box<Self>,
    },
    /// A view with a custom context-menu.
    Contextual {
        /// The context menu.
        menu: Menu,
        /// The view.
        view: Box<View>,
    },
    /// A view with a musical context.
    CursorWindow(CursorWindow),
    /// An empty (transparent) view.
    #[default]
    Empty,
    /// A function that generates a view.
    Generator(#[debug(skip)] Box<Generator>),
    /// A view that can be grabbed.
    Grabbable {
        /// The grabbed object.
        #[debug(skip)]
        object: Box<dyn Fn(Rectangle, Point) -> Option<HoldableObject> + Send + Sync + 'static>,
        /// The view.
        view: Box<View>,
    },
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
    /// A scrollable view.
    Scrollable {
        /// The action to take when scrolled.
        action: fn(Vector) -> Action,
        /// The view.
        view: Box<View>,
    },
    /// A view that needs to know its container's size.
    SizeInformed(#[debug(skip)] Box<dyn Fn(Size) -> View + Send + Sync>),
    /// A solid colour.
    Solid(Colour),
    /// A stack of views.
    Stack {
        /// The axis along which the elements are laid out.
        axis: Axis,
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
        /// Whether the title is allowed to be cropped.
        croppable: bool,
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
