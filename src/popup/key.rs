use crate::app::action::Action;
use crate::cell::Cell;
use crate::chroma::Chroma;
use crate::key::{Key, KeyInterval};
use crate::popup::button::Terminating;
use crate::popup::info::PopupInfo;
use crate::popup::Popup;
use crate::sign::Sign;
use crate::time::instant::Instant;
use crate::widget::block::Bordered;
use crate::widget::button::Button;
use crate::widget::heterogeneous_stack::{FourStack, TwoStack};
use crate::widget::multi_selector::{multi_selector, MultiSelector};
use crate::widget::single_selector::{single_selector, SingleSelector};
use crate::widget::to_widget::ToWidget;
use bitbag::BitBag;
use educe::Educe;
use ratatui::layout::{Constraint, Flex};
use std::sync::Weak;

const TITLE: &str = "select key";

#[derive(Clone, Educe)]
#[educe(Eq, PartialEq)]
pub struct KeySelector {
    pub info: PopupInfo,

    #[educe(Eq(ignore))]
    tonic: Cell<Chroma>,
    #[educe(Eq(ignore))]
    sign: Cell<Sign>,
    #[educe(Eq(ignore))]
    intervals: Cell<BitBag<KeyInterval>>,
}

impl KeySelector {
    pub fn new(key: Key, this: Weak<Popup>) -> KeySelector {
        KeySelector {
            info: PopupInfo::new(TITLE.to_owned(), this),
            tonic: Cell::new(key.tonic),
            sign: Cell::new(key.sign),
            intervals: Cell::new(key.intervals),
        }
    }

    fn key(&self) -> Key {
        Key {
            tonic: self.tonic.get(),
            sign: self.sign.get(),
            intervals: self.intervals.get(),
        }
    }
}

impl ToWidget for KeySelector {
    type Widget<'cell> = FourStack<
        SingleSelector<'cell, Chroma>,
        SingleSelector<'cell, Sign>,
        MultiSelector<'cell, KeyInterval>,
        TwoStack<Terminating<Bordered<Button>>, Terminating<Bordered<Button>>>,
    >;

    fn to_widget(&self) -> Self::Widget<'_> {
        let buttons = TwoStack::horizontal_sized((
            Terminating {
                child: Button::standard("cancel", Action::None),
                popup: self.info.this(),
            },
            Terminating {
                child: Button::standard("confirm", Action::SetKey(Instant::START, self.key())),
                popup: self.info.this(),
            },
        ))
        .flex(Flex::SpaceBetween);

        FourStack::vertical(
            (
                single_selector(&self.tonic),
                single_selector(&self.sign),
                multi_selector(&self.intervals),
                buttons,
            ),
            [Constraint::Length(3); 4],
        )
    }
}
