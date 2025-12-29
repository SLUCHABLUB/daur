use crate::app::Action;
use crate::holdable::WindowSide;
use crate::note::Key;
use crate::project::Edit;
use crate::sync::{ArcCell, Cell};
use crate::ui::{Point, Rectangle, ThemeColour};
use crate::view::{
    Alignment, Axis, OnClick, RenderArea, ToText as _, file_selector, multi, single,
};
use crate::{Holdable, Id, Popup, Ratio, UserInterface, View};
use anyhow::Error;
use arcstr::{ArcStr, literal};
use closure::closure;
use derive_more::Debug;
use serde::Deserialize;
use std::env::current_dir;
use std::sync::{Arc, LazyLock};

const ACKNOWLEDGE: ArcStr = literal!("ok");
const CANCEL: ArcStr = literal!("cancel");
const CONFIRM: ArcStr = literal!("confirm");

const AUDIO_IMPORTER_TITLE: ArcStr = literal!("import audio");
const ERROR_TITLE: ArcStr = literal!("error");
const KEY_SELECTOR_TITLE: ArcStr = literal!("select key");

// TODO: keyboard navigation of popups
/// A specification for a popup window.
#[cfg_attr(doc, doc(hidden))]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Specification {
    /// A file selector for importing an audio file.
    AudioImporter,
    /// An error message.
    #[serde(skip)]
    Error(Arc<Error>),
    /// A window for selecting a key.
    #[serde(skip)]
    KeySelector {
        /// The current key.
        key: Key,
    },
}

impl Specification {
    /// Returns the title of the popup.
    #[must_use]
    pub const fn title(&self) -> ArcStr {
        match self {
            Specification::AudioImporter => AUDIO_IMPORTER_TITLE,
            Specification::Error { .. } => ERROR_TITLE,
            Specification::KeySelector { .. } => KEY_SELECTOR_TITLE,
        }
    }

    pub(crate) fn generate_id(&self) -> Id<Popup> {
        static AUDIO_FILE_IMPORTER: LazyLock<Id<Popup>> = LazyLock::new(Id::generate);
        static KEY_SELECTOR: LazyLock<Id<Popup>> = LazyLock::new(Id::generate);

        match self {
            Specification::AudioImporter => *AUDIO_FILE_IMPORTER,
            Specification::Error(_) => Id::generate(),
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
            Specification::AudioImporter => {
                // TODO: use the project dir rather than the current dir
                let selected_file =
                    Arc::new(ArcCell::new(Arc::from(current_dir().unwrap_or_default())));

                let file = Arc::clone(&selected_file);

                let confirm = View::standard_button(
                    CONFIRM,
                    OnClick::action(move || Action::Edit(Edit::ImportAudio { file: file.get() })),
                )
                .terminating(id);
                let cancel = CANCEL.centred().bordered().terminating(id);

                let buttons = View::minimal_stack(Axis::X, vec![cancel, confirm]);

                View::y_stack([
                    file_selector(selected_file).fill_remaining(),
                    buttons.quotated_minimally(),
                ])
            }
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
                                    Action::Edit(Edit::SetKey(Key {
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
        let ui_size = ui.size();

        let view = Arc::new(self.view::<Ui>(id));

        let size = view.minimum_size::<Ui>(ui.render_area());

        let position = Point {
            x: (ui_size.width - size.width) * Ratio::HALF,
            y: (ui_size.height - size.height) * Ratio::HALF,
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
