//! Items pertaining to [`Serial`].

use crate::Project;
use crate::metre::Changing;
use crate::metre::TimeSignature;
use crate::note::Key;
use crate::project::track;
use crate::time::Tempo;
use arcstr::ArcStr;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

/// The serial representation of a [project](Project).
#[derive(Serialize, Deserialize)]
pub(super) struct Serial<'data> {
    /// The name.
    pub name: Cow<'data, str>,

    /// The tempo.
    pub tempo: Changing<Tempo>,
    /// The time signature.
    pub time_signature: Changing<TimeSignature>,
    /// The key.
    pub key: Changing<Key>,

    /// The tracks.
    pub tracks: Vec<track::Serial<'data>>,
}

impl<'data> From<&'data Project> for Serial<'data> {
    fn from(project: &'data Project) -> Self {
        let Project {
            name,
            tempo,
            time_signature,
            key,
            tracks,
        } = project;

        Serial {
            name: Cow::Borrowed(name),
            tempo: tempo.clone(),
            time_signature: time_signature.clone(),
            key: key.clone(),
            tracks: tracks.values().map(track::Serial::from).collect(),
        }
    }
}

impl<'data> From<Serial<'data>> for Project {
    fn from(serial: Serial<'data>) -> Self {
        let Serial {
            name,
            tempo,
            time_signature,
            key,
            tracks,
        } = serial;

        // TODO: convert the tracks
        drop(tracks);

        Project {
            name: ArcStr::from(name),
            tempo,
            time_signature,
            key,
            tracks: IndexMap::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use anyhow::Context as _;
    use anyhow::ensure;
    use std::fs::read_dir;
    use std::fs::read_to_string;
    use std::path::Path;

    #[test]
    fn parse_toml_example_projects() -> anyhow::Result<()> {
        let lib = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = lib.parent().context("invalid CARGO_MANIFEST_DIR")?;
        let examples = root.join("examples").join("toml");

        let directory_context = || format!("reading {}", examples.display());

        for entry in read_dir(&examples).with_context(directory_context)? {
            let entry = entry?;
            let path = entry.path();

            let file_context = || format!("reading {}", path.display());

            let content = read_to_string(&path).with_context(file_context)?;

            let project: Serial = toml::from_str(&content).with_context(file_context)?;

            let string = toml::to_string(&project).with_context(file_context)?;

            ensure!(
                content == string,
                "comparing {} with serializer output",
                path.display()
            );
        }

        Ok(())
    }

    #[test]
    fn serialize_project_default() -> anyhow::Result<()> {
        let lib = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = lib.parent().context("invalid CARGO_MANIFEST_DIR")?;
        let example_path = root.join("examples").join("toml").join("empty.toml");

        let empty_example = read_to_string(&example_path)
            .with_context(|| format!("reading {}", example_path.display()))?;

        let default_project = toml::to_string(&Project::default())?;

        ensure!(
            empty_example == default_project,
            "`Project::default()` does not serialize to {}, it serializes to: ```\n{default_project}```",
            example_path.display()
        );

        Ok(())
    }
}
