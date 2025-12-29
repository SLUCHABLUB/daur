//! The inner workings of the DAW.

pub mod app;
pub mod audio;
pub mod holdable;
pub mod metre;
pub mod note;
pub mod popup;
pub mod project;
pub mod sync;
pub mod time;
pub mod ui;
pub mod view;

mod extension;
mod id;
mod node;
mod piano_roll;
mod ratio;
mod select;
mod string;

pub use id::Id;
pub use piano_roll::PianoRoll;
pub use ratio::NonZeroRatio;
pub use ratio::Ratio;
pub use select::Selectable;
pub use string::ToArcStr;

#[doc(inline)]
pub use app::App;
#[doc(inline)]
pub use audio::Audio;
#[doc(inline)]
pub use holdable::Holdable;
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
