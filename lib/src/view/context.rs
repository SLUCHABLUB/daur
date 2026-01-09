//! Types pertaining to context menus.

use crate::UserInterface;
use crate::View;
use crate::app::Action;
use crate::popup::Specification;
use crate::project::Edit;
use crate::ui::Point;
use crate::ui::Rectangle;
use crate::ui::ThemeColour;
use crate::view::Axis;
use crate::view::OnClick;
use arcstr::ArcStr;
use arcstr::literal;
use mitsein::btree_map1::BTreeMap1;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::Arc;

const ADD_NOTES: ArcStr = literal!("add notes");
const IMPORT_AUDIO: ArcStr = literal!("import audio");
const TOGGLE_PIANO_ROLL: ArcStr = literal!("toggle piano roll");

/// A context (right-click) menu specification.
#[derive(Clone)]
pub struct Menu {
    /// The buttons in the menu.
    pub buttons: BTreeMap1<ArcStr, Action>,
}

impl Menu {
    /// The context menu for the track overview.
    #[must_use]
    pub fn track_overview() -> Menu {
        Menu {
            buttons: BTreeMap1::from([
                (ADD_NOTES, Action::Edit(Edit::AddNoteGroup)),
                (
                    IMPORT_AUDIO,
                    Action::OpenPopup(Specification::AudioImporter),
                ),
                (TOGGLE_PIANO_ROLL, Action::TogglePianoRoll),
            ]),
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
            .quoted_2d(self.area.size)
            .positioned(self.area.position.relative_to(Point::ZERO))
    }
}
