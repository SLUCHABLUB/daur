use crate::app::action::Action;
use crate::popup::button::TerminatingButton;
use crate::popup::info::PopupInfo;
use crate::widget::button::Button;
use crate::widget::two_stack::TwoStack;
use crate::widget::Widget;
use educe::Educe;
use ratatui::layout::{Constraint, Flex, Size};
use ratatui_explorer::{File, FileExplorer};
use saturating_cast::SaturatingCast;
use std::sync::Arc;

const DEFAULT_SIZE: Size = Size::new(40, 10);

const CANCEL: &str = "cancel";
const CONFIRM: &str = "confirm";

const CANCEL_BUTTON: Button = Button::new(CANCEL, Action::None).bordered();

#[derive(Clone, Educe)]
#[educe(Debug)]
pub struct ExplorerPopup {
    pub info: PopupInfo,
    pub explorer: FileExplorer,
    pub size: Size,
    #[educe(Debug(ignore))]
    pub action: Arc<dyn Fn(&File) -> Action + Send + Sync>,
}

impl ExplorerPopup {
    pub fn new(
        title: String,
        explorer: FileExplorer,
        action: impl Fn(&File) -> Action + Send + Sync + 'static,
    ) -> ExplorerPopup {
        ExplorerPopup {
            info: PopupInfo::new(title),
            explorer,
            size: DEFAULT_SIZE,
            action: Arc::new(action),
        }
    }

    pub fn to_widget(&self) -> impl Widget + use<'_> {
        let action = (self.action)(self.explorer.current());

        let cancel_size = CANCEL.chars().count().saturating_cast::<u16>() + 2;
        let confirm_size = CONFIRM.chars().count().saturating_cast::<u16>() + 2;

        let confirm = TerminatingButton {
            button: Button::new(CONFIRM, action).bordered(),
            id: self.info.id(),
        };
        let cancel = TerminatingButton {
            button: CANCEL_BUTTON,
            id: self.info.id(),
        };

        let buttons = TwoStack::horizontal(
            (cancel, confirm),
            [Constraint::Max(cancel_size), Constraint::Max(confirm_size)],
        )
        .flex(Flex::SpaceBetween);

        TwoStack::vertical(
            (&self.explorer, buttons),
            [Constraint::Fill(1), Constraint::Max(3)],
        )
    }
}
