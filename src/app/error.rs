use std::ffi::OsString;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("The format `{}` is not (yet) supported", format.to_string_lossy())]
pub struct UnsupportedFormatError {
    pub format: OsString,
}

#[derive(Debug, Error)]
#[error("Unable to infer the audio format of file `{file}`")]
pub struct NoExtensionError {
    pub file: PathBuf,
}
