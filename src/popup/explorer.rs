use crate::app::action::Action;
use crate::lock::Lock;
use crate::popup::button::Terminating;
use crate::popup::info::PopupInfo;
use crate::popup::Popup;
use crate::widget::bordered::Bordered;
use crate::widget::button::Button;
use crate::widget::heterogeneous::TwoStack;
use crate::widget::to_widget::ToWidget;
use educe::Educe;
use ratatui::layout::{Constraint, Flex};
use ratatui::widgets::Block;
use ratatui_explorer::{File, FileExplorer, Theme};
use std::sync::{Arc, Weak};

const CANCEL: &str = "cancel";
const CONFIRM: &str = "confirm";

fn theme() -> Theme {
    Theme::new()
        .with_block(Block::bordered())
        .add_default_title()
        .with_highlight_symbol("> ")
}

#[derive(Clone, Educe)]
#[educe(Eq, PartialEq)]
pub struct ExplorerPopup {
    pub info: PopupInfo,
    #[educe(Eq(ignore))]
    pub explorer: Lock<FileExplorer>,
    #[educe(Eq(ignore))]
    pub action: Arc<dyn Fn(&File) -> Action + Send + Sync>,
}

impl ExplorerPopup {
    pub fn new<A: Fn(&File) -> Action + Send + Sync + 'static>(
        title: String,
        this: Weak<Popup>,
        mut explorer: FileExplorer,
        action: A,
    ) -> ExplorerPopup {
        explorer.set_theme(theme());
        ExplorerPopup {
            info: PopupInfo::new(title, this),
            explorer: Lock::new(explorer),
            action: Arc::new(action),
        }
    }

    fn vertical_constraints() -> [Constraint; 2] {
        [Constraint::Fill(1), Constraint::Max(3)]
    }
}

impl ToWidget for ExplorerPopup {
    type Widget<'lock> = TwoStack<
        &'lock Lock<FileExplorer>,
        TwoStack<Terminating<Bordered<Button>>, Terminating<Bordered<Button>>>,
    >;

    fn to_widget(&self) -> Self::Widget<'_> {
        let action = (self.action)(self.explorer.read().current());

        let confirm = Terminating {
            child: Button::standard(CONFIRM, action),
            popup: self.info.this(),
        };
        let cancel = Terminating {
            child: Button::standard(CANCEL, Action::None),
            popup: self.info.this(),
        };

        let buttons = TwoStack::horizontal_sized((cancel, confirm)).flex(Flex::SpaceBetween);

        TwoStack::vertical((&self.explorer, buttons), Self::vertical_constraints())
    }
}
