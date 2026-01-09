use crate::Holdable;
use crate::Id;
use crate::Popup;
use crate::Ratio;
use crate::UserInterface;
use crate::View;
use crate::app::Action;
use crate::holdable::WindowSide;
use crate::note::Key;
use crate::note::NonUnisonSimpleInterval;
use crate::note::PitchClass;
use crate::note::Sign;
use crate::project::Edit;
use crate::sync::Cell;
use crate::ui::Point;
use crate::ui::Rectangle;
use crate::ui::ThemeColour;
use crate::view::Alignment;
use crate::view::Axis;
use crate::view::CANCEL;
use crate::view::CONFIRM;
use crate::view::OnClick;
use crate::view::RenderArea;
use crate::view::ToText as _;
use crate::view::file;
use crate::view::multi;
use crate::view::single;
use arcstr::ArcStr;
use arcstr::literal;
use derive_more::Debug;
use enumset::EnumSet;
use serde::Deserialize;
use std::sync::Arc;
use std::sync::LazyLock;

const ACKNOWLEDGE: ArcStr = literal!("ok");

// TODO: keyboard navigation of popups
/// A specification for a popup window.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
#[remain::sorted]
#[serde(rename_all = "snake_case")]
pub enum Specification {
    /// A file selector for importing an audio file.
    AudioImporter,
    /// An error message.
    #[serde(skip)]
    Error(Arc<anyhow::Error>),
    /// A window for selecting a key.
    #[serde(skip)]
    KeySelector {
        /// The current key.
        key: Key,
    },
    /// A file selector for opening a project.
    ProjectOpener,
    /// A file selector for selecting the save location.
    SaveLocationPicker,
}

impl Specification {
    /// Returns the title of the popup.
    #[must_use]
    pub const fn title(&self) -> ArcStr {
        const AUDIO_IMPORTER_TITLE: ArcStr = literal!("import audio");
        const ERROR_TITLE: ArcStr = literal!("error");
        const KEY_SELECTOR_TITLE: ArcStr = literal!("select key");
        const SAVE_LOCATION_PICKER_TITLE: ArcStr = literal!("save project as");
        const PROJECT_OPENER_TITLE: ArcStr = literal!("open project");

        match self {
            Specification::AudioImporter => AUDIO_IMPORTER_TITLE,
            Specification::Error { .. } => ERROR_TITLE,
            Specification::KeySelector { .. } => KEY_SELECTOR_TITLE,
            Specification::SaveLocationPicker => SAVE_LOCATION_PICKER_TITLE,
            Specification::ProjectOpener => PROJECT_OPENER_TITLE,
        }
    }

    pub(crate) fn generate_id(&self) -> Id<Popup> {
        static AUDIO_FILE_IMPORTER: LazyLock<Id<Popup>> = LazyLock::new(Id::generate);
        static KEY_SELECTOR: LazyLock<Id<Popup>> = LazyLock::new(Id::generate);
        static SAVE_LOCATION_PICKER: LazyLock<Id<Popup>> = LazyLock::new(Id::generate);
        static PROJECT_OPENER: LazyLock<Id<Popup>> = LazyLock::new(Id::generate);

        match self {
            Specification::AudioImporter => *AUDIO_FILE_IMPORTER,
            Specification::Error(_) => Id::generate(),
            Specification::KeySelector { .. } => *KEY_SELECTOR,
            Specification::SaveLocationPicker => *SAVE_LOCATION_PICKER,
            Specification::ProjectOpener => *PROJECT_OPENER,
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
                file::picker_in_popup(|file| Action::Edit(Edit::ImportAudio { file }), id)
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
                fn confirm_action(
                    tonic: Arc<Cell<PitchClass>>,
                    sign: Arc<Cell<Sign>>,
                    intervals: Arc<Cell<EnumSet<NonUnisonSimpleInterval>>>,
                ) -> impl Fn() -> Action {
                    move || {
                        Action::Edit(Edit::SetKey(Key {
                            tonic: tonic.get(),
                            sign: sign.get(),
                            intervals: intervals.get(),
                        }))
                    }
                }

                fn pitch_class_formatter(
                    sign: Arc<Cell<Sign>>,
                ) -> impl Fn(&PitchClass) -> ArcStr + Clone {
                    move |class| class.name(sign.get())
                }

                let tonic = Arc::new(Cell::new(key.tonic));
                let sign = Arc::new(Cell::new(key.sign));
                let intervals = Arc::new(Cell::new(key.intervals));

                let sign_selector = single::selector(&sign, Axis::X);
                let interval_selector = multi::selector(&intervals, Axis::X);

                let tonic_selector = single::selector_with_formatter(
                    &tonic,
                    Axis::X,
                    pitch_class_formatter(Arc::clone(&sign)),
                );

                let buttons = View::minimal_stack(
                    Axis::X,
                    vec![
                        CANCEL.centred().bordered().terminating(id),
                        View::standard_button(
                            CONFIRM,
                            OnClick::action(confirm_action(tonic, sign, intervals)),
                        )
                        .terminating(id),
                    ],
                );

                View::minimal_stack(
                    Axis::Y,
                    vec![tonic_selector, sign_selector, interval_selector, buttons],
                )
            }
            Specification::SaveLocationPicker => file::picker_in_popup(Action::SaveAs, id),
            Specification::ProjectOpener => file::picker_in_popup(Action::OpenProject, id),
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

impl<E: Into<anyhow::Error>> From<E> for Specification {
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
