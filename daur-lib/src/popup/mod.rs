//! Types pertaining to [`Popup`].

mod button_panel;
mod error_message;
mod file_selector;
mod info;
mod key_selector;
mod popups;

pub use button_panel::ButtonPanel;
pub use error_message::ErrorMessage;
pub use file_selector::FileSelector;
pub use info::Info;
pub use key_selector::KeySelector;
pub use popups::Manager;

use crate::key::Key;
use crate::view::OnClick;
use crate::{Action, Cell, View};
use arcstr::ArcStr;
use dirs::home_dir;
use std::env::current_dir;
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Weak};

/// A popup window.
#[doc(hidden)]
#[derive(Debug)]
pub enum Popup {
    /// A panel of buttons.
    Buttons(ButtonPanel),
    /// An error message.
    Error(ErrorMessage),
    /// A file selector.
    FileSelector(FileSelector),
    /// A window for selecting a key.
    KeySelector(KeySelector),
}

impl Popup {
    /// Constructs a new button panel.
    pub fn buttons<B>(buttons: B) -> Arc<Popup>
    where
        B: IntoIterator<Item = (ArcStr, Action)>,
    {
        Arc::new_cyclic(|this| {
            let info = Info::new(ArcStr::new(), Weak::clone(this));

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
            Popup::Error(ErrorMessage::from_error(error, this))
        })
    }

    /// Constructs a new [file-explorer popup](Popup::FileSelector).
    pub fn explorer<A: Fn(&Path) -> Action + Send + Sync + 'static>(
        title: ArcStr,
        action: A,
    ) -> Arc<Popup> {
        Arc::new_cyclic(|this| {
            let this = Weak::clone(this);

            let path = current_dir().ok().or_else(home_dir).unwrap_or_default();

            Popup::FileSelector(FileSelector::new(title, this, Arc::from(path), action))
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

    /// Returns the [popup info](Info).
    pub fn info(&self) -> &Info {
        match self {
            Popup::Error(message) => &message.info,
            Popup::FileSelector(explorer) => &explorer.info,
            Popup::Buttons(buttons) => &buttons.info,
            Popup::KeySelector(selector) => &selector.info,
        }
    }

    /// Returns the popups [view](View).
    pub fn view(&self) -> View {
        let title = self.info().title();

        match self {
            Popup::Buttons(buttons) => buttons.view().titled(title),
            Popup::Error(message) => message.view().titled(title),
            Popup::FileSelector(explorer) => explorer.view().titled(title),
            Popup::KeySelector(selector) => selector.view().titled(title),
        }
    }
}

impl View {
    /// Makes the view close a popup when clicked.
    fn terminating(self, popup: Weak<Popup>) -> View {
        self.on_click(OnClick::from(Action::ClosePopup(popup)))
    }
}
