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
mod render_area;
mod ruler;
mod selectable_item;
mod text;

pub use alignment::Alignment;
pub use axis::Axis;
pub use button::OnClick;
pub use canvas::Context;
pub use cursor_window::CursorWindow;
pub use file_selector::file_selector;
pub use quotum::{Quotated, Quotated2D, Quotum, Quotum2D};
pub use render_area::RenderArea;
pub use ruler::ruler;
pub use selectable_item::SelectableItem;
pub use text::ToText;

pub(crate) use feed::feed;

use crate::HoldableObject;
use crate::app::Action;
use crate::ui::{Colour, Vector, relative};
use crate::view::context::Menu;
use arcstr::ArcStr;
use derive_more::Debug;
use std::num::NonZeroU64;
use std::sync::Arc;

/// A function for painting a canvas.
pub type Painter = dyn Fn(&mut dyn Context) + Send + Sync;
/// A function for getting an action when dropping an object on a view.
pub type DropAction = dyn Fn(HoldableObject, RenderArea) -> Option<Action> + Send + Sync;
/// A function for getting an object when grabbing a view.
pub type GrabObject = dyn Fn(RenderArea) -> Option<HoldableObject> + Send + Sync;
/// A function for a reactive view.
pub type Reactive = dyn Fn(RenderArea) -> View + Send + Sync;

/// A UI element.
#[cfg_attr(doc, doc(hidden))]
#[must_use]
#[derive(Debug, Default)]
#[remain::sorted]
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
    /// A view that can be grabbed.
    Grabbable {
        /// The grabbable object.
        #[debug(skip)]
        object: Box<GrabObject>,
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
    /// A view on which an [object](HoldableObject) may be dropped.
    ObjectAcceptor {
        /// The action to take when an object is dropped on the view.
        #[debug(skip)]
        drop: Box<DropAction>,
        /// The view.
        view: Box<View>,
    },
    /// An offset view.
    Positioned {
        /// The position of the view, relative to its parent.
        position: relative::Point,
        /// The view.
        view: Box<Quotated2D>,
    },
    /// A reactive view. It needs information about the user interface to be rendered/processed.
    Reactive(#[debug(skip)] Box<Reactive>),
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
    /// A view that can be selected.
    Selectable {
        /// The item to be selected.
        item: SelectableItem,
        /// The view.
        view: Box<Self>,
    },
    /// A transparent/translucent selection box.
    SelectionBox,
    /// A reference counted view.
    Shared(Arc<View>),
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
}
