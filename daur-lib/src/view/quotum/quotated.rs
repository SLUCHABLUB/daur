use crate::ui::Length;
use crate::view::Quotum;
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
    pub const EMPTY: Quotated = View::Empty.quotated(Length::ZERO);
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
    pub const fn quotated(self, size: Length) -> Quotated {
        self.with_quotum(size.quotum())
    }

    /// Adds a quotum to the view using [`minimum_size`](View::minimum_size).
    pub fn quotated_minimally<Ui: UserInterface>(self) -> Quotated {
        let size = self.minimum_size::<Ui>();
        self.with_quotum(Quotum::DirectionDependent(size))
    }
}
