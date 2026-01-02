use crate::configuration::Configuration;
use crossterm::event::MouseButton;
use daur::UserInterface;
use daur::sync::Cell;
use daur::ui::Length;
use daur::ui::NonZeroLength;
use daur::ui::Point;
use daur::ui::Rectangle;
use daur::ui::Size;
use directories::ProjectDirs;
use non_zero::non_zero;
use saturating_cast::SaturatingCast as _;
use unicode_width::UnicodeWidthStr;

pub(crate) struct Tui {
    pub configuration: Configuration,

    pub should_exit: Cell<bool>,
    // TODO: update
    pub should_redraw: bool,

    pub mouse_position: Cell<Point>,
    pub area: Cell<Rectangle>,

    pub mouse_movement_since_mouse_down: Cell<bool>,
    // Some terminals do not send the mouse button on release.
    pub last_mouse_button_down: Cell<MouseButton>,
}

impl Tui {
    pub(crate) fn new(directories: &ProjectDirs) -> anyhow::Result<Tui> {
        Ok(Tui {
            configuration: Configuration::read_from_file(directories)?,

            should_exit: Cell::new(false),
            should_redraw: true,

            mouse_position: Cell::new(Point::ZERO),
            area: Cell::new(Rectangle::default()),

            mouse_movement_since_mouse_down: Cell::new(false),
            last_mouse_button_down: Cell::new(MouseButton::Left),
        })
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
    const TITLE_PADDING: Length = Length::ZERO;
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
        let width = string
            .lines()
            .map(UnicodeWidthStr::width)
            .max()
            .unwrap_or(0);

        Length {
            pixels: width.saturating_cast(),
        }
    }

    fn string_height(string: &str) -> Length {
        let pixels = string.lines().count().saturating_cast();

        Length { pixels }
    }
}
