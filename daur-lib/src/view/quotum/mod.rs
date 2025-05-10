mod quotated;

pub use quotated::Quotated;

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
