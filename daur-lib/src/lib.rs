//! The inner workings of the DAW

mod app;
mod audio;
mod cell;
mod changing;
mod chroma;
mod clip;
mod colour;
mod interval;
mod key;
mod lock;
mod note;
mod notes;
mod piano_roll;
mod pitch;
mod popup;
mod ratio;
mod receiver;
mod sign;
mod string;

pub mod context;
pub mod project;
pub mod time;
pub mod track;
pub mod ui;
pub mod view;

#[cfg(test)]
mod test;

pub use app::{Action, App};
pub use audio::{Audio, AudioSource};
pub use cell::{ArcCell, Cell, OptionArcCell};
pub use changing::Changing;
pub use clip::{Clip, ClipContent, ClipSource};
pub use colour::Colour;
pub use piano_roll::PianoRollSettings;
pub use popup::{Popup, Popups};
#[doc(inline)]
pub use project::Project;
pub use ratio::{NonZeroRatio, Ratio};
pub use string::ToArcStr;
