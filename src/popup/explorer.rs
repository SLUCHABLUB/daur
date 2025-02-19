use crate::app::action::Action;
use crate::keyboard::Key;
use crate::lock::Lock;
use crate::popup::button::Terminating;
use crate::popup::info::PopupInfo;
use crate::popup::Popup;
use crate::widget::bordered::Bordered;
use crate::widget::button::Button;
use crate::widget::heterogeneous::TwoStack;
use crate::widget::macros::or_popup;
use crate::widget::to_widget::ToWidget;
use crossterm::event::KeyCode;
use educe::Educe;
use ratatui::layout::{Constraint, Flex};
use ratatui::widgets::Block;
use ratatui_explorer::{File, FileExplorer, Input, Theme};
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

    pub fn handle_key(&self, key: Key, actions: &mut Vec<Action>) -> bool {
        if key.code == KeyCode::Enter {
            let action = (self.action)(self.explorer.read().current());
            actions.push(action);
            true
        } else {
            let input = Input::from(&key.to_event());

            or_popup!(self.explorer.write().handle(input), actions);

            input != Input::None
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
