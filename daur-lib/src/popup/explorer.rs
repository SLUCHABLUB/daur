use crate::app::Action;
use crate::clone_cell::ArcCell;
use crate::popup::info::PopupInfo;
use crate::popup::terminating::terminating;
use crate::popup::Popup;
use crate::view::{Direction, OnClick, View};
use arcstr::{literal, ArcStr};
use derive_more::Debug;
use std::path::Path;
use std::sync::{Arc, Weak};

const CANCEL: ArcStr = literal!("cancel");
const CONFIRM: ArcStr = literal!("confirm");

#[derive(Clone, Debug)]
pub struct ExplorerPopup {
    pub info: PopupInfo,
    pub selected_file: Arc<ArcCell<Path>>,
    #[debug(ignore)]
    pub action: Arc<dyn Fn(&Path) -> Action + Send + Sync>,
}

impl ExplorerPopup {
    pub fn new<A: Fn(&Path) -> Action + Send + Sync + 'static>(
        title: ArcStr,
        this: Weak<Popup>,
        selected_file: Arc<Path>,
        action: A,
    ) -> ExplorerPopup {
        ExplorerPopup {
            info: PopupInfo::new(title, this),
            selected_file: Arc::new(ArcCell::new(selected_file)),
            action: Arc::new(action),
        }
    }

    pub fn view(&self) -> View {
        let current_file = Arc::clone(&self.selected_file);
        let action = Arc::clone(&self.action);
        let selected_file = Arc::clone(&self.selected_file);

        let confirm = terminating(
            View::standard_button(
                CONFIRM,
                OnClick::new(move |_, _, _, actions| {
                    let path = current_file.get();
                    let action = action(&path);
                    actions.send(action);
                }),
            ),
            self.info.this(),
        );
        let cancel = terminating(View::centred(CANCEL).bordered(), self.info.this());

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
