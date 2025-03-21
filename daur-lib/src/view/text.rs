use crate::view::{Alignment, View};
use arcstr::ArcStr;

/// An extension trait for [`ArcStr`] to turn it into a [view](`View`).
pub trait ToText: Sized {
    /// Aligns the text to some alignment.
    fn aligned_to(self, alignment: Alignment) -> View;

    /// Centres the text.
    fn centred(self) -> View {
        self.aligned_to(Alignment::Centre)
    }
}

impl ToText for ArcStr {
    fn aligned_to(self, alignment: Alignment) -> View {
        View::Text {
            string: self,
            alignment,
        }
    }
}
