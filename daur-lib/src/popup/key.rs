use crate::chroma::Chroma;
use crate::key::{Key, KeyInterval};
use crate::popup::info::PopupInfo;
use crate::popup::terminating::terminating;
use crate::popup::Popup;
use crate::sign::Sign;
use crate::view::{multi, single, Direction, OnClick, ToText as _, View};
use crate::{project, Action, Cell};
use arcstr::{literal, ArcStr};
use bitbag::BitBag;
use std::sync::{Arc, Weak};

const TITLE: ArcStr = literal!("select key");
const CANCEL: ArcStr = literal!("cancel");
const CONFIRM: ArcStr = literal!("confirm");

#[derive(Clone, Debug)]
pub struct KeySelector {
    pub info: PopupInfo,

    tonic: Arc<Cell<Chroma>>,
    sign: Arc<Cell<Sign>>,
    intervals: Arc<Cell<BitBag<KeyInterval>>>,
}

impl KeySelector {
    pub fn new(key: Key, this: Weak<Popup>) -> KeySelector {
        KeySelector {
            info: PopupInfo::new(TITLE, this),
            tonic: Arc::new(Cell::new(key.tonic)),
            sign: Arc::new(Cell::new(key.sign)),
            intervals: Arc::new(Cell::new(key.intervals)),
        }
    }

    fn key(&self) -> Key {
        Key {
            tonic: self.tonic.get(),
            sign: self.sign.get(),
            intervals: self.intervals.get(),
        }
    }

    pub fn view(&self) -> View {
        let buttons = View::spaced_stack(
            Direction::Right,
            vec![
                terminating(CANCEL.centred().bordered(), self.info.this()),
                terminating(
                    View::standard_button(
                        CONFIRM,
                        OnClick::from(Action::Project(project::Action::SetDefaultKey(self.key()))),
                    ),
                    self.info.this(),
                ),
            ],
        );

        View::spaced_stack(
            Direction::Down,
            vec![
                single::selector_with_formatter(&self.tonic, Direction::Right, |chroma| {
                    chroma.name(self.sign.get())
                }),
                single::selector(&self.sign, Direction::Right),
                multi::selector(&self.intervals, Direction::Right),
                buttons,
            ],
        )
    }
}
