use crate::app::action::Action;
use crate::popup::button::TerminatingButton;
use crate::popup::info::PopupInfo;
use crate::widget::button::Button;
use crate::widget::three_stack::ThreeStack;
use crate::widget::Widget;
use min_max::max;
use ratatui::layout::{Constraint, Flex, Size};
use ratatui::widgets::Paragraph;
use saturating_cast::SaturatingCast;
use std::error::Error;

const ACKNOWLEDGE: &str = "ok";
const PADDING: u16 = 1;
const ACKNOWLEDGE_BUTTON_HEIGHT: u16 = 3;
const ACKNOWLEDGE_BUTTON: Button = Button::new(ACKNOWLEDGE, Action::None).bordered();

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ErrorPopup {
    pub info: PopupInfo,
    pub display: String,
    pub debug: String,
}

impl ErrorPopup {
    pub fn size(&self) -> Size {
        Size {
            width: max!(
                self.display.chars().count(),
                self.debug.chars().count(),
                ACKNOWLEDGE.chars().count() + 2
            )
            .saturating_cast(),
            height: (self.display.lines().count() + self.debug.lines().count())
                .saturating_cast::<u16>()
                + 2 * PADDING
                + ACKNOWLEDGE_BUTTON_HEIGHT,
        }
    }

    pub fn to_widget(&self) -> impl Widget + use<'_> {
        ThreeStack::vertical(
            (
                Paragraph::new(self.display.as_str()),
                Paragraph::new(self.debug.as_str()),
                TerminatingButton {
                    button: ACKNOWLEDGE_BUTTON,
                    id: self.info.id(),
                },
            ),
            [
                Constraint::Max(self.display.lines().count().saturating_cast()),
                Constraint::Fill(1),
                Constraint::Length(ACKNOWLEDGE_BUTTON_HEIGHT),
            ],
        )
        .flex(Flex::SpaceBetween)
        .spacing(PADDING)
    }
}

impl<E: Error> From<E> for ErrorPopup {
    fn from(value: E) -> Self {
        ErrorPopup {
            info: PopupInfo::new(String::from("error")),
            display: format!("{value}"),
            debug: format!("{value:?}"),
        }
    }
}
