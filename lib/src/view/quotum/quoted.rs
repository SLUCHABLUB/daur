//! Items pertaining to [`Quoted`].

use crate::UserInterface;
use crate::View;
use crate::ui::Length;
use crate::ui::relative;
use crate::view::Axis;
use crate::view::Quotum;
use crate::view::Quotum2D;
use crate::view::RenderArea;

/// A [view](View) with a [quotum](Quotum).
#[must_use = "the inner View must be used"]
#[derive(Debug)]
pub struct Quoted {
    /// The [quotum](Quotum) of the view.
    pub quotum: Quotum,
    /// The quoted [view](View).
    pub view: View,
}

impl Quoted {
    /// An [empty view](View::Empty) with a zero quotum.
    pub const EMPTY: Quoted = View::Empty.with_quotum(Length::ZERO.quotum());

    /// Calculates the size of the view parallell to a given axis.
    ///
    /// If the quoted size is infinite, [`None`] is returned.
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
    pub const fn with_quotum(self, quotum: Quotum) -> Quoted {
        Quoted { quotum, view: self }
    }

    /// Makes the view fill the remaining space.
    pub fn fill_remaining(self) -> Quoted {
        self.with_quotum(Quotum::Remaining)
    }

    /// Makes the view take up the specified [amount of space](Length).
    pub fn quoted<L: Into<Length>>(self, size: L) -> Quoted {
        self.with_quotum(size.into().quotum())
    }

    /// Adds a quotum to the view using [`minimum_size`](View::minimum_size).
    pub fn quoted_minimally(self) -> Quoted {
        self.with_quotum(Quotum::Minimum)
    }
}
