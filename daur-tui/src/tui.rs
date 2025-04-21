use crate::convert::{point_to_position, rect_to_rectangle, size_to_ratatui};
use crate::popup_handle::PopupHandle;
use daur::arcstr::ArcStr;
use daur::popup::Id;
use daur::ui::{Length, NonZeroLength};
use daur::{Cell, Lock, Observed, OptionArcCell, Ratio, UserInterface, View};
use ratatui::layout::{Position, Rect};
use saturating_cast::SaturatingCast as _;
use std::sync::Arc;
use unicode_segmentation::UnicodeSegmentation as _;

macro_rules! non_zero_length {
    ($pixels:literal) => {
        NonZeroLength {
            pixels: std::num::NonZeroU16::MIN.saturating_add($pixels - 1),
        }
    };
}

pub struct Tui {
    pub popups: Arc<Lock<Vec<(Id, Rect, View)>>>,
    pub context_menu: OptionArcCell<(Rect, View)>,

    pub should_exit: Observed<bool>,
    pub should_redraw: Observed<bool>,
    pub mouse_position: Cell<Position>,
    pub window_area: Cell<Rect>,
}

impl UserInterface for Tui {
    const BLACK_KEY_DEPTH: NonZeroLength = non_zero_length!(6);
    const DOUBLE_BORDER_THICKNESS: Length = Length { pixels: 2 };
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

    fn string_height(string: &str) -> Length {
        let pixels = string.lines().count().saturating_cast();

        Length { pixels }
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

    fn title_height(_title: &str, titled: &View) -> Length {
        if matches!(titled, View::Bordered { .. }) {
            Length::ZERO
        } else {
            Length::PIXEL
        }
    }

    type PopupHandle = PopupHandle;

    fn open_popup(&self, title: ArcStr, view: View, id: Id) -> Self::PopupHandle {
        let view = view.bordered().titled(title);

        let window_area = rect_to_rectangle(self.window_area.get());

        let size = view.minimum_size::<Tui>();
        let position = point_to_position(
            (window_area.bottom_right().position() * Ratio::HALF - size.diagonal() * Ratio::HALF)
                .point(),
        );

        let area = Rect::from((position, size_to_ratatui(size)));

        self.popups.write().push((id, area, view));

        PopupHandle::new(Arc::clone(&self.popups), id)
    }
}

impl Default for Tui {
    fn default() -> Self {
        Tui {
            popups: Arc::new(Lock::new(Vec::new())),
            context_menu: OptionArcCell::none(),

            should_exit: Observed::new(false),
            should_redraw: Observed::new(true),
            mouse_position: Cell::new(Position::ORIGIN),
            window_area: Cell::new(Rect::ZERO),
        }
    }
}
