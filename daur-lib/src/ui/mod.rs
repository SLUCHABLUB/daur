//! Types pertaining to [`UserInterface`].

mod grid;
mod length;
mod mapping;
mod offset;
mod point;
mod rectangle;
mod size;
mod vector;

use crate::View;
use crate::popup::Id;
use arcstr::ArcStr;
pub use grid::Grid;
pub use length::{Length, NonZeroLength};
pub use mapping::Mapping;
pub use offset::Offset;
pub use point::Point;
pub use rectangle::Rectangle;
pub use size::Size;
pub use vector::Vector;

/// A user interface for the DAW.
#[doc(hidden)]
pub trait UserInterface {
    /// Exits the DAW.
    ///
    /// It is OK for implementations not to do anything or restart when this is run.
    /// This may be the case if the application, for example, can't close itself.
    fn exit(&self);

    /// A [RAII](wikipedia.org/wiki/RAII) handle to a popup.
    type PopupHandle;

    /// Opens a popup.
    fn open_popup(&self, title: ArcStr, view: View, id: Id) -> Self::PopupHandle;

    /// Closes a popup.
    fn close_popup(&self, handle: Self::PopupHandle);
}
