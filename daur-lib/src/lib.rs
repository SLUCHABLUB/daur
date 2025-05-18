//! The inner workings of the DAW.

extern crate alloc;

pub mod audio;
pub mod clip;
pub mod metre;
pub mod notes;
pub mod popup;
pub mod project;
pub mod sync;
pub mod time;
pub mod track;
pub mod ui;
pub mod view;

mod app;
mod extension;
mod id;
mod piano_roll;
mod ratio;
mod string;

pub use app::{Action, Actions, App, HoldableObject, Selection};
pub use id::Id;
pub use piano_roll::PianoRoll;
pub use ratio::{NonZeroRatio, Ratio};
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
pub use ui::UserInterface;
#[doc(inline)]
pub use view::View;
