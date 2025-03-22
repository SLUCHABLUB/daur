//! The inner workings of the DAW

mod app;
mod cell;
mod changing;
mod chroma;
mod colour;
mod interval;
mod key;
mod lock;
mod note;
mod notes;
mod pitch;
mod ratio;
mod receiver;
mod sign;
mod string;

pub mod audio;
pub mod clip;
pub mod context;
pub mod popup;
pub mod project;
pub mod time;
pub mod track;
pub mod ui;
pub mod view;

#[cfg(test)]
mod test;

pub use app::{Action, App};
pub use cell::{ArcCell, Cell, OptionArcCell};
pub use changing::Changing;
pub use colour::Colour;
pub use ratio::{NonZeroRatio, Ratio};
pub use receiver::Receiver;
pub use string::ToArcStr;

#[doc(inline)]
pub use audio::Audio;
#[doc(inline)]
pub use clip::Clip;
#[doc(inline)]
pub use popup::Popup;
#[doc(inline)]
pub use project::Project;
#[doc(inline)]
pub use track::Track;
#[doc(inline)]
pub use view::View;

pub use arcstr;
