use crate::ui::{Length, relative};
use crate::view::{Axis, Quotum, Quotum2D, RenderArea};
use crate::{UserInterface, View};

/// A [view](View) with a [quotum](Quotum).
#[must_use = "the inner View must be used"]
#[derive(Debug)]
pub struct Quotated {
    /// The [quotum](Quotum) of the view.
    pub quotum: Quotum,
    /// The quotated [view](View).
    pub view: View,
}

impl Quotated {
    /// An [empty view](View::Empty) with a zero quotum.
    pub const EMPTY: Quotated = View::Empty.with_quotum(Length::ZERO.quotum());

    pub(crate) fn size_parallel_to<Ui: UserInterface>(
        &self,
        axis: Axis,
        render_area: RenderArea,
    ) -> Option<Length> {
        match self.quotum {
            Quotum::Remaining => None,
            Quotum::Exact(length) => Some(length),
            Quotum::Minimum => Some(self.view.minimum_size::<Ui>(render_area).parallel_to(axis)),
        }
    }

    /// Positions the view along the x-axis. See [`View::Positioned`].
    pub fn x_positioned(self, position: Length) -> View {
        let quotum = self.quotum;
        self.view
            .with_2d_quotum(Quotum2D {
                x: quotum,
                y: Quotum::Remaining,
            })
            .positioned(relative::Point {
                x: position,
                y: Length::ZERO,
            })
    }
}

impl View {
    /// Adds a [quotum](Quotum) to the view.
    pub const fn with_quotum(self, quotum: Quotum) -> Quotated {
        Quotated { quotum, view: self }
    }

    /// Makes the view fill the remaining space.
    pub fn fill_remaining(self) -> Quotated {
        self.with_quotum(Quotum::Remaining)
    }

    /// Makes the view take up the specified [amount of space](Length).
    pub fn quotated<L: Into<Length>>(self, size: L) -> Quotated {
        self.with_quotum(size.into().quotum())
    }

    /// Adds a quotum to the view using [`minimum_size`](View::minimum_size).
    pub fn quotated_minimally(self) -> Quotated {
        self.with_quotum(Quotum::Minimum)
    }
}
