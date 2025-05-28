use crate::app::Action;
use crate::metre::Instant;
use crate::note::Key;
use crate::popup::Popup;
use crate::sync::{ArcCell, Cell};
use crate::ui::{Point, Rectangle};
use crate::view::{Alignment, Axis, OnClick, ToText as _, file_selector, multi, single};
use crate::{Id, Ratio, UserInterface, View, project};
use anyhow::Error;
use arcstr::{ArcStr, literal};
use closure::closure;
use derive_more::Debug;
use dirs::home_dir;
use mitsein::vec1::Vec1;
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
#[cfg_attr(doc, doc(hidden))]
#[derive(Clone, Debug)]
pub enum Specification {
    /// A panel of buttons.
    ButtonPanel {
        /// The title of the popup.
        title: ArcStr,
        /// The buttons.
        buttons: Vec1<(ArcStr, Action)>,
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

impl Specification {
    /// Constructs a new [file-section popup](Specification::FileSelector) starting at the current dir.
    pub fn file_selector<A: Fn(&Path) -> Action + Send + Sync + 'static>(
        title: ArcStr,
        action: A,
    ) -> Specification {
        let path = current_dir().ok().or_else(home_dir).unwrap_or_default();

        Specification::FileSelector {
            title,
            path: Arc::from(path),
            action: Arc::new(action),
        }
    }

    /// Returns the title of the popup.
    #[must_use]
    pub fn title(&self) -> ArcStr {
        match self {
            Specification::ButtonPanel { title, .. }
            | Specification::FileSelector { title, .. } => title.clone(),
            Specification::Error { .. } => ERROR_TITLE,
            // TODO: Display at what instant the key is being set.
            Specification::KeySelector { .. } => KEY_SELECTOR_TITLE,
        }
    }

    /// Returns the popups [view](View) with a border and title.
    fn view(&self, id: Id<Popup>) -> View {
        self.inner_view(id)
            .bordered()
            .titled(self.title())
            .on_click(OnClick::from(Action::CloseContextMenu))
    }

    /// Returns the popups inner [view](View), with no border and title.
    fn inner_view(&self, id: Id<Popup>) -> View {
        match self {
            Specification::ButtonPanel { buttons, .. } => View::balanced_stack(
                Axis::Y,
                buttons.iter().map(|(label, action)| {
                    View::simple_button(label.clone(), OnClick::from(action.clone()))
                        .terminating(id)
                }),
            ),
            Specification::Error(error) => {
                let acknowledge_button = ACKNOWLEDGE.centred().bordered();

                View::minimal_stack(
                    Axis::Y,
                    [
                        arcstr::format!("{error:#}").aligned_to(Alignment::TopLeft),
                        acknowledge_button.terminating(id),
                    ],
                )
            }
            Specification::FileSelector { path, action, .. } => {
                let selected_file = Arc::new(ArcCell::new(Arc::clone(path)));

                let path = Arc::clone(&selected_file);
                let action = Arc::clone(action);

                let confirm = View::standard_button(
                    CONFIRM,
                    OnClick::new(move |_, actions| {
                        let path = path.get();
                        let action = action(&path);
                        actions.push(action);
                    }),
                )
                .terminating(id);
                let cancel = CANCEL.centred().bordered().terminating(id);

                let buttons = View::minimal_stack(Axis::X, vec![cancel, confirm]);

                View::y_stack([
                    file_selector(selected_file).fill_remaining(),
                    buttons.quotated_minimally(),
                ])
            }
            Specification::KeySelector { instant, key } => {
                let instant = *instant;

                let tonic = Arc::new(Cell::new(key.tonic));
                let sign = Arc::new(Cell::new(key.sign));
                let intervals = Arc::new(Cell::new(key.intervals));

                let buttons = View::minimal_stack(
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

                View::minimal_stack(
                    Axis::Y,
                    vec![
                        single::selector_with_formatter(
                            &tonic,
                            Axis::X,
                            closure!([clone sign] move |chroma| {
                                chroma.name(sign.get())
                            }),
                        ),
                        single::selector(&sign, Axis::X),
                        multi::selector(&intervals, Axis::X),
                        buttons,
                    ],
                )
            }
        }
        .on_click(OnClick::from(Action::CloseContextMenu))
    }

    pub(crate) fn instantiate<Ui: UserInterface>(&self, id: Id<Popup>, ui: &Ui) -> Popup {
        let view = Arc::new(self.view(id));

        let size = view.minimum_size::<Ui>();

        let position = Point {
            x: (ui.size().width - size.width) * Ratio::HALF,
            y: (ui.size().height - size.height) * Ratio::HALF,
        };

        let area = Rectangle { position, size };

        Popup::new(view, area)
    }
}

impl<E: Into<Error>> From<E> for Specification {
    fn from(error: E) -> Self {
        Specification::Error(Arc::new(error.into()))
    }
}

impl View {
    /// Makes the view close a popup when clicked.
    fn terminating(self, popup: Id<Popup>) -> View {
        self.on_click(OnClick::from(Action::ClosePopup(popup)))
    }
}
