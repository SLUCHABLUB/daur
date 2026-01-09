use crate::Project;
use crate::metre::Instant;
use crate::popup;
use crate::project::Edit;
use crate::project::HistoryEntry;
use crate::select::Selection;
use anyhow::Context as _;
use getset::Getters;
use std::fs::read_to_string;
use std::fs::write;
use std::path::Path;
use std::sync::Arc;

/// Manages editing of a [project](Project).
#[derive(Debug, Default, Getters)]
pub struct Manager {
    /// The project.
    #[get = "pub"]
    project: Project,

    // TODO: undoing
    history: Vec<HistoryEntry>,

    // TODO: Add a format field.
    save_location: Option<Arc<Path>>,
}

impl Manager {
    pub(crate) fn edit(
        &mut self,
        action: Edit,
        cursor: Instant,
        selection: &mut Selection,
    ) -> anyhow::Result<()> {
        let entry = self.project.edit(action, cursor, selection)?;
        self.history.push(entry);

        Ok(())
    }

    pub(crate) fn open(path: Arc<Path>) -> anyhow::Result<Manager> {
        let content =
            read_to_string(&path).with_context(|| format!("reading from {}", path.display()))?;

        let project: Project =
            toml::from_str(&content).with_context(|| format!("parsing {}", path.display()))?;

        Ok(Manager {
            project,
            history: Vec::new(),
            save_location: Some(path),
        })
    }

    pub(crate) fn save(&mut self) -> Result<(), popup::Specification> {
        let path = self
            .save_location
            .clone()
            .ok_or(popup::Specification::SaveLocationPicker)?;

        self.save_as(path)?;

        Ok(())
    }

    pub(crate) fn save_as(&mut self, mut path: Arc<Path>) -> anyhow::Result<()> {
        let string = toml::to_string(&self.project)?;

        if path.is_dir() {
            let file_name = format!("{}.toml", self.project.name);

            path = path.join(file_name).into();
        }

        write(&path, string).with_context(|| format!("writing to {}", path.display()))?;

        self.save_location.get_or_insert(path);

        Ok(())
    }
}
