use arcstr::ArcStr;
use core::fmt::Display;

/// Like [`ToString`] but for [`ArcStr`].
pub trait ToArcStr {
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
