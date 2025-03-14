use crate::app::Action;
use crate::keyboard::Key;
use crate::lock::Lock;
use crate::popup::info::PopupInfo;
use crate::popup::terminating::Terminating;
use crate::popup::Popup;
use crate::widget::heterogeneous::TwoStack;
use crate::widget::{or_popup, Bordered, Button, OnClick, Text, ToWidget};
use arcstr::{literal, ArcStr};
use crossterm::event::KeyCode;
use educe::Educe;
use ratatui::layout::{Constraint, Flex};
use ratatui::widgets::Block;
use ratatui_explorer::{File, FileExplorer, Input, Theme};
use std::sync::{Arc, Weak};

const CANCEL: ArcStr = literal!("cancel");
const CONFIRM: ArcStr = literal!("confirm");

fn theme() -> Theme {
    Theme::new()
        .with_block(Block::bordered())
        .add_default_title()
        .with_highlight_symbol("> ")
}

#[derive(Clone, Educe)]
#[educe(Eq, PartialEq, Debug)]
pub struct ExplorerPopup {
    pub info: PopupInfo,
    #[educe(Eq(ignore))]
    pub explorer: Lock<FileExplorer>,
    #[educe(Eq(ignore), Debug(ignore))]
    pub action: Arc<dyn Fn(&File) -> Action + Send + Sync>,
}

impl ExplorerPopup {
    pub fn new<A: Fn(&File) -> Action + Send + Sync + 'static>(
        title: ArcStr,
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
        [Constraint::Fill(1), Constraint::Length(3)]
    }
}

impl ToWidget for ExplorerPopup {
    type Widget<'lock> = TwoStack<
        &'lock Lock<FileExplorer>,
        TwoStack<Terminating<Bordered<Text>>, Terminating<Button<'static, Bordered<Text>>>>,
    >;

    fn to_widget(&self) -> Self::Widget<'_> {
        let action = (self.action)(self.explorer.read().current());

        let confirm = Terminating {
            child: Button::standard(CONFIRM, OnClick::from(action)),
            popup: self.info.this(),
        };
        let cancel = Terminating {
            child: Bordered::plain(Text::centred(CANCEL)),
            popup: self.info.this(),
        };

        let buttons = TwoStack::horizontal_sized((cancel, confirm)).flex(Flex::SpaceBetween);

        TwoStack::vertical((&self.explorer, buttons), Self::vertical_constraints())
    }
}
