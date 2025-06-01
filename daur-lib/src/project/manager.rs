use crate::Selection;
use crate::metre::Instant;
use crate::project::{Action, HistoryEntry, Project};
use anyhow::Result;
use getset::Getters;

/// Manages editing of a [project](Project).
#[derive(Debug, Default, Getters)]
pub struct Manager {
    /// The project.
    #[get = "pub"]
    project: Project,
    // TODO: undoing
    history: Vec<HistoryEntry>,
}

impl Manager {
    pub(crate) fn take_action(
        &mut self,
        action: Action,
        cursor: Instant,
        selection: &mut Selection,
    ) -> Result<()> {
        let entry = self.project.take_action(action, cursor, selection)?;
        self.history.extend(entry);

        Ok(())
    }
}
