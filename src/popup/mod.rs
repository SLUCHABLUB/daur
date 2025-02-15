use crate::app::action::Action;
use crate::key::Key;
use crate::popup::button::TerminatingButton;
use crate::popup::error::ErrorPopup;
use crate::popup::explorer::ExplorerPopup;
use crate::popup::info::PopupInfo;
use crate::popup::key::KeySelector;
use crate::popup::panel::ButtonPanel;
use crate::widget::button::Button;
use crate::widget::sized::Sized;
use crate::widget::to_widget::ToWidget;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect, Size};
use ratatui::widgets::{Block, Clear};
use ratatui_explorer::{File, FileExplorer};
use std::borrow::Cow;
use std::error::Error;
use std::sync::{Arc, Weak};

mod button;
mod error;
mod explorer;
mod info;
mod key;
mod panel;

#[derive(Clone, Eq, PartialEq)]
pub enum Popup {
    Buttons(ButtonPanel),
    Error(ErrorPopup),
    Explorer(ExplorerPopup),
    KeySelector(KeySelector),
}

impl Popup {
    pub fn buttons<B, S>(buttons: B) -> Arc<Popup>
    where
        B: IntoIterator<Item = (S, Action)>,
        S: Into<Cow<'static, str>>,
    {
        Self::_buttons(buttons, |this| PopupInfo::new(String::new(), this))
    }

    pub fn unimportant_buttons<B, S>(buttons: B) -> Arc<Popup>
    where
        B: IntoIterator<Item = (S, Action)>,
        S: Into<Cow<'static, str>>,
    {
        Self::_buttons(buttons, |this| {
            let mut info = PopupInfo::new(String::new(), this);
            info.unimportant = true;
            info
        })
    }

    fn _buttons<B, S>(buttons: B, info: impl FnOnce(Weak<Popup>) -> PopupInfo) -> Arc<Popup>
    where
        B: IntoIterator<Item = (S, Action)>,
        S: Into<Cow<'static, str>>,
    {
        Arc::new_cyclic(|this| {
            let info = info(Weak::clone(this));

            Popup::Buttons(ButtonPanel {
                buttons: buttons
                    .into_iter()
                    .map(|(name, action)| TerminatingButton {
                        button: Button::new(name.into().as_ref(), action),
                        popup: info.this(),
                    })
                    .collect(),
                info,
            })
        })
    }

    pub fn explorer(
        title: String,
        action: impl Fn(&File) -> Action + Send + Sync + 'static,
    ) -> Arc<Popup> {
        Arc::new_cyclic(|this| {
            let this = Weak::clone(this);
            match FileExplorer::new() {
                Ok(explorer) => Popup::Explorer(ExplorerPopup::new(title, this, explorer, action)),
                Err(error) => Popup::Error(ErrorPopup::from_error(error, this)),
            }
        })
    }

    pub fn key_selector(key: Key) -> Arc<Popup> {
        Arc::new_cyclic(|this| Popup::KeySelector(KeySelector::new(key, Weak::clone(this))))
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
            Popup::Explorer(explorer) => explorer.to_widget().size(),
            Popup::KeySelector(selector) => selector.to_widget().size(),
        }
    }

    pub fn info(&self) -> &PopupInfo {
        match self {
            Popup::Error(message) => &message.info,
            Popup::Explorer(explorer) => &explorer.info,
            Popup::Buttons(buttons) => &buttons.info,
            Popup::KeySelector(selector) => &selector.info,
        }
    }

    pub fn from_error(error: impl Error) -> Arc<Popup> {
        Arc::new_cyclic(|this| Popup::Error(ErrorPopup::from_error(error, Weak::clone(this))))
    }

    pub fn at(self: Arc<Self>, position: Position) -> Arc<Self> {
        self.info().position.set(Some(position));
        self
    }

    pub fn area_in_window(&self, area: Rect) -> Rect {
        let size = self.preferred_size();

        let position = if let Some(position) = self.info().position.get() {
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

impl Widget for Popup {
    fn render(&self, area: Rect, buf: &mut Buffer, mouse_position: Position) {
        Clear.render(area, buf, mouse_position);
        let block = Block::bordered().title(self.info().title.as_str());
        block.render(area, buf, mouse_position);

        let area = block.inner(area);

        match self {
            Popup::Buttons(buttons) => buttons.to_widget().render(area, buf, mouse_position),
            Popup::Error(message) => message.to_widget().render(area, buf, mouse_position),
            Popup::Explorer(explorer) => explorer.to_widget().render(area, buf, mouse_position),
            Popup::KeySelector(selector) => selector.to_widget().render(area, buf, mouse_position),
        }
    }

    fn click(
        &self,
        area: Rect,
        button: MouseButton,
        position: Position,
        action_queue: &mut Vec<Action>,
    ) {
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
            Popup::KeySelector(selector) => {
                selector
                    .to_widget()
                    .click(area, button, position, action_queue);
            }
        }
    }
}
