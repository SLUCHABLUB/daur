use crate::SHOULD_EXIT;
use daur::UserInterface;

pub struct Tui;

impl UserInterface for Tui {
    fn exit(&self) {
        SHOULD_EXIT.set(true);
    }
}
