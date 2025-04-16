use crate::UserInterface;
use crate::ui::{Length, Size};
use crate::view::{Direction, View};

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
    pub(crate) fn size_parallel_to(self, direction: Direction) -> Option<Length> {
        match self {
            Quotum::Remaining => None,
            Quotum::Exact(length) => Some(length),
            Quotum::DirectionDependent(size) => Some(size.parallel_to(direction)),
        }
    }
}

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
    pub const EMPTY: Quotated = Quotated {
        quotum: Quotum::Exact(Length::ZERO),
        view: View::Empty,
    };
}

impl View {
    /// Adds a [quotum](Quotum) to the view.
    pub fn with_quotum(self, quotum: Quotum) -> Quotated {
        Quotated { quotum, view: self }
    }

    /// Makes the view fill the remaining space.
    pub fn fill_remaining(self) -> Quotated {
        self.with_quotum(Quotum::Remaining)
    }

    /// Makes the view take up the specified [amount of space](Length).
    pub fn quotated(self, size: Length) -> Quotated {
        self.with_quotum(size.quotum())
    }

    /// Adds a quotum to the view using [`minimum_size`](View::minimum_size).
    pub fn quotated_minimally<Ui: UserInterface>(self) -> Quotated {
        let size = self.minimum_size::<Ui>();
        self.with_quotum(Quotum::DirectionDependent(size))
    }
}
