use crate::app::action::Action;
use crate::popup::button::TerminatingButton;
use crate::popup::info::PopupInfo;
use crate::widget::button::Button;
use crate::widget::two_stack::TwoStack;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use min_max::max;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Flex, Layout, Position, Rect, Size};
use ratatui::prelude::Direction;
use ratatui::widgets::Block;
use ratatui_explorer::{File, FileExplorer, Theme};
use saturating_cast::SaturatingCast;
use std::sync::Arc;

const CANCEL: &str = "cancel";
const CONFIRM: &str = "confirm";

fn theme() -> Theme {
    Theme::new()
        .with_block(Block::bordered())
        .add_default_title()
        .with_highlight_symbol("> ")
}

#[derive(Clone)]
pub struct ExplorerPopup {
    pub info: PopupInfo,
    pub explorer: FileExplorer,
    pub action: Arc<dyn Fn(&File) -> Action + Send + Sync>,
}

impl ExplorerPopup {
    pub fn new(
        title: String,
        mut explorer: FileExplorer,
        action: impl Fn(&File) -> Action + Send + Sync + 'static,
    ) -> ExplorerPopup {
        explorer.set_theme(theme());
        ExplorerPopup {
            info: PopupInfo::new(title),
            explorer,
            action: Arc::new(action),
        }
    }

    pub fn size(&self) -> Size {
        // + 2 for the explorer's block, + 3 for the buttons
        let height = self.explorer.files().len().saturating_cast::<u16>() + 2 + 3;

        // +2 for blocks
        let button_width = CANCEL.chars().count() + 2 + CONFIRM.chars().count() + 2;
        let title_width = self.explorer.cwd().to_string_lossy().chars().count() + 2;
        let content_width = self
            .explorer
            .files()
            .iter()
            .map(|file| file.name().chars().count())
            .max()
            .unwrap_or(0)
            + 2;

        let width = max!(title_width, content_width, button_width).saturating_cast();

        Size { width, height }
    }

    fn vertical_constraints() -> [Constraint; 2] {
        [Constraint::Fill(1), Constraint::Max(3)]
    }

    fn to_widget(&self) -> impl Widget + use<'_> {
        let action = (self.action)(self.explorer.current());

        let cancel_size = CANCEL.chars().count().saturating_cast::<u16>() + 2;
        let confirm_size = CONFIRM.chars().count().saturating_cast::<u16>() + 2;

        let confirm = TerminatingButton {
            button: Button::new(CONFIRM, action).bordered(),
            id: self.info.id(),
        };
        let cancel = TerminatingButton {
            button: Button::new(CANCEL, Action::None).bordered(),
            id: self.info.id(),
        };

        let buttons = TwoStack::horizontal(
            (cancel, confirm),
            [Constraint::Max(cancel_size), Constraint::Max(confirm_size)],
        )
        .flex(Flex::SpaceBetween);

        TwoStack::vertical((&self.explorer, buttons), Self::vertical_constraints())
    }
}

impl Widget for ExplorerPopup {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        self.to_widget().render(area, buf, mouse_position);
    }

    fn click(
        &self,
        area: Rect,
        button: MouseButton,
        position: Position,
        action_queue: &mut Vec<Action>,
    ) {
        self.to_widget().click(area, button, position, action_queue);

        let [explorer_area, _] =
            Layout::new(Direction::Vertical, Self::vertical_constraints()).areas(area);

        let inner_area = self
            .explorer
            .theme()
            .block()
            .unwrap_or(&Block::new())
            .inner(explorer_area);

        if inner_area.contains(position) {
            let index = usize::from(position.y - inner_area.y);
            action_queue.push(Action::Select {
                popup: self.info.id(),
                index,
            });
        }
    }
}
