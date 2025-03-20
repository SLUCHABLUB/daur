use crate::popup::info::PopupInfo;
use crate::popup::terminating;
use crate::view::{Direction, OnClick, View};
use crate::{Action, Cell};
use arcstr::ArcStr;

#[derive(Debug)]
pub struct ButtonPanel {
    pub info: PopupInfo,
    pub buttons: Vec<(ArcStr, Action)>,
    // TODO: display
    pub selected: Cell<Option<usize>>,
}

impl ButtonPanel {
    pub fn view(&self) -> View {
        View::balanced_stack(
            Direction::Down,
            self.buttons.iter().map(|(label, action)| {
                terminating(
                    View::simple_button(ArcStr::clone(label), OnClick::from(action.clone())),
                    self.info.this(),
                )
            }),
        )
    }
}
