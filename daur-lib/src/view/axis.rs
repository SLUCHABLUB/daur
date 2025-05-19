use crate::ui::{Offset, Vector};
use std::ops::Mul;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// An axis along which view can be aligned.
pub enum Axis {
    /// The horizontal axis.
    X,
    /// The vertical axis.
    Y,
}

impl Mul<Offset> for Axis {
    type Output = Vector;

    fn mul(self, rhs: Offset) -> Self::Output {
        match self {
            Axis::X => Vector::from_x(rhs),
            Axis::Y => Vector::from_y(rhs),
        }
    }
}
