use crate::app::Action;
use crate::cell::Cell;
use crate::chroma::Chroma;
use crate::key::{Key, KeyInterval};
use crate::popup::info::PopupInfo;
use crate::popup::terminating::Terminating;
use crate::popup::Popup;
use crate::sign::Sign;
use crate::widget::heterogeneous::{FourStack, TwoStack};
use crate::widget::{multi, single, Bordered, Button, ToWidget};
use crate::{keyboard, project};
use arcstr::{literal, ArcStr};
use bitbag::BitBag;
use crossterm::event::{KeyCode, KeyModifiers};
use educe::Educe;
use ratatui::layout::{Constraint, Flex};
use std::sync::Weak;

const TITLE: ArcStr = literal!("select key");
const CANCEL: ArcStr = literal!("cancel");
const CONFIRM: ArcStr = literal!("confirm");

#[derive(Clone, Debug, Educe)]
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
            info: PopupInfo::new(TITLE, this),
            tonic: Cell::new(key.tonic),
            sign: Cell::new(key.sign),
            intervals: Cell::new(key.intervals),
        }
    }

    pub fn handle_key(&self, key: keyboard::Key, actions: &mut Vec<Action>) -> bool {
        #[expect(clippy::wildcard_enum_match_arm, reason = "we only care about these")]
        match key.code {
            KeyCode::Enter => {
                actions.push(Action::ClosePopup(self.info.this()));
                actions.push(Action::Project(project::Action::SetDefaultKey(self.key())));
                true
            }
            KeyCode::Tab | KeyCode::BackTab => {
                self.sign.set(!self.sign.get());
                true
            }
            KeyCode::Char(char) => {
                let set_tonic = |chroma| {
                    self.tonic.set(chroma);
                    true
                };

                let invert_interval = |interval| {
                    let mut bag = self.intervals.get();
                    if bag.is_set(interval) {
                        bag.unset(interval);
                    } else {
                        bag.set(interval);
                    }
                    self.intervals.set(bag);
                    true
                };

                // TODO: make the controls changeable in the settings

                if !matches!(key.modifiers, KeyModifiers::SHIFT | KeyModifiers::NONE) {
                    return false;
                }

                match char {
                    '"' => invert_interval(KeyInterval::M2),
                    '#' => invert_interval(KeyInterval::M3),
                    '&' => invert_interval(KeyInterval::M6),
                    '/' => invert_interval(KeyInterval::M7),

                    '2' => invert_interval(KeyInterval::m2),
                    '3' => invert_interval(KeyInterval::m3),
                    '6' => invert_interval(KeyInterval::m6),
                    '7' => invert_interval(KeyInterval::m7),

                    '4' => invert_interval(KeyInterval::P4),
                    '5' => invert_interval(KeyInterval::P5),

                    't' => invert_interval(KeyInterval::TT),

                    'A' => set_tonic(Chroma::A.with_sign(self.sign.get())),
                    'B' => set_tonic(Chroma::B.with_sign(self.sign.get())),
                    'C' => set_tonic(Chroma::C.with_sign(self.sign.get())),
                    'D' => set_tonic(Chroma::D.with_sign(self.sign.get())),
                    'E' => set_tonic(Chroma::E.with_sign(self.sign.get())),
                    'F' => set_tonic(Chroma::F.with_sign(self.sign.get())),
                    'G' => set_tonic(Chroma::G.with_sign(self.sign.get())),

                    'a' => set_tonic(Chroma::A),
                    'b' => set_tonic(Chroma::B),
                    'c' => set_tonic(Chroma::C),
                    'd' => set_tonic(Chroma::D),
                    'e' => set_tonic(Chroma::E),
                    'f' => set_tonic(Chroma::F),
                    'g' => set_tonic(Chroma::G),

                    _ => false,
                }
            }
            _ => false,
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
        single::Selector<'cell, Chroma>,
        single::Selector<'cell, Sign>,
        multi::Selector<'cell, KeyInterval>,
        TwoStack<Terminating<Bordered<Button>>, Terminating<Bordered<Button>>>,
    >;

    fn to_widget(&self) -> Self::Widget<'_> {
        let buttons = TwoStack::horizontal_sized((
            Terminating {
                child: Button::standard(CANCEL, Action::None),
                popup: self.info.this(),
            },
            Terminating {
                child: Button::standard(
                    CONFIRM,
                    Action::Project(project::Action::SetDefaultKey(self.key())),
                ),
                popup: self.info.this(),
            },
        ))
        .flex(Flex::SpaceBetween);

        FourStack::vertical(
            (
                single::selector_with_formatter(&self.tonic, |chroma| chroma.name(self.sign.get())),
                single::selector(&self.sign).flex(Flex::Center),
                multi::selector(&self.intervals),
                buttons,
            ),
            [Constraint::Length(3); 4],
        )
    }
}
