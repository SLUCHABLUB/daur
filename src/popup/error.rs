use crate::app::Action;
use crate::cell::Cell;
use crate::keyboard::Key;
use crate::length::Length;
use crate::popup::button::Terminating;
use crate::popup::info::PopupInfo;
use crate::popup::Popup;
use crate::widget::bordered::Bordered;
use crate::widget::button::Button;
use crate::widget::has_size::HasSize as _;
use crate::widget::heterogeneous::ThreeStack;
use crate::widget::text::Text;
use crate::widget::to_widget::ToWidget;
use arcstr::{format, literal, ArcStr};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Flex};
use std::error::Error;
use std::sync::Weak;

const ACKNOWLEDGE: ArcStr = literal!("ok");
const PADDING: Length = Length::CHAR_HEIGHT;

#[derive(Clone, Eq, PartialEq)]
pub struct ErrorPopup {
    pub info: PopupInfo,
    pub display: ArcStr,
    pub debug: ArcStr,
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

    pub fn handle_key(&self, key: Key, actions: &mut Vec<Action>) -> bool {
        #[expect(clippy::wildcard_enum_match_arm, reason = "we only care about these")]
        match key.code {
            KeyCode::Enter if self.selected.get() => {
                actions.push(Action::ClosePopup(self.info.this()));
                true
            }
            KeyCode::Enter | KeyCode::Tab | KeyCode::BackTab => {
                self.selected.set(!self.selected.get());
                true
            }
            _ => false,
        }
    }
}

impl ToWidget for ErrorPopup {
    type Widget<'ignore> = ThreeStack<Text, Text, Terminating<Bordered<Button>>>;

    fn to_widget(&self) -> Self::Widget<'_> {
        let acknowledge_button =
            Button::standard(ACKNOWLEDGE, Action::None).thickness(self.selected.get());
        let constraints = [
            Length::string_height(&self.display).constraint_max(),
            Constraint::Fill(1),
            acknowledge_button.size().height.constraint(),
        ];

        ThreeStack::vertical(
            (
                Text::left_aligned(self.display()),
                Text::left_aligned(self.debug()),
                Terminating {
                    child: acknowledge_button,
                    popup: self.info.this(),
                },
            ),
            constraints,
        )
        .flex(Flex::SpaceBetween)
        .spacing(PADDING)
    }
}
