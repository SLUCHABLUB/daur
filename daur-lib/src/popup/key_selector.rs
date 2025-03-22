use crate::chroma::Chroma;
use crate::key::{Key, KeyInterval};
use crate::popup::Popup;
use crate::popup::info::Info;
use crate::sign::Sign;
use crate::view::{Direction, OnClick, ToText as _, View, multi, single};
use crate::{Action, Cell, project};
use arcstr::{ArcStr, literal};
use bitbag::BitBag;
use std::sync::{Arc, Weak};

const TITLE: ArcStr = literal!("select key");
const CANCEL: ArcStr = literal!("cancel");
const CONFIRM: ArcStr = literal!("confirm");

/// A key selector.
#[derive(Clone, Debug)]
pub struct KeySelector {
    /// The popup info.
    pub info: Info,

    /// The currently selected tonic.
    pub tonic: Arc<Cell<Chroma>>,
    /// The currently selected sign.
    pub sign: Arc<Cell<Sign>>,
    /// The currently selected intervals.
    pub intervals: Arc<Cell<BitBag<KeyInterval>>>,
}

impl KeySelector {
    /// Constructs a new key selector.
    #[must_use]
    pub fn new(key: Key, this: Weak<Popup>) -> KeySelector {
        KeySelector {
            info: Info::new(TITLE, this),
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

    pub(super) fn view(&self) -> View {
        let buttons = View::spaced_stack(
            Direction::Right,
            vec![
                CANCEL.centred().bordered().terminating(self.info.this()),
                View::standard_button(
                    CONFIRM,
                    OnClick::from(Action::Project(project::Action::SetDefaultKey(self.key()))),
                )
                .terminating(self.info.this()),
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
