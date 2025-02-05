use crate::app::action::Action;
use crate::popup::button::TerminatingButton;
use crate::popup::error::ErrorPopup;
use crate::popup::explorer::ExplorerPopup;
use crate::popup::info::PopupInfo;
use crate::popup::panel::ButtonPanel;
use crate::widget::button::Button;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect, Size};
use ratatui::widgets::Block;
use ratatui_explorer::{File, FileExplorer};
use std::error::Error;

mod button;
mod error;
mod explorer;
mod info;
mod panel;

#[derive(Clone, Debug)]
pub enum Popup {
    Buttons(ButtonPanel),
    Error(ErrorPopup),
    Explorer(ExplorerPopup),
}

impl Popup {
    pub fn unimportant_buttons(buttons: impl IntoIterator<Item = (&'static str, Action)>) -> Popup {
        let mut info = PopupInfo::new(String::new());
        info.unimportant = true;
        let uuid = info.id();

        Popup::Buttons(ButtonPanel {
            info,
            buttons: buttons
                .into_iter()
                .map(|(name, action)| TerminatingButton {
                    button: Button::new(name, action),
                    id: uuid,
                })
                .collect(),
            unimportant: true,
        })
    }

    pub fn explorer(
        title: String,
        action: impl Fn(&File) -> Action + Send + Sync + 'static,
    ) -> Popup {
        match FileExplorer::new() {
            Ok(explorer) => Popup::Explorer(ExplorerPopup::new(title, explorer, action)),
            Err(error) => Popup::Error(ErrorPopup::from(error)),
        }
    }

    fn preferred_size(&self) -> Size {
        let inner = self.preferred_inner_size();

        Size {
            width: inner.width + 2,
            height: inner.height + 2,
        }
    }

    fn preferred_inner_size(&self) -> Size {
        match self {
            Popup::Buttons(buttons) => buttons.size(),
            Popup::Error(message) => message.size(),
            Popup::Explorer(explorer) => explorer.size,
        }
    }

    pub fn info(&self) -> &PopupInfo {
        match self {
            Popup::Error(message) => &message.info,
            Popup::Explorer(explorer) => &explorer.info,
            Popup::Buttons(buttons) => &buttons.info,
        }
    }

    fn info_mut(&mut self) -> &mut PopupInfo {
        match self {
            Popup::Buttons(buttons) => &mut buttons.info,
            Popup::Error(message) => &mut message.info,
            Popup::Explorer(explorer) => &mut explorer.info,
        }
    }

    pub fn at(mut self, position: Position) -> Self {
        self.info_mut().position = Some(position);
        self
    }

    pub fn unimportant(&self) -> bool {
        match self {
            Popup::Buttons(buttons) => buttons.unimportant,
            Popup::Error(_) | Popup::Explorer(_) => false,
        }
    }

    pub fn area_in_window(&self, area: Rect) -> Rect {
        let size = self.preferred_size();

        let position = if let Some(position) = self.info().position {
            if area.contains(position) {
                position
            } else {
                Position {
                    x: (area.x + area.width).saturating_sub(size.width + 1),
                    y: (area.y + area.height).saturating_sub(size.height + 1),
                }
            }
        } else {
            Position {
                x: area.x + (area.width / 2).saturating_sub(size.width / 2),
                y: area.y + (area.height / 2).saturating_sub(size.height / 2),
            }
        };

        area.intersection(Rect {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        })
    }
}

impl<E: Error> From<E> for Popup {
    fn from(error: E) -> Self {
        Popup::Error(ErrorPopup::from(error))
    }
}

impl Widget for Popup {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        let block = Block::bordered().title(self.info().title.as_str());
        block.render(area, buf, mouse_position);

        let area = block.inner(area);

        match self {
            Popup::Buttons(buttons) => buttons.to_widget().render(area, buf, mouse_position),
            Popup::Error(message) => message.to_widget().render(area, buf, mouse_position),
            Popup::Explorer(explorer) => explorer.to_widget().render(area, buf, mouse_position),
        }
    }

    fn click(
        &self,
        area: Rect,
        button: MouseButton,
        position: Position,
        action_queue: &mut Vec<Action>,
    ) {
        // TODO: scale popup
        let area = Block::bordered().inner(area);

        match self {
            Popup::Buttons(buttons) => {
                buttons
                    .to_widget()
                    .click(area, button, position, action_queue);
            }
            Popup::Error(message) => {
                message
                    .to_widget()
                    .click(area, button, position, action_queue);
            }
            Popup::Explorer(explorer) => {
                explorer
                    .to_widget()
                    .click(area, button, position, action_queue);
            }
        }
    }
}
