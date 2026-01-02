use arcstr::ArcStr;
use std::fmt::Display;

/// Like [`ToString`] but for [`ArcStr`].
pub(crate) trait ToArcStr {
    /// Converts the value to an [`ArcStr`].
    fn to_arc_str(&self) -> ArcStr;
}

impl<T> ToArcStr for T
where
    T: Display + ?Sized,
{
    fn to_arc_str(&self) -> ArcStr {
        arcstr::format!("{self}")
    }
}
