//! Types pertaining to [`Popup`].

mod id;
mod manager;

pub use id::Id;
pub use manager::Manager;

use crate::key::Key;
use crate::time::Instant;
use crate::view::{Alignment, Direction, OnClick, ToText as _, multi, single};
use crate::{Action, ArcCell, Cell, ToArcStr as _, UserInterface, View, project};
use arcstr::{ArcStr, format, literal};
use derive_more::Debug;
use dirs::home_dir;
use std::env::current_dir;
use std::error::Error;
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
        title: ArcStr,
        buttons: Vec<(ArcStr, Action)>,
    },
    /// An error message.
    Error { display: ArcStr, debug: ArcStr },
    /// A file selector.
    FileSelector {
        title: ArcStr,
        path: Arc<Path>,
        #[debug(skip)]
        action: Arc<dyn Fn(&Path) -> Action + Send + Sync>,
    },
    /// A window for selecting a key.
    KeySelector { instant: Instant, key: Key },
}

impl Popup {
    /// Constructs a new [file-explorer popup](Popup::FileSelector) starting at the current dir.
    pub fn explorer<A: Fn(&Path) -> Action + Send + Sync + 'static>(
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

    /// Returns the popups [view](View).
    pub fn view<Ui: UserInterface>(&self, id: Id) -> View {
        match self {
            Popup::ButtonPanel { title: _, buttons } => View::balanced_stack(
                Direction::Down,
                buttons.iter().map(|(label, action)| {
                    View::simple_button(label.clone(), OnClick::from(action.clone()))
                        .terminating(id)
                }),
            ),
            Popup::Error { display, debug } => {
                let acknowledge_button = ACKNOWLEDGE.centred().bordered();

                View::spaced_stack::<Ui>(
                    Direction::Down,
                    [
                        display.clone().aligned_to(Alignment::TopLeft),
                        debug.clone().aligned_to(Alignment::TopLeft),
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

                let buttons = View::spaced_stack::<Ui>(Direction::Right, vec![cancel, confirm]);

                View::Stack {
                    direction: Direction::Down,
                    elements: vec![
                        View::FileSelector { selected_file }.fill_remaining(),
                        buttons.quotated_minimally::<Ui>(),
                    ],
                }
            }
            Popup::KeySelector { instant, key } => {
                let tonic = Arc::new(Cell::new(key.tonic));
                let sign = Arc::new(Cell::new(key.sign));
                let intervals = Arc::new(Cell::new(key.intervals));

                let buttons = View::spaced_stack::<Ui>(
                    Direction::Right,
                    vec![
                        CANCEL.centred().bordered().terminating(id),
                        View::standard_button(
                            CONFIRM,
                            OnClick::from(Action::Project(project::Action::SetKey {
                                instant: *instant,
                                key: Key {
                                    tonic: tonic.get(),
                                    sign: sign.get(),
                                    intervals: intervals.get(),
                                },
                            })),
                        )
                        .terminating(id),
                    ],
                );

                View::spaced_stack::<Ui>(
                    Direction::Down,
                    vec![
                        single::selector_with_formatter(&tonic, Direction::Right, |chroma| {
                            chroma.name(sign.get())
                        }),
                        single::selector(&sign, Direction::Right),
                        multi::selector(&intervals, Direction::Right),
                        buttons,
                    ],
                )
            }
        }
    }
}

impl<E: Error> From<E> for Popup {
    fn from(error: E) -> Self {
        Popup::Error {
            display: error.to_arc_str(),
            debug: format!("{error:?}"),
        }
    }
}

impl View {
    /// Makes the view close a popup when clicked.
    fn terminating(self, popup: Id) -> View {
        self.on_click(OnClick::from(Action::ClosePopup(popup)))
    }
}
