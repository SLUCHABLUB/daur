//! The inner workings of the DAW.

pub mod audio;
pub mod metre;
pub mod note;
pub mod popup;
pub mod project;
pub mod sync;
pub mod time;
pub mod ui;
pub mod view;

mod app;
mod extension;
mod id;
mod node;
mod piano_roll;
mod ratio;
mod string;

// TODO: move `Action` to `mod App`
pub use app::{Action, Actions, App, HoldableObject, Selection};
pub use id::Id;
pub use piano_roll::PianoRoll;
pub use ratio::{NonZeroRatio, Ratio};
pub use string::ToArcStr;

#[doc(inline)]
pub use audio::Audio;
#[doc(inline)]
pub use note::Note;
#[doc(inline)]
pub use popup::Popup;
#[doc(inline)]
pub use project::Project;
#[doc(inline)]
pub use ui::UserInterface;
#[doc(inline)]
pub use view::View;
