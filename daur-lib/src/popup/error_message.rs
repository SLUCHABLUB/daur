use crate::Cell;
use crate::popup::Popup;
use crate::popup::info::Info;
use crate::view::{Alignment, Direction, ToText as _, View};
use arcstr::{ArcStr, format, literal};
use std::error::Error;
use std::sync::Weak;

const ACKNOWLEDGE: ArcStr = literal!("ok");

/// An error message.
#[derive(Clone, Debug)]
pub struct ErrorMessage {
    /// The popup info.
    pub info: Info,
    /// The display representation of the error.
    pub display: ArcStr,
    /// The debug representation of the error.
    pub debug: ArcStr,
    /// Whether the acknowledge button is selected.
    pub selected: Cell<bool>,
}

impl ErrorMessage {
    /// Construct a new error message from an error.
    pub fn from_error<E: Error>(error: E, this: Weak<Popup>) -> Self {
        ErrorMessage {
            info: Info::new(literal!("error"), this),
            display: format!("{error}"),
            debug: format!("{error:?}"),
            selected: Cell::new(false),
        }
    }

    // TODO: derive
    fn display(&self) -> ArcStr {
        ArcStr::clone(&self.display)
    }

    // TODO: derive
    fn debug(&self) -> ArcStr {
        ArcStr::clone(&self.debug)
    }

    pub(super) fn view(&self) -> View {
        let acknowledge_button = ACKNOWLEDGE
            .centred()
            .bordered()
            .with_thickness(self.selected.get());

        View::spaced_stack(
            Direction::Down,
            [
                self.display().aligned_to(Alignment::TopLeft),
                self.debug().aligned_to(Alignment::TopLeft),
                acknowledge_button.terminating(self.info.this()),
            ],
        )
    }
}
