use crate::convert::{point_to_position, rect_to_rectangle, size_to_ratatui};
use daur::arcstr::ArcStr;
use daur::popup::Id;
use daur::{Cell, Lock, Observed, OptionArcCell, Ratio, UserInterface, View};
use ratatui::layout::{Position, Rect};

pub struct Tui {
    pub popups: Lock<Vec<(Id, Rect, View)>>,
    pub context_menu: OptionArcCell<(Rect, View)>,

    pub should_exit: Observed<bool>,
    pub should_redraw: Observed<bool>,
    pub mouse_position: Cell<Position>,
    pub window_area: Cell<Rect>,
}

impl UserInterface for Tui {
    fn exit(&self) {
        self.should_exit.set(true);
    }

    type PopupHandle = Id;

    fn open_popup(&self, title: ArcStr, view: View, id: Id) -> Self::PopupHandle {
        let view = view.titled(title);

        let window_area = rect_to_rectangle(self.window_area.get());

        let size = view.minimum_size();
        let position = point_to_position(
            (window_area.bottom_right().position() * Ratio::HALF - size.diagonal() * Ratio::HALF)
                .point(),
        );

        let area = Rect::from((position, size_to_ratatui(size)));

        self.popups.write().push((id, area, view));

        id
    }

    fn close_popup(&self, handle: Self::PopupHandle) {
        let mut popups = self.popups.write();

        if let Some(index) = popups.iter().position(|(id, _, _)| *id == handle) {
            let popup = popups.remove(index);
            drop(popup);
        }
    }
}

impl Default for Tui {
    fn default() -> Self {
        Tui {
            popups: Lock::new(Vec::new()),
            context_menu: OptionArcCell::none(),

            should_exit: Observed::new(false),
            should_redraw: Observed::new(true),
            mouse_position: Cell::new(Position::ORIGIN),
            window_area: Cell::new(Rect::ZERO),
        }
    }
}
