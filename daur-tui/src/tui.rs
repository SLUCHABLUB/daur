use crate::controls::controls;
use crate::convert::to_length;
use crossterm::event::{KeyCode, KeyModifiers, MouseButton};
use daur::app::Action;
use daur::sync::Cell;
use daur::ui::{Length, NonZeroLength, Point, Rectangle, Size};
use daur::{Ratio, UserInterface, View};
use non_zero::non_zero;
use saturating_cast::SaturatingCast as _;
use std::collections::HashMap;
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation as _;

pub(crate) struct Tui {
    pub should_exit: Cell<bool>,
    // TODO: move to app
    pub key_actions: HashMap<(KeyModifiers, KeyCode), Action>,
    pub mouse_movement_since_mouse_down: Cell<bool>,
    // Some terminals do not send the mouse button on release.
    pub last_mouse_button_down: Cell<MouseButton>,
    // TODO: update
    pub should_redraw: bool,
    pub mouse_position: Cell<Point>,
    pub area: Cell<Rectangle>,
}

impl Tui {
    pub(crate) fn key_action(&self, modifiers: KeyModifiers, code: KeyCode) -> Option<Action> {
        self.key_actions.get(&(modifiers, code)).cloned()
    }
}

impl UserInterface for Tui {
    const BLACK_KEY_DEPTH: NonZeroLength = NonZeroLength {
        pixels: non_zero!(6),
    };
    const BORDER_THICKNESS: Length = Length { pixels: 1 };
    const CELL_WIDTH: NonZeroLength = NonZeroLength {
        pixels: non_zero!(4),
    };
    const KEY_WIDTH: NonZeroLength = NonZeroLength {
        pixels: non_zero!(1),
    };
    const PIANO_DEPTH: NonZeroLength = NonZeroLength {
        pixels: non_zero!(10),
    };
    const PLAYBACK_BUTTON_WIDTH: NonZeroLength = NonZeroLength {
        pixels: non_zero!(7),
    };
    const PROJECT_BAR_HEIGHT: NonZeroLength = NonZeroLength {
        pixels: non_zero!(5),
    };
    const RULER_HEIGHT: NonZeroLength = NonZeroLength {
        pixels: non_zero!(2),
    };
    const TRACK_SETTINGS_WITH: NonZeroLength = NonZeroLength {
        pixels: non_zero!(20),
    };

    fn exit(&self) {
        self.should_exit.set(true);
    }

    fn size(&self) -> Size {
        self.area.get().size
    }

    fn mouse_position(&self) -> Point {
        self.mouse_position.get()
    }

    fn string_width(string: &str) -> Length {
        let graphemes = string
            .lines()
            .map(|line| line.graphemes(true).count())
            .max()
            .unwrap_or(0);

        Length {
            pixels: graphemes.saturating_cast(),
        }
    }

    fn string_height(string: &str) -> Length {
        let pixels = string.lines().count().saturating_cast();

        Length { pixels }
    }

    fn title_width(title: &str, titled: &View) -> Length {
        Self::string_width(title)
            + if matches!(titled, View::Bordered { .. }) {
                Length::PIXEL * Ratio::integer(2)
            } else {
                Length::ZERO
            }
    }

    fn title_height(_title: &str, titled: &View) -> Length {
        if matches!(titled, View::Bordered { .. }) {
            Length::ZERO
        } else {
            Length::PIXEL
        }
    }

    fn file_selector_size(path: &Path) -> Size {
        let Ok(reader) = path.read_dir() else {
            return Size::ZERO;
        };

        // + 1 for ".."
        let height = Length::PIXEL * Ratio::from_usize(reader.count()) + Length::PIXEL;
        // This is not very important, the user can resize the popup.
        let width = Length::PIXEL * Ratio::from_usize(path.as_os_str().len());

        Size { height, width }
    }
}

const DEFAULT_TERMINAL_SIZE: Size = Size {
    width: to_length(80),
    height: to_length(24),
};

impl Default for Tui {
    fn default() -> Tui {
        Tui {
            should_exit: Cell::new(false),
            key_actions: controls(),
            mouse_movement_since_mouse_down: Cell::new(false),
            last_mouse_button_down: Cell::new(MouseButton::Left),
            should_redraw: true,
            mouse_position: Cell::new(Point::ZERO),
            area: Cell::new(Rectangle {
                position: Point::ZERO,
                size: DEFAULT_TERMINAL_SIZE,
            }),
        }
    }
}
