use crate::keyboard::Key;
use crate::popup::info::PopupInfo;
use crate::popup::terminating::Terminating;
use crate::popup::Popup;
use crate::ui::Length;
use crate::view::heterogeneous::ThreeStack;
use crate::view::{Bordered, Composition, HasSize as _, Text};
use crate::{Action, Cell};
use arcstr::{format, literal, ArcStr};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Flex};
use std::error::Error;
use std::sync::Weak;

const ACKNOWLEDGE: ArcStr = literal!("ok");
const PADDING: Length = Length::CHAR_HEIGHT;

#[derive(Clone, Eq, PartialEq, Debug)]
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

impl Composition for ErrorPopup {
    type Body<'ignore> = ThreeStack<Text, Text, Terminating<Bordered<Text>>>;

    fn body(&self) -> Self::Body<'_> {
        let acknowledge_button =
            Bordered::plain(Text::centred(ACKNOWLEDGE)).thickness(self.selected.get());
        // TODO: favour buttons (by means of size-informed?)
        let constraints = [
            Length::string_height(&self.display).constraint(),
            Constraint::Fill(1),
            acknowledge_button.size().height.constraint(),
        ];

        ThreeStack::vertical(
            (
                Text::top_left(self.display()),
                Text::top_left(self.debug()),
                Terminating {
                    content: acknowledge_button,
                    popup: self.info.this(),
                },
            ),
            constraints,
        )
        .flex(Flex::SpaceBetween)
        .spacing(PADDING)
    }
}
