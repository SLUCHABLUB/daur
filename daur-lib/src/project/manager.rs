use crate::Selection;
use crate::metre::Instant;
use crate::project::{Action, Project};
use anyhow::Result;
use getset::Getters;

/// Manages editing of a [project](Project).
#[derive(Debug, Default, Getters)]
pub struct Manager {
    /// The project.
    #[get = "pub"]
    project: Project,
    // TODO: history
}

impl Manager {
    pub(crate) fn take_action(
        &mut self,
        action: Action,
        cursor: Instant,
        selection: &mut Selection,
    ) -> Result<()> {
        // TODO: add to history
        self.project.take_action(action, cursor, selection)
    }
}
