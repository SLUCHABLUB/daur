//! Items pertaining to [`Plugins`].

use crate::Project;
use crate::View;
use crate::project::Track;
use crate::select::Selection;
use crate::ui::Vector;
use crate::view::ToText as _;
use arcstr::ArcStr;
use arcstr::literal;

/// Volatile settings for the plugin workspace.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Plugins;

impl Plugins {
    /// Return a view of the content of the workspace.
    pub(super) fn content(self) -> View {
        let _: Plugins = self;

        literal!("plugins").centred()
    }

    /// Return the title of the workspace.
    pub(super) fn title(project: &Project, selection: &Selection) -> ArcStr {
        selection
            .top_track()
            .and_then(|id| project.track(id))
            .map_or_else(|| project.name(), Track::name)
    }

    /// Moves the workspace.
    pub(super) fn move_by(&mut self, by: Vector) {
        let _: (&Plugins, Vector) = (self, by);
    }
}
