//! The inner workings of the DAW

mod app;
mod audio;
mod cell;
mod changing;
mod chroma;
mod clip;
mod interval;
mod key;
mod keyboard;
mod lock;
mod note;
mod notes;
mod piano_roll;
mod pitch;
mod popup;
mod ratio;
mod sign;
mod string;
mod track;

pub mod project;
pub mod time;
pub mod ui;
pub mod view;

mod receiver;
#[cfg(test)]
mod test;

pub use app::{Action, App};
pub use audio::{Audio, AudioSource};
pub use cell::Cell;
pub use changing::Changing;
pub use clip::{Clip, ClipContent, ClipSource};
pub use piano_roll::PianoRollSettings;
#[doc(inline)]
pub use project::Project;
pub use ratio::{NonZeroRatio, Ratio};
pub use string::ToArcStr;
