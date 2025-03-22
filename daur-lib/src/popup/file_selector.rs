use crate::ArcCell;
use crate::app::Action;
use crate::popup::Popup;
use crate::popup::info::Info;
use crate::view::{Direction, OnClick, ToText as _, View};
use arcstr::{ArcStr, literal};
use derive_more::Debug;
use std::path::Path;
use std::sync::{Arc, Weak};

const CANCEL: ArcStr = literal!("cancel");
const CONFIRM: ArcStr = literal!("confirm");

/// A file selector.
#[derive(Clone, Debug)]
pub struct FileSelector {
    /// The popup info.
    pub info: Info,
    /// The currently selected file.
    pub selected_file: Arc<ArcCell<Path>>,
    /// The action to take when the confirmation button is pressed.
    #[debug(ignore)]
    pub action: Arc<dyn Fn(&Path) -> Action + Send + Sync>,
}

impl FileSelector {
    /// Constructs a new file selector.
    pub fn new<A: Fn(&Path) -> Action + Send + Sync + 'static>(
        title: ArcStr,
        this: Weak<Popup>,
        selected_file: Arc<Path>,
        action: A,
    ) -> FileSelector {
        FileSelector {
            info: Info::new(title, this),
            selected_file: Arc::new(ArcCell::new(selected_file)),
            action: Arc::new(action),
        }
    }

    pub(super) fn view(&self) -> View {
        let current_file = Arc::clone(&self.selected_file);
        let action = Arc::clone(&self.action);
        let selected_file = Arc::clone(&self.selected_file);

        let confirm = View::standard_button(
            CONFIRM,
            OnClick::new(move |_, _, actions| {
                let path = current_file.get();
                let action = action(&path);
                actions.send(action);
            }),
        )
        .terminating(self.info.this());
        let cancel = CANCEL.centred().bordered().terminating(self.info.this());

        let buttons = View::spaced_stack(Direction::Right, vec![cancel, confirm]);

        View::Stack {
            direction: Direction::Down,
            elements: vec![
                View::FileSelector { selected_file }.fill_remaining(),
                buttons.quotated_minimally(),
            ],
        }
    }
}
