mod quotated;
mod quotated_2d;
mod quotum_2d;

pub use quotated::Quotated;
pub use quotated_2d::Quotated2D;
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
