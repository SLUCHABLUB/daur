//! Types pertaining to context menus.

use crate::UserInterface;
use crate::app::Action;
use crate::popup::Specification;
use crate::project::Edit;
use crate::ui::{Point, Rectangle, ThemeColour};
use crate::view::{Axis, OnClick, View};
use arcstr::{ArcStr, literal};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

const IMPORT_AUDIO: ArcStr = literal!("import audio");
const ADD_NOTES: ArcStr = literal!("add notes");
const TOGGLE_PIANO_ROLL: ArcStr = literal!("toggle piano roll");

/// A context (right-click) menu specification.
#[derive(Clone)]
pub struct Menu {
    /// The buttons in the menu.
    pub buttons: Vec<(ArcStr, Action)>,
}

impl Menu {
    /// The context menu for the track overview.
    #[must_use]
    pub fn track_overview() -> Menu {
        Menu {
            buttons: vec![
                (
                    IMPORT_AUDIO,
                    Action::OpenPopup(Specification::AudioImporter),
                ),
                (ADD_NOTES, Action::Edit(Edit::AddNoteGroup)),
                (TOGGLE_PIANO_ROLL, Action::TogglePianoRoll),
            ],
        }
    }

    /// Returns the view of the menu.
    pub fn into_view(self) -> View {
        View::Layers(vec![
            View::Solid(ThemeColour::Background),
            View::balanced_stack(
                Axis::Y,
                self.buttons
                    .into_iter()
                    .map(|(label, action)| View::simple_button(label, OnClick::from(action))),
            )
            .bordered()
            .on_click(OnClick::from(Action::CloseContextMenu)),
        ])
    }

    /// Constructs a new [`MenuInstance`].
    #[must_use]
    pub fn instantiate<Ui: UserInterface>(self, position: Point, ui: &Ui) -> MenuInstance {
        let view = Arc::new(self.into_view());
        let size = view.minimum_size::<Ui>(ui.render_area());
        let area = Rectangle { position, size };
        MenuInstance { area, view }
    }
}

impl Debug for Menu {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();

        for (name, action) in &self.buttons {
            map.entry(name, action);
        }

        map.finish()
    }
}

/// An instance of a context menu.
#[derive(Clone, Debug)]
pub struct MenuInstance {
    /// The area of the context menu.
    pub area: Rectangle,
    /// The view of the context menu.
    pub view: Arc<View>,
}

impl MenuInstance {
    /// Converts the context menu into a [view](View).
    pub fn into_view(self) -> View {
        // We call `.relative_to(Point::ZERO)` since the context menu is positioned absolutely.
        View::Shared(self.view)
            .quotated_2d(self.area.size)
            .positioned(self.area.position.relative_to(Point::ZERO))
    }
}
