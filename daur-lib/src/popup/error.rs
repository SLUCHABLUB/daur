use crate::popup::info::PopupInfo;
use crate::popup::terminating::terminating;
use crate::popup::Popup;
use crate::view::{Direction, View};
use crate::Cell;
use arcstr::{format, literal, ArcStr};
use std::error::Error;
use std::sync::Weak;

const ACKNOWLEDGE: ArcStr = literal!("ok");

#[derive(Clone, Debug)]
pub struct ErrorPopup {
    /// Info about the popup.
    pub info: PopupInfo,
    /// The display representation of the error.
    pub display: ArcStr,
    /// The debug representation of the error.
    pub debug: ArcStr,
    /// Whether the acknowledge button is selected.
    pub selected: Cell<bool>,
}

impl ErrorPopup {
    pub fn from_error<E: Error>(error: E, this: Weak<Popup>) -> Self {
        ErrorPopup {
            info: PopupInfo::new(literal!("error"), this),
            display: format!("{error}"),
            debug: format!("{error:?}"),
            selected: Cell::new(false),
        }
    }

    pub fn display(&self) -> ArcStr {
        ArcStr::clone(&self.display)
    }

    pub fn debug(&self) -> ArcStr {
        ArcStr::clone(&self.debug)
    }

    pub fn view(&self) -> View {
        let acknowledge_button = View::centred(ACKNOWLEDGE)
            .bordered()
            .with_thickness(self.selected.get());

        View::spaced_stack(
            Direction::Down,
            [
                View::top_left(self.display()),
                View::top_left(self.debug()),
                terminating(acknowledge_button, self.info.this()),
            ],
        )
    }
}
