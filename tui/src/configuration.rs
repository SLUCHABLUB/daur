//! Items pertaining to [`Configuration`].

use crate::Key;
use anyhow::Context as _;
use daur::app::Action;
use directories::ProjectDirs;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

/// The name of the configuration file.
const CONFIGURATION_FILE_NAME: &str = "configuration.toml";
/// The contents of the [default configuration file](../resources/default_configuration.toml).
const DEFAULT_CONFIGURATION: &str = include_str!("../resources/default_configuration.toml");

/// Settings for the user interface.
#[derive(Debug, Deserialize)]
pub(crate) struct Configuration {
    /// The key to action map.
    pub key_map: HashMap<Key, Action>,
}

impl Configuration {
    /// Reads a [configuration instance](Configuration) from the configuration file,
    /// the location of which is system dependent.
    pub(crate) fn read_from_file(directories: &ProjectDirs) -> anyhow::Result<Configuration> {
        let configuration_path = &directories.config_dir().join(CONFIGURATION_FILE_NAME);

        let string = match read_to_string(configuration_path) {
            Ok(string) => Cow::Owned(string),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                Cow::Borrowed(DEFAULT_CONFIGURATION)
            }
            Err(error) => {
                return Err(anyhow::Error::from(error).context(format!(
                    "reading configuration file at {}",
                    configuration_path.display()
                )));
            }
        };

        let configuration = toml::from_str(&string).with_context(|| {
            format!(
                "parsing configuration file at {}",
                configuration_path.display()
            )
        })?;

        Ok(configuration)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_default_configuration() -> anyhow::Result<()> {
        toml::from_str::<Configuration>(DEFAULT_CONFIGURATION)?;

        Ok(())
    }
}
