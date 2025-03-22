use crate::popup::Popup;
use arcstr::ArcStr;
use std::sync::Weak;

/// Info about a popup.
#[derive(Clone, Debug)]
pub struct Info {
    title: ArcStr,
    this: Weak<Popup>,
}

impl Info {
    /// Construct a new popup info.
    #[must_use]
    pub fn new(title: ArcStr, this: Weak<Popup>) -> Info {
        Info { title, this }
    }

    // TODO: derive
    /// Returns the title of the popup.
    #[must_use]
    pub fn title(&self) -> ArcStr {
        ArcStr::clone(&self.title)
    }

    // TODO: derive
    /// Returns a pointer to the popup.
    #[must_use]
    pub fn this(&self) -> Weak<Popup> {
        Weak::clone(&self.this)
    }
}
