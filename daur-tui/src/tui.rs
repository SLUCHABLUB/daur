use crate::convert::to_length;
use daur::ui::{Length, NonZeroLength, Point, Rectangle, Size};
use daur::{Cell, OptionArcCell, Ratio, UserInterface, View};
use saturating_cast::SaturatingCast as _;
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation as _;

macro_rules! non_zero_length {
    ($pixels:literal) => {
        NonZeroLength {
            pixels: std::num::NonZeroU16::MIN.saturating_add($pixels - 1),
        }
    };
}

pub struct Tui {
    pub should_exit: Cell<bool>,
    pub cached_view: OptionArcCell<View>,
    pub should_redraw: Cell<bool>,
    pub mouse_position: Cell<Point>,
    pub window_area: Cell<Rectangle>,
}

impl UserInterface for Tui {
    const BLACK_KEY_DEPTH: NonZeroLength = non_zero_length!(6);
    const BORDER_THICKNESS: Length = Length::PIXEL;
    const CELL_WIDTH: NonZeroLength = non_zero_length!(4);
    const KEY_WIDTH: NonZeroLength = non_zero_length!(1);
    const PIANO_DEPTH: NonZeroLength = non_zero_length!(10);
    const PLAYBACK_BUTTON_WIDTH: NonZeroLength = non_zero_length!(7);
    const PROJECT_BAR_HEIGHT: NonZeroLength = non_zero_length!(5);
    const RULER_HEIGHT: NonZeroLength = non_zero_length!(2);
    const TRACK_SETTINGS_WITH: NonZeroLength = non_zero_length!(20);

    fn exit(&self) {
        self.should_exit.set(true);
    }

    fn rerender(&self) {
        self.cached_view.set_none();
        self.should_redraw.set(true);
    }

    fn size(&self) -> Size {
        self.window_area.get().size
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
    fn default() -> Self {
        Tui {
            should_exit: Cell::new(false),
            cached_view: OptionArcCell::none(),
            should_redraw: Cell::new(true),
            mouse_position: Cell::new(Point::ZERO),
            window_area: Cell::new(Rectangle {
                position: Point::ZERO,
                size: DEFAULT_TERMINAL_SIZE,
            }),
        }
    }
}
