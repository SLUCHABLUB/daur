//! Items pertaining to [`Quotum`].

mod quoted;
mod quoted_2d;
mod quotum_2d;

pub use quoted::Quoted;
pub use quoted_2d::Quoted2D;
pub use quotum_2d::Quotum2D;

use crate::ui::Length;

/// The amount of space that is allocated to a view.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Quotum {
    /// All the remaining space, split equally across all views with this quotum.
    Remaining,
    /// An exact length.
    Exact(Length),
    /// The minimum allowed size for the view.
    Minimum,
}
