//! Types pertaining to [`View`].

pub mod context;
pub mod file;
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
mod minimum_size;
mod quotum;
mod render_area;
mod ruler;
mod text;

pub use alignment::Alignment;
pub use axis::Axis;
pub use button::OnClick;
pub use canvas::Context;
pub use cursor_window::CursorWindow;
pub use quotum::Quoted;
pub use quotum::Quoted2D;
pub use quotum::Quotum;
pub use quotum::Quotum2D;
pub use render_area::RenderArea;
pub use ruler::ruler;
pub use text::ToText;

use crate::Holdable;
use crate::Selectable;
use crate::app::Action;
use crate::ui::Colour;
use crate::ui::Length;
use crate::ui::ThemeColour;
use crate::ui::Vector;
use crate::ui::relative;
use crate::view::context::Menu;
use arcstr::ArcStr;
use arcstr::literal;
use derive_more::Debug;
use std::num::NonZeroU64;
use std::sync::Arc;

/// The label for buttons that cancel a process.
pub(crate) const CANCEL: ArcStr = literal!("cancel");
/// The label for buttons that confirm actions.
pub(crate) const CONFIRM: ArcStr = literal!("confirm");

/// A function for painting a canvas.
pub type Painter = dyn Fn(&mut dyn Context) + Send + Sync;
/// A function for getting an action when dropping an object on a view.
pub type DropAction = dyn Fn(Holdable, RenderArea) -> Option<Action> + Send + Sync;
/// A function for getting an object when grabbing a view.
pub type GrabObject = dyn Fn(RenderArea) -> Option<Holdable> + Send + Sync;
/// A function for a reactive view.
pub type Reactive = dyn Fn(RenderArea) -> View + Send + Sync;

/// A UI element.
#[must_use]
#[derive(Debug, Default)]
#[remain::sorted]
pub enum View {
    /// A view with a border and optional title.
    Bordered {
        /// An optional title.
        title: Option<ArcStr>,
        /// Whether the border is **thick**.
        thick: bool,
        /// The bordered view.
        view: Box<View>,
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
        view: Box<View>,
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
        default: Box<View>,
        /// The view to use when hovered.
        hovered: Box<View>,
    },
    /// Multiple views layered on each other.
    Layers(Vec<View>),
    /// A view on which an [object](Holdable) may be dropped.
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
        view: Box<Quoted2D>,
    },
    /// A reactive view. It needs information about the user interface to be rendered/processed.
    Reactive(#[debug(skip)] Box<Reactive>),
    /// A rule of a ruler.
    Rule {
        /// The display-index of the rule.
        index: usize,
        /// The number of cells (the number of markings - 1).
        cells: NonZeroU64,
        /// How far from the left the rule is cropped.
        left_crop: Length,
        /// The full (uncropped) width of the rule.
        width: Length,
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
        item: Selectable,
        /// The view.
        view: Box<View>,
    },
    /// A transparent/translucent selection box.
    SelectionBox,
    /// A reference counted view.
    Shared(Arc<View>),
    /// A solid colour.
    Solid(ThemeColour),
    /// A stack of views.
    Stack {
        /// The axis along which the elements are laid out.
        axis: Axis,
        /// The stacked views.
        elements: Vec<Quoted>,
    },
    /// Some text.
    Text {
        /// The text.
        string: ArcStr,
        /// How the text should be aligned.
        alignment: Alignment,
    },
    /// A standalone title bar.
    TitleBar {
        /// The title.
        title: ArcStr,
        /// Whether the title is highlighted.
        highlighted: bool,
    },
}
