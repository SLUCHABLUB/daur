use arcstr::ArcStr;
use std::fmt::Display;

/// Like [`std::string::ToString`] but for [`ArcStr`]
pub trait ToArcStr {
    /// Converts `self` to an [`ArcStr`]
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
