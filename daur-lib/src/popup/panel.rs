use crate::app::Action;
use crate::cell::Cell;
use crate::keyboard::Key;
use crate::measure::{Point, Rectangle};
use crate::popup::info::PopupInfo;
use crate::popup::terminating::Terminating;
use crate::widget::button::Button;
use crate::widget::homogenous::Stack;
use crate::widget::to_widget::ToWidget;
use crate::widget::Widget as _;
use crossterm::event::{KeyCode, MouseButton};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ButtonPanel {
    pub info: PopupInfo,
    pub buttons: Vec<Terminating<Button>>,
    // TODO: display
    pub selected: Cell<Option<usize>>,
}

impl ButtonPanel {
    pub fn handle_key(&self, event: Key, actions: &mut Vec<Action>) -> bool {
        #[expect(clippy::wildcard_enum_match_arm, reason = "we only care about these")]
        match event.code {
            KeyCode::Enter => {
                if let Some(index) = self.selected.get() {
                    if let Some(button) = self.buttons.get(index) {
                        button.click(
                            Rectangle::default(),
                            MouseButton::Left,
                            Point::default(),
                            actions,
                        );
                        return true;
                    }
                }

                false
            }
            KeyCode::Left | KeyCode::Up | KeyCode::BackTab => {
                self.selected.set(match self.selected.get() {
                    Some(0) | None => self.buttons.len().checked_sub(1),
                    Some(index) => index.checked_sub(1),
                });
                true
            }
            KeyCode::Right | KeyCode::Down | KeyCode::Tab => {
                self.selected.set(match self.selected.get() {
                    None => Some(0),
                    Some(index) => index.wrapping_add(1).checked_rem(self.buttons.len()),
                });
                true
            }
            _ => false,
        }
    }
}

impl ToWidget for ButtonPanel {
    type Widget<'buttons> = Stack<&'buttons Terminating<Button>>;

    fn to_widget(&self) -> Self::Widget<'_> {
        Stack::equidistant_vertical(&self.buttons)
    }
}
