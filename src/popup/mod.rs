use crate::app::action::Action;
use crate::key::Key;
use crate::length::point::Point;
use crate::length::rectangle::Rectangle;
use crate::length::size::Size;
use crate::length::Length;
use crate::popup::button::Terminating;
use crate::popup::error::ErrorPopup;
use crate::popup::explorer::ExplorerPopup;
use crate::popup::info::PopupInfo;
use crate::popup::key::KeySelector;
use crate::popup::panel::ButtonPanel;
use crate::widget::block::Bordered;
use crate::widget::button::Button;
use crate::widget::sized::Sized as _;
use crate::widget::to_widget::ToWidget as _;
use crate::widget::Widget;
use crossterm::event::MouseButton;
use ratatui::buffer::Buffer;
use ratatui::widgets::Clear;
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
                    .map(|(name, action)| Terminating {
                        child: Button::simple(name.into().as_ref(), action),
                        popup: info.this(),
                    })
                    .collect(),
                info,
            })
        })
    }

    pub fn explorer<A: Fn(&File) -> Action + Send + Sync + 'static>(
        title: String,
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
        Arc::new_cyclic(|this| Popup::KeySelector(KeySelector::new(key, Weak::clone(this))))
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

    pub fn info(&self) -> &PopupInfo {
        match self {
            Popup::Error(message) => &message.info,
            Popup::Explorer(explorer) => &explorer.info,
            Popup::Buttons(buttons) => &buttons.info,
            Popup::KeySelector(selector) => &selector.info,
        }
    }

    pub fn from_error<E: Error>(error: E) -> Arc<Popup> {
        Arc::new_cyclic(|this| Popup::Error(ErrorPopup::from_error(error, Weak::clone(this))))
    }

    pub fn at(self: Arc<Self>, position: Point) -> Arc<Self> {
        self.info().position.set(Some(position));
        self
    }

    pub fn area_in_window(&self, area: Rectangle) -> Rectangle {
        let size = self.preferred_size();

        let position = if let Some(position) = self.info().position.get() {
            if area.contains(position) {
                position
            } else {
                Point {
                    x: area.x + area.width - size.width,
                    y: area.y + area.height - size.height,
                }
            }
        } else {
            Point {
                x: area.x + area.width / 2 - size.width / 2,
                y: area.y + area.height / 2 - size.height / 2,
            }
        };

        area.intersection(Rectangle {
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        })
    }
}

impl Widget for Popup {
    fn render(&self, area: Rectangle, buf: &mut Buffer, mouse_position: Point) {
        Clear.render(area, buf, mouse_position);
        let title = self.info().title.clone();

        match self {
            Popup::Buttons(buttons) => {
                Bordered::plain(title, buttons.to_widget()).render(area, buf, mouse_position);
            }
            Popup::Error(message) => {
                Bordered::plain(title, message.to_widget()).render(area, buf, mouse_position);
            }
            Popup::Explorer(explorer) => {
                Bordered::plain(title, explorer.to_widget()).render(area, buf, mouse_position);
            }
            Popup::KeySelector(selector) => {
                Bordered::plain(title, selector.to_widget()).render(area, buf, mouse_position);
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
        let title = "";

        match self {
            Popup::Buttons(buttons) => {
                Bordered::plain(title, buttons.to_widget()).click(area, button, position, actions);
            }
            Popup::Error(message) => {
                Bordered::plain(title, message.to_widget()).click(area, button, position, actions);
            }
            Popup::Explorer(explorer) => {
                Bordered::plain(title, explorer.to_widget()).click(area, button, position, actions);
            }
            Popup::KeySelector(selector) => {
                Bordered::plain(title, selector.to_widget()).click(area, button, position, actions);
            }
        }
    }
}
