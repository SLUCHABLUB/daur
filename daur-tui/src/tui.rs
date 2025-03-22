use crate::SHOULD_EXIT;
use daur::Ui;

pub struct Tui;

impl Ui for Tui {
    fn exit(&self) {
        SHOULD_EXIT.set(true);
    }
}
