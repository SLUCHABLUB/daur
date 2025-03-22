//! Types pertaining to [`Ui`].

mod grid;
mod length;
mod mapping;
mod offset;
mod point;
mod rectangle;
mod size;
mod vector;

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
pub trait Ui {
    /// Exits the DAW.
    ///
    /// It is ok for implementations not to do anything or restart when this is run.
    /// This may be the case if the application, for example, can't close itself.
    fn exit(&self);
}
