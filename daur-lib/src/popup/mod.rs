mod error;
mod explorer;
mod info;
mod key;
mod panel;
mod popups;
mod terminating;

pub use popups::Popups;

use crate::key::Key;
use crate::popup::error::ErrorPopup;
use crate::popup::explorer::ExplorerPopup;
use crate::popup::info::PopupInfo;
use crate::popup::key::KeySelector;
use crate::popup::panel::ButtonPanel;
use crate::popup::terminating::Terminating;
use crate::ui::{Length, Point, Rectangle, Size};
use crate::widget::{Bordered, Button, HasSize as _, OnClick, ToWidget as _, Widget};
use crate::{keyboard, Action, Cell, Ratio};
use arcstr::ArcStr;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::widgets;
use ratatui::widgets::Clear;
use ratatui_explorer::{File, FileExplorer};
use std::error::Error;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub enum Popup {
    Buttons(ButtonPanel),
    Error(ErrorPopup),
    Explorer(ExplorerPopup),
    KeySelector(KeySelector),
}

impl Popup {
    pub fn buttons<B>(buttons: B) -> Arc<Popup>
    where
        B: IntoIterator<Item = (ArcStr, Action)>,
    {
        Self::_buttons(buttons, |this| PopupInfo::new(ArcStr::new(), this))
    }

    pub fn unimportant_buttons<B>(buttons: B) -> Arc<Popup>
    where
        B: IntoIterator<Item = (ArcStr, Action)>,
    {
        Self::_buttons(buttons, |this| {
            let mut info = PopupInfo::new(ArcStr::new(), this);
            info.unimportant = true;
            info
        })
    }

    fn _buttons<B>(buttons: B, info: impl FnOnce(Weak<Popup>) -> PopupInfo) -> Arc<Popup>
    where
        B: IntoIterator<Item = (ArcStr, Action)>,
    {
        Arc::new_cyclic(|this| {
            let info = info(Weak::clone(this));

            Popup::Buttons(ButtonPanel {
                buttons: buttons
                    .into_iter()
                    .map(|(name, action)| Terminating {
                        child: Button::simple(name, OnClick::from(action)),
                        popup: info.this(),
                    })
                    .collect(),
                info,
                selected: Cell::new(None),
            })
        })
    }

    pub fn error<E: Error>(error: E) -> Arc<Popup> {
        Arc::new_cyclic(|this| {
            let this = Weak::clone(this);
            Popup::Error(ErrorPopup::from_error(error, this))
        })
    }

    pub fn explorer<A: Fn(&File) -> Action + Send + Sync + 'static>(
        title: ArcStr,
        action: A,
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
        Arc::new_cyclic(|this| {
            let this = Weak::clone(this);
            Popup::KeySelector(KeySelector::new(key, this))
        })
    }

    pub fn at(self: Arc<Self>, position: Point) -> Arc<Self> {
        self.info().position.set(Some(position));
        self
    }

    pub fn info(&self) -> &PopupInfo {
        match self {
            Popup::Error(message) => &message.info,
            Popup::Explorer(explorer) => &explorer.info,
            Popup::Buttons(buttons) => &buttons.info,
            Popup::KeySelector(selector) => &selector.info,
        }
    }

    pub fn area_in_window(&self, area: Rectangle) -> Rectangle {
        let size = self.preferred_size();

        let position = if let Some(position) = self.info().position.get() {
            if area.contains(position) {
                position
            } else {
                Point {
                    x: area.position.x + area.size.width - size.width,
                    y: area.position.y + area.size.height - size.height,
                }
            }
        } else {
            Point {
                x: area.position.x + area.size.width * Ratio::HALF - size.width * Ratio::HALF,
                y: area.position.y + area.size.height * Ratio::HALF - size.height * Ratio::HALF,
            }
        };

        area.intersection(Rectangle { position, size })
    }

    fn preferred_size(&self) -> Size {
        let inner = self.preferred_inner_size();

        Size {
            width: inner.width + Length::DOUBLE_BORDER,
            height: inner.height + Length::DOUBLE_BORDER,
        }
    }

    fn preferred_inner_size(&self) -> Size {
        match self {
            Popup::Buttons(buttons) => buttons.to_widget().size(),
            Popup::Error(message) => message.to_widget().size(),
            Popup::Explorer(explorer) => explorer.to_widget().size(),
            Popup::KeySelector(selector) => selector.to_widget().size(),
        }
    }

    #[must_use]
    pub fn handle_key(&self, key: keyboard::Key, actions: &mut Vec<Action>) -> bool {
        match self {
            Popup::Buttons(panel) => panel.handle_key(key, actions),
            Popup::Error(error) => error.handle_key(key, actions),
            Popup::Explorer(explorer) => explorer.handle_key(key, actions),
            Popup::KeySelector(selector) => selector.handle_key(key, actions),
        }
    }
}

impl Widget for Popup {
    fn render(&self, area: Rectangle, buffer: &mut Buffer, mouse_position: Point) {
        widgets::Widget::render(Clear, area.to_rect(), buffer);
        let title = self.info().title.clone();

        match self {
            Popup::Buttons(buttons) => {
                Bordered::titled(title, buttons.to_widget()).render(area, buffer, mouse_position);
            }
            Popup::Error(message) => {
                Bordered::titled(title, message.to_widget()).render(area, buffer, mouse_position);
            }
            Popup::Explorer(explorer) => {
                Bordered::titled(title, explorer.to_widget()).render(area, buffer, mouse_position);
            }
            Popup::KeySelector(selector) => {
                Bordered::titled(title, selector.to_widget()).render(area, buffer, mouse_position);
            }
        }
    }

    fn click(
        &self,
        area: Rectangle,
        button: MouseButton,
        position: Point,
        actions: &mut Vec<Action>,
    ) {
        // Unimportant for clicking
        let title = ArcStr::new();

        match self {
            Popup::Buttons(buttons) => {
                Bordered::titled(title, buttons.to_widget()).click(area, button, position, actions);
            }
            Popup::Error(message) => {
                Bordered::titled(title, message.to_widget()).click(area, button, position, actions);
            }
            Popup::Explorer(explorer) => {
                Bordered::titled(title, explorer.to_widget())
                    .click(area, button, position, actions);
            }
            Popup::KeySelector(selector) => {
                Bordered::titled(title, selector.to_widget())
                    .click(area, button, position, actions);
            }
        }
    }
}
