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
mod track;
mod widget;

pub mod time;
pub mod ui;

#[cfg(test)]
mod test;

pub use app::App;
pub use audio::{Audio, AudioSource};
pub use cell::Cell;
pub use clip::{Clip, ClipContent, ClipSource};
pub use piano_roll::PianoRollSettings;
pub use ratio::{NonZeroRatio, Ratio};
