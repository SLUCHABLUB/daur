//! Items pertaining to [`Workspace`].

mod piano_roll;
mod plugins;
mod settings;

pub use piano_roll::PianoRoll;
pub use plugins::Plugins;
pub use settings::Settings;

use crate::Holdable;
use crate::view::RenderArea;
use serde::Deserialize;

/// A kind of workspace.
/// A workspace is the area where most "work" is performed.
/// It is distinct from the project overview where all tracks are listed.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Workspace {
    /// The piano roll, where notes are inputted.
    PianoRoll,
    /// The plugin panel, where instruments and effects are configured.
    Plugins,
}

impl Workspace {
    /// Returns the [holdable object](Holdable) representing the handle (top edge) of the workspace.
    fn handle_grabber(render_area: RenderArea) -> Option<Holdable> {
        let y = render_area.relative_mouse_position()?.y;

        Some(Holdable::WorkspaceHandle { y })
    }
}
