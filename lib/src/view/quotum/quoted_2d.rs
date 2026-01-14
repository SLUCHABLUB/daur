//! Items pertaining to [`Quoted2D`].

use crate::UserInterface;
use crate::View;
use crate::ui::Size;
use crate::ui::relative;
use crate::view::Quotum;
use crate::view::Quotum2D;
use crate::view::RenderArea;
use std::cell::LazyCell;

/// A [view](View) with a two-dimensional [quotum](Quotum2D).
#[must_use = "the inner View must be used"]
#[derive(Debug)]
pub struct Quoted2D {
    /// The [quotum](Quotum2D) of the view.
    pub quotum: Quotum2D,
    /// The quoted [view](View).
    pub view: View,
}

impl Quoted2D {
    /// An [empty view](View::Empty) with a zero quotum.
    pub const EMPTY: Quoted2D = View::Empty.quoted_2d(Size::ZERO);

    /// Calculates the [size](Size) of the view.
    pub(crate) fn calculate_size<Ui: UserInterface>(
        &self,
        maximum: Size,
        render_area: RenderArea,
    ) -> Size {
        let minimum = LazyCell::new(|| self.view.minimum_size::<Ui>(render_area));

        Size {
            width: match self.quotum.x {
                Quotum::Remaining => maximum.width,
                Quotum::Exact(width) => width,
                Quotum::Minimum => minimum.width,
            },
            height: match self.quotum.y {
                Quotum::Remaining => maximum.height,
                Quotum::Exact(height) => height,
                Quotum::Minimum => minimum.height,
            },
        }
    }

    /// Positions the view on the screen. See [`View::Positioned`].
    pub fn positioned(self, position: relative::Point) -> View {
        View::Positioned {
            position,
            view: Box::new(self),
        }
    }
}

impl View {
    /// Adds a [quotum](Quotum2D) to the view.
    pub const fn with_2d_quotum(self, quotum: Quotum2D) -> Quoted2D {
        Quoted2D { quotum, view: self }
    }

    /// Makes the view fill the remaining space.
    pub fn fill_remaining_2d(self) -> Quoted2D {
        self.with_2d_quotum(Quotum2D::REMAINING)
    }

    /// Makes the view take up the specified [amount of space](Size).
    pub const fn quoted_2d(self, size: Size) -> Quoted2D {
        self.with_2d_quotum(size.quotum())
    }

    /// Adds a quotum to the view using [`minimum_size`](View::minimum_size).
    pub fn quoted_minimally_2d(self) -> Quoted2D {
        self.with_2d_quotum(Quotum2D::MINIMUM)
    }
}
