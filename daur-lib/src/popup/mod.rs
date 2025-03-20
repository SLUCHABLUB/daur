mod error;
mod explorer;
mod info;
mod key;
mod panel;
mod popups;
mod terminating;

pub use popups::Popups;
use std::env::current_dir;
pub use terminating::terminating;

use crate::key::Key;
use crate::popup::error::ErrorPopup;
use crate::popup::explorer::ExplorerPopup;
use crate::popup::info::PopupInfo;
use crate::popup::key::KeySelector;
use crate::popup::panel::ButtonPanel;
use crate::ui::{Point, Rectangle};
use crate::view::View;
use crate::{Action, Cell, Ratio};
use arcstr::ArcStr;
use dirs::home_dir;
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Weak};

/// A popup window.
#[derive(Debug)]
pub enum Popup {
    /// A panel of buttons.
    Buttons(ButtonPanel),
    /// An error message.
    Error(ErrorPopup),
    /// A file selector.
    Explorer(ExplorerPopup),
    /// A window for selecting a key.
    KeySelector(KeySelector),
}

impl Popup {
    /// Constructs a new button panel.
    pub fn buttons<B>(buttons: B) -> Arc<Popup>
    where
        B: IntoIterator<Item = (ArcStr, Action)>,
    {
        Self::_buttons(buttons, |this| PopupInfo::new(ArcStr::new(), this))
    }

    /// Constructs a new button panel that closes when unfocused.
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
                buttons: buttons.into_iter().collect(),
                info,
                selected: Cell::new(None),
            })
        })
    }

    /// Construct an [error-message popup](Popup::Error) from an [error](Error).
    pub fn error<E: Error>(error: E) -> Arc<Popup> {
        Arc::new_cyclic(|this| {
            let this = Weak::clone(this);
            Popup::Error(ErrorPopup::from_error(error, this))
        })
    }

    /// Constructs a new [file-explorer popup](Popup::Explorer).
    pub fn explorer<A: Fn(&Path) -> Action + Send + Sync + 'static>(
        title: ArcStr,
        action: A,
    ) -> Arc<Popup> {
        Arc::new_cyclic(|this| {
            let this = Weak::clone(this);

            let path = current_dir().ok().or_else(home_dir).unwrap_or_default();

            Popup::Explorer(ExplorerPopup::new(title, this, Arc::from(path), action))
        })
    }

    /// Constructs a new [key-selector popup](Popup::KeySelector).
    #[must_use]
    pub fn key_selector(key: Key) -> Arc<Popup> {
        Arc::new_cyclic(|this| {
            let this = Weak::clone(this);
            Popup::KeySelector(KeySelector::new(key, this))
        })
    }

    /// Sets the position of the popup.
    pub fn at(self: Arc<Self>, position: Point) -> Arc<Self> {
        self.info().position.set(Some(position));
        self
    }

    /// Returns the [popup info](PopupInfo).
    pub fn info(&self) -> &PopupInfo {
        match self {
            Popup::Error(message) => &message.info,
            Popup::Explorer(explorer) => &explorer.info,
            Popup::Buttons(buttons) => &buttons.info,
            Popup::KeySelector(selector) => &selector.info,
        }
    }

    /// Calculates the rectangle encompassing the popup.
    pub fn area_in_window(&self, area: Rectangle) -> Rectangle {
        let size = self.view().minimum_size();

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

        Rectangle { position, size }
    }

    /// Returns the popups [view](View).
    pub fn view(&self) -> View {
        let title = self.info().title();

        match self {
            Popup::Buttons(buttons) => buttons.view().titled(title),
            Popup::Error(message) => message.view().titled(title),
            Popup::Explorer(explorer) => explorer.view().titled(title),
            Popup::KeySelector(selector) => selector.view().titled(title),
        }
    }
}
