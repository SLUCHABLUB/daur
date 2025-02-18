use crate::app::action::Action;
use crate::length::Length;
use crate::popup::button::Terminating;
use crate::popup::info::PopupInfo;
use crate::popup::Popup;
use crate::widget::bordered::Bordered;
use crate::widget::button::Button;
use crate::widget::heterogeneous::ThreeStack;
use crate::widget::has_size::HasSize as _;
use crate::widget::text::Text;
use crate::widget::to_widget::ToWidget;
use ratatui::layout::{Constraint, Flex};
use std::error::Error;
use std::sync::Weak;

const ACKNOWLEDGE: &str = "ok";
const PADDING: Length = Length::CHAR_HEIGHT;

#[derive(Clone, Eq, PartialEq)]
pub struct ErrorPopup {
    pub info: PopupInfo,
    pub display: String,
    pub debug: String,
}

impl ErrorPopup {
    pub fn from_error<E: Error>(error: E, this: Weak<Popup>) -> Self {
        ErrorPopup {
            info: PopupInfo::new(String::from("error"), this),
            display: format!("{error}"),
            debug: format!("{error:?}"),
        }
    }
}

impl ToWidget for ErrorPopup {
    type Widget<'ignore> = ThreeStack<Text, Text, Terminating<Bordered<Button>>>;

    fn to_widget(&self) -> Self::Widget<'_> {
        let acknowledge_button = Button::standard(ACKNOWLEDGE, Action::None);
        let constraints = [
            Length::string_height(&self.display).constraint_max(),
            Constraint::Fill(1),
            acknowledge_button.size().height.constraint(),
        ];

        ThreeStack::vertical(
            (
                Text::left_aligned(self.display.as_str()),
                Text::left_aligned(self.debug.as_str()),
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
