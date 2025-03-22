use crate::popup::info::Info;
use crate::view::{Direction, OnClick, View};
use crate::{Action, Cell};
use arcstr::ArcStr;

/// A panel of buttons.
#[derive(Debug)]
pub struct ButtonPanel {
    /// The popup info.
    pub info: Info,
    /// The buttons.
    pub buttons: Vec<(ArcStr, Action)>,
    // TODO: display
    /// The index of the currently selected button.
    pub selected: Cell<Option<usize>>,
}

impl ButtonPanel {
    pub(super) fn view(&self) -> View {
        View::balanced_stack(
            Direction::Down,
            self.buttons.iter().map(|(label, action)| {
                View::simple_button(ArcStr::clone(label), OnClick::from(action.clone()))
                    .terminating(self.info.this())
            }),
        )
    }
}
