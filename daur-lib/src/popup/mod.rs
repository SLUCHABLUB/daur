//! Types pertaining to [`Popup`].

mod id;
mod instance;
mod manager;

pub use id::Id;
pub(crate) use instance::Instance;
pub(crate) use manager::Manager;

use crate::metre::Instant;
use crate::notes::Key;
use crate::sync::{ArcCell, Cell};
use crate::ui::Rectangle;
use crate::view::{Alignment, Axis, OnClick, ToText as _, file_selector, multi, single};
use crate::{Action, Ratio, UserInterface, View, project};
use anyhow::Error;
use arcstr::{ArcStr, format, literal};
use closure::closure;
use derive_more::Debug;
use dirs::home_dir;
use std::env::current_dir;
use std::path::Path;
use std::sync::Arc;

const ACKNOWLEDGE: ArcStr = literal!("ok");
const CANCEL: ArcStr = literal!("cancel");
const CONFIRM: ArcStr = literal!("confirm");

const ERROR_TITLE: ArcStr = literal!("error");
const KEY_SELECTOR_TITLE: ArcStr = literal!("select key");

// TODO: keyboard navigation of popups
/// A specification for a popup window.
#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum Popup {
    /// A panel of buttons.
    ButtonPanel {
        /// The title of the popup.
        title: ArcStr,
        // TODO: make this non empty
        /// The buttons.
        buttons: Vec<(ArcStr, Action)>,
    },
    /// An error message.
    Error(Arc<Error>),
    /// A file selector.
    FileSelector {
        /// The title of the popup.
        title: ArcStr,
        /// The starting path.
        path: Arc<Path>,
        /// The function for generating the action.
        #[debug(skip)]
        action: Arc<dyn Fn(&Path) -> Action + Send + Sync>,
    },
    /// A window for selecting a key.
    KeySelector {
        /// The instant at which the key should be changed.
        instant: Instant,
        /// The current key.
        key: Key,
    },
}

impl Popup {
    /// Constructs a new [file-section popup](Popup::FileSelector) starting at the current dir.
    pub fn file_selector<A: Fn(&Path) -> Action + Send + Sync + 'static>(
        title: ArcStr,
        action: A,
    ) -> Popup {
        let path = current_dir().ok().or_else(home_dir).unwrap_or_default();

        Popup::FileSelector {
            title,
            path: Arc::from(path),
            action: Arc::new(action),
        }
    }

    /// Returns the title of the popup.
    #[must_use]
    pub fn title(&self) -> ArcStr {
        match self {
            Popup::ButtonPanel { title, .. } | Popup::FileSelector { title, .. } => title.clone(),
            Popup::Error { .. } => ERROR_TITLE,
            // TODO: Display at what instant the key is being set.
            Popup::KeySelector { .. } => KEY_SELECTOR_TITLE,
        }
    }

    /// Returns the popups [view](View) with a border and title.
    fn view<Ui: UserInterface>(&self, id: Id) -> View {
        self.inner_view::<Ui>(id)
            .bordered()
            .titled(self.title())
            .on_click(OnClick::from(Action::CloseContextMenu))
    }

    /// Returns the popups inner [view](View), with no border and title.
    fn inner_view<Ui: UserInterface>(&self, id: Id) -> View {
        match self {
            Popup::ButtonPanel { title: _, buttons } => View::balanced_stack::<Ui, _>(
                Axis::Y,
                buttons.iter().map(|(label, action)| {
                    View::simple_button(label.clone(), OnClick::from(action.clone()))
                        .terminating(id)
                }),
            ),
            Popup::Error(error) => {
                let acknowledge_button = ACKNOWLEDGE.centred().bordered();

                View::spaced_stack::<Ui, _>(
                    Axis::Y,
                    [
                        format!("{error:#}").aligned_to(Alignment::TopLeft),
                        acknowledge_button.terminating(id),
                    ],
                )
            }
            Popup::FileSelector {
                title: _,
                path,
                action,
            } => {
                let selected_file = Arc::new(ArcCell::new(Arc::clone(path)));

                let path = Arc::clone(&selected_file);
                let action = Arc::clone(action);

                let confirm = View::standard_button(
                    CONFIRM,
                    OnClick::new(move |_, _, actions| {
                        let path = path.get();
                        let action = action(&path);
                        actions.send(action);
                    }),
                )
                .terminating(id);
                let cancel = CANCEL.centred().bordered().terminating(id);

                let buttons = View::spaced_stack::<Ui, _>(Axis::X, vec![cancel, confirm]);

                View::y_stack([
                    file_selector::<Ui>(&selected_file).fill_remaining(),
                    buttons.quotated_minimally::<Ui>(),
                ])
            }
            Popup::KeySelector { instant, key } => {
                let instant = *instant;

                let tonic = Arc::new(Cell::new(key.tonic));
                let sign = Arc::new(Cell::new(key.sign));
                let intervals = Arc::new(Cell::new(key.intervals));

                let buttons = View::spaced_stack::<Ui, _>(
                    Axis::X,
                    vec![
                        CANCEL.centred().bordered().terminating(id),
                        View::standard_button(
                            CONFIRM,
                            OnClick::action(
                                closure!([clone tonic, clone sign, clone intervals] move || {
                                    Action::Project(project::Action::SetKey {
                                        instant,
                                        key: Key {
                                            tonic: tonic.get(),
                                            sign: sign.get(),
                                            intervals: intervals.get(),
                                        },
                                    })
                                }),
                            ),
                        )
                        .terminating(id),
                    ],
                );

                View::spaced_stack::<Ui, _>(
                    Axis::Y,
                    vec![
                        single::selector_with_formatter::<Ui, _, _>(
                            &tonic,
                            Axis::X,
                            closure!([clone sign] move |chroma| {
                                chroma.name(sign.get())
                            }),
                        ),
                        single::selector::<Ui, _>(&sign, Axis::X),
                        multi::selector::<Ui, _>(&intervals, Axis::X),
                        buttons,
                    ],
                )
            }
        }
        .on_click(OnClick::from(Action::CloseContextMenu))
    }

    pub(crate) fn instantiate<Ui: UserInterface>(&self, id: Id, ui: &Ui) -> Instance {
        let view = Arc::new(self.view::<Ui>(id));

        let size = view.minimum_size::<Ui>();
        let centre = (ui.size().diagonal() * Ratio::HALF).point();
        let offset = -(size.diagonal() * Ratio::HALF);
        let position = centre + offset;
        let area = Rectangle { position, size };

        Instance::new(id, area, view)
    }
}

impl<E: Into<Error>> From<E> for Popup {
    fn from(error: E) -> Self {
        Popup::Error(Arc::new(error.into()))
    }
}

impl View {
    /// Makes the view close a popup when clicked.
    fn terminating(self, popup: Id) -> View {
        self.on_click(OnClick::from(Action::ClosePopup(popup)))
    }
}
