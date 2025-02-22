//! The inner workings of the DAW

mod app;
mod audio;
mod cell;
mod chroma;
mod clip;
mod interval;
mod key;
mod keyboard;
mod length;
mod lock;
mod note;
mod notes;
mod pitch;
mod popup;
mod project;
mod ratio;
mod sign;
pub mod time;
mod track;
mod widget;

#[cfg(test)]
mod test;

pub use app::App;
pub use ratio::{NonZeroRatio, Ratio};
