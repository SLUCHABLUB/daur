mod quotated;

pub use quotated::Quotated;

use crate::ui::{Length, Size};
use crate::view::Axis;

/// The amount of space that is allocated to a view.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Quotum {
    /// All the remaining space, split equally across all views with this quotum.
    Remaining,
    /// An exact length.
    Exact(Length),
    /// A quotum that depends on the direction.
    DirectionDependent(Size),
}

impl Quotum {
    pub(crate) fn size_parallel_to(self, axis: Axis) -> Option<Length> {
        match self {
            Quotum::Remaining => None,
            Quotum::Exact(length) => Some(length),
            Quotum::DirectionDependent(size) => Some(size.parallel_to(axis)),
        }
    }
}
