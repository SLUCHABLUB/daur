use crate::app::Action;
use crate::holdable::WindowSide;
use crate::note::Key;
use crate::sync::{ArcCell, Cell};
use crate::ui::{Point, Rectangle, ThemeColour};
use crate::view::{
    Alignment, Axis, OnClick, RenderArea, ToText as _, file_selector, multi, single,
};
use crate::{Holdable, Id, Popup, Ratio, UserInterface, View, project};
use anyhow::Error;
use arcstr::{ArcStr, literal};
use closure::closure;
use derive_more::Debug;
use dirs::home_dir;
use std::env::current_dir;
use std::path::Path;
use std::sync::{Arc, LazyLock};

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
            Specification::FileSelector { title, .. } => title.clone(),
            Specification::Error { .. } => ERROR_TITLE,
            // TODO: Display at what instant the key is being set.
            Specification::KeySelector { .. } => KEY_SELECTOR_TITLE,
        }
    }

    pub(crate) fn generate_id(&self) -> Id<Popup> {
        static FILE_SELECTOR: LazyLock<Id<Popup>> = LazyLock::new(Id::generate);
        static KEY_SELECTOR: LazyLock<Id<Popup>> = LazyLock::new(Id::generate);

        match self {
            Specification::Error(_) => Id::generate(),
            Specification::FileSelector { .. } => *FILE_SELECTOR,
            Specification::KeySelector { .. } => *KEY_SELECTOR,
        }
    }

    /// Returns the popups [view](View) with a border and title.
    fn view<Ui: UserInterface>(&self, id: Id<Popup>) -> View {
        let grab_edge = move |render_area: RenderArea| {
            let mouse_position = render_area.relative_mouse_position()?;

            let left = mouse_position.x < Ui::BORDER_THICKNESS;
            let top = mouse_position.y < Ui::BORDER_THICKNESS;

            let right = render_area.area.size.width - Ui::BORDER_THICKNESS <= mouse_position.x;
            let bottom = render_area.area.size.height - Ui::BORDER_THICKNESS <= mouse_position.y;

            let side = if top && left {
                WindowSide::TopLeft
            } else if top && right {
                WindowSide::TopRight
            } else if top {
                return Some(Holdable::Popup {
                    id,
                    point: mouse_position,
                });
            } else if bottom && left {
                WindowSide::BottomLeft
            } else if bottom && right {
                WindowSide::BottomRight
            } else if bottom {
                WindowSide::Bottom
            } else if left {
                WindowSide::Left
            } else if right {
                WindowSide::Right
            } else {
                return None;
            };

            Some(Holdable::PopupSide { side, popup: id })
        };

        let foreground = self
            .inner_view(id)
            .bordered_with_title(self.title())
            .grabbable(grab_edge)
            .on_click(OnClick::from(Action::CloseContextMenu));

        View::Layers(vec![View::Solid(ThemeColour::Background), foreground])
    }

    /// Returns the popups inner [view](View), with no border and title.
    fn inner_view(&self, id: Id<Popup>) -> View {
        match self {
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
            Specification::KeySelector { key } => {
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
                                    Action::Edit(project::Edit::SetKey(Key {
                                            tonic: tonic.get(),
                                            sign: sign.get(),
                                            intervals: intervals.get(),
                                        }))
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
        let view = Arc::new(self.view::<Ui>(id));

        let size = view.minimum_size::<Ui>(ui.render_area());

        let position = Point {
            x: (ui.size().width - size.width) * Ratio::HALF,
            y: (ui.size().height - size.height) * Ratio::HALF,
        };

        let area = Rectangle { position, size };

        Popup::new(view, area)
    }
}

impl<E: Into<Error>> From<E> for Specification {
    fn from(error: E) -> Specification {
        Specification::Error(Arc::new(error.into()))
    }
}

impl View {
    /// Makes the view close a popup when clicked.
    fn terminating(self, popup: Id<Popup>) -> View {
        self.on_click(OnClick::from(Action::ClosePopup(popup)))
    }
}
