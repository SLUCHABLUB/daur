//! The inner workings of the DAW

mod app;
mod audio;
mod cell;
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
mod project;
mod ratio;
mod sign;
mod string;
mod track;

pub mod time;
pub mod ui;
pub mod widget;

#[cfg(test)]
mod test;

pub use app::App;
pub use audio::{Audio, AudioSource};
pub use cell::Cell;
pub use clip::{Clip, ClipContent, ClipSource};
pub use piano_roll::PianoRollSettings;
pub use ratio::{NonZeroRatio, Ratio};
pub use string::ToArcStr;
