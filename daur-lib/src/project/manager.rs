use crate::metre::Instant;
use crate::project::Edit;
use crate::project::HistoryEntry;
use crate::project::Project;
use crate::select::Selection;
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
    pub(crate) fn edit(
        &mut self,
        action: Edit,
        cursor: Instant,
        selection: &mut Selection,
    ) -> Result<()> {
        let entry = self.project.edit(action, cursor, selection)?;
        self.history.push(entry);

        Ok(())
    }
}
