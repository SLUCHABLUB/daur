use crate::app::action::Action;
use crate::lock::Lock;
use crate::popup::button::TerminatingButton;
use crate::popup::info::PopupInfo;
use crate::popup::Popup;
use crate::widget::button::Button;
use crate::widget::heterogeneous_stack::TwoStack;
use crate::widget::to_widget::ToWidget;
use educe::Educe;
use ratatui::layout::{Constraint, Flex};
use ratatui::widgets::Block;
use ratatui_explorer::{File, FileExplorer, Theme};
use saturating_cast::SaturatingCast;
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
    pub fn new(
        title: String,
        this: Weak<Popup>,
        mut explorer: FileExplorer,
        action: impl Fn(&File) -> Action + Send + Sync + 'static,
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
    type Widget<'a> =
        TwoStack<&'a Lock<FileExplorer>, TwoStack<TerminatingButton, TerminatingButton>>;

    fn to_widget(&self) -> Self::Widget<'_> {
        let action = (self.action)(self.explorer.read().current());

        let cancel_size = CANCEL.chars().count().saturating_cast::<u16>() + 2;
        let confirm_size = CONFIRM.chars().count().saturating_cast::<u16>() + 2;

        let confirm = TerminatingButton {
            button: Button::new(CONFIRM, action).bordered(),
            popup: self.info.this(),
        };
        let cancel = TerminatingButton {
            button: Button::new(CANCEL, Action::None).bordered(),
            popup: self.info.this(),
        };

        let buttons = TwoStack::horizontal(
            (cancel, confirm),
            [Constraint::Max(cancel_size), Constraint::Max(confirm_size)],
        )
        .flex(Flex::SpaceBetween);

        TwoStack::vertical((&self.explorer, buttons), Self::vertical_constraints())
    }
}
