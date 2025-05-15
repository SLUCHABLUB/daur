use crate::ui::Colour;
use arcstr::ArcStr;
use getset::CloneGetters;

/// Settings for a [clip](crate::Clip)
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, CloneGetters)]
pub struct Settings {
    /// The name of the clip.
    #[get_clone = "pub"]
    pub name: ArcStr,
    /// The colour of the clip.
    pub colour: Colour,
}
