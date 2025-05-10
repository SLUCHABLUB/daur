//! Types pertaining to [`Visitor`].

mod clicker;
mod grabber;
mod scroller;

pub use clicker::Clicker;
pub use grabber::Grabber;
pub use scroller::Scroller;

use crate::app::HoldableObject;
use crate::ui::{Colour, Length, Point, Rectangle, Vector};
use crate::view::context::Menu;
use crate::view::{Alignment, OnClick, Painter};
use crate::{Action, Ratio, UserInterface, View};
use core::iter::zip;
use core::num::NonZeroU64;

/// A type that can visit a [view](View).
pub trait Visitor {
    /// Whether views should be visited from inside out.
    /// This also reverses the order in which layers are visited.
    #[must_use]
    fn reverse_order() -> bool {
        false
    }

    /// Visits a bordered view.
    fn visit_border(&mut self, area: Rectangle, thick: bool);

    /// Visits a canvas.
    fn visit_canvas(&mut self, area: Rectangle, background: Colour, painter: &Painter);

    /// Visits a clickable view.
    fn visit_clickable(&mut self, area: Rectangle, on_click: &OnClick);

    /// Visits a view with a context menu.
    fn visit_contextual(&mut self, area: Rectangle, menu: &Menu);

    /// Visits a cursor window.
    fn visit_cursor_window(&mut self, area: Rectangle, offset: Length);

    /// Visits a grabbable view.
    fn visit_grabbable(&mut self, area: Rectangle, object: HoldableObject);

    /// Visits a rule.
    fn visit_rule(&mut self, area: Rectangle, index: isize, cells: NonZeroU64);

    /// Visits a scrollable view.
    fn visit_scrollable(&mut self, area: Rectangle, action: fn(Vector) -> Action);

    /// Visits a solid colour.
    fn visit_solid(&mut self, area: Rectangle, colour: Colour);

    /// Visits a text view.
    fn visit_text(&mut self, area: Rectangle, string: &str, alignment: Alignment);

    /// Visits a titled view.
    fn visit_titled(&mut self, area: Rectangle, title: &str, highlighted: bool);

    /// Visits a window.
    fn visit_window(&mut self, area: Rectangle);

    // --- compound methods ---

    /// Visits a titled bordered view.
    fn visit_titled_bordered(
        &mut self,
        area: Rectangle,
        titled_area: Rectangle,
        title: &str,
        highlighted: bool,
        thick: bool,
    ) {
        self.visit_titled(area, title, highlighted);
        self.visit_border(titled_area, thick);
    }
}

macro_rules! compound {
    (
        $container:expr,
        $content:expr,
    ) => {{
        if !V::reverse_order() {
            $container;
        }

        $content;

        if V::reverse_order() {
            $container;
        }
    }};
}

impl View {
    /// Accepts a view visitor.
    #[expect(clippy::too_many_lines, reason = "`View` is a large")]
    pub fn accept<Ui: UserInterface, V: Visitor + ?Sized>(
        &self,
        visitor: &mut V,
        area: Rectangle,
        mouse_position: Point,
    ) {
        match self {
            View::Bordered { thick, view } => compound!(
                visitor.visit_border(area, *thick),
                view.accept::<Ui, V>(visitor, inner_area::<Ui>(area), mouse_position),
            ),
            View::Canvas {
                background,
                painter,
            } => visitor.visit_canvas(area, *background, painter),
            View::Clickable { on_click, view } => compound!(
                visitor.visit_clickable(area, on_click),
                view.accept::<Ui, V>(visitor, area, mouse_position),
            ),
            View::Contextual { menu, view } => compound!(
                visitor.visit_contextual(area, menu),
                view.accept::<Ui, V>(visitor, area, mouse_position),
            ),
            View::CursorWindow(window) => {
                if let Some(offset) = window.offset() {
                    visitor.visit_cursor_window(area, offset);
                }
            }
            View::Empty => (),
            View::Generator(generator) => {
                generator().accept::<Ui, V>(visitor, area, mouse_position);
            }
            View::Grabbable { object, view } => compound!(
                if let Some(object) = object(area, mouse_position) {
                    visitor.visit_grabbable(area, object);
                },
                view.accept::<Ui, V>(visitor, area, mouse_position),
            ),
            View::Hoverable {
                is_hovered,
                default,
                hovered,
            } => {
                if area.contains(mouse_position) {
                    is_hovered.set(true);
                    hovered.accept::<Ui, V>(visitor, area, mouse_position);
                } else {
                    is_hovered.set(false);
                    default.accept::<Ui, V>(visitor, area, mouse_position);
                }
            }
            View::Layers(layers) => {
                let visit = |layer: &View| layer.accept::<Ui, V>(visitor, area, mouse_position);

                if V::reverse_order() {
                    layers.iter().rev().for_each(visit);
                } else {
                    layers.iter().for_each(visit);
                }
            }
            View::Rule { index, cells } => visitor.visit_rule(area, *index, *cells),
            View::Scrollable { action, view } => compound!(
                visitor.visit_scrollable(area, *action),
                view.accept::<Ui, V>(visitor, area, mouse_position),
            ),
            View::SizeInformed(generator) => {
                generator(area.size).accept::<Ui, V>(visitor, area, mouse_position);
            }
            View::Solid(colour) => visitor.visit_solid(area, *colour),
            View::Stack { axis, elements } => {
                let rectangles = area.split::<Ui>(*axis, elements);

                for (area, quoted) in zip(rectangles, elements) {
                    quoted.view.accept::<Ui, V>(visitor, area, mouse_position);
                }
            }
            View::Text { string, alignment } => visitor.visit_text(area, string, *alignment),
            View::Titled {
                title,
                highlighted,
                view,
                ..
            } => {
                let titled_area = titled_area::<Ui>(area, title, view);

                if let View::Bordered { thick, view } = &**view {
                    let inner_area = inner_area::<Ui>(titled_area);
                    compound!(
                        visitor.visit_titled_bordered(
                            area,
                            titled_area,
                            title,
                            *highlighted,
                            *thick
                        ),
                        view.accept::<Ui, V>(visitor, inner_area, mouse_position),
                    );
                    return;
                }

                compound!(
                    visitor.visit_titled(area, title, *highlighted),
                    view.accept::<Ui, V>(visitor, titled_area, mouse_position),
                );
            }
            View::Window {
                area: relative,
                view,
            } => {
                if let Some(area) = window_area(area, *relative) {
                    compound!(
                        visitor.visit_window(area),
                        view.accept::<Ui, V>(visitor, area, mouse_position),
                    );
                }
            }
        }
    }
}

fn window_area(full: Rectangle, relative: Rectangle) -> Option<Rectangle> {
    let max_position = full.bottom_right() - relative.size.diagonal();
    let preferred_position = relative.position + full.position.position();

    // Whether the window needs to be moved due to it otherwise not fitting in the area
    let need_move = preferred_position.x > max_position.x || preferred_position.y > max_position.y;

    let position = if need_move {
        max_position
    } else {
        preferred_position
    };
    let size = relative.size;

    full.intersection(Rectangle { position, size })
}

fn titled_area<Ui: UserInterface>(mut area: Rectangle, title: &str, view: &View) -> Rectangle {
    let title_height = Ui::title_height(title, view);
    area.position.y += title_height;
    area.size.height -= title_height;
    area
}

fn inner_area<Ui: UserInterface>(mut area: Rectangle) -> Rectangle {
    area.position.x += Ui::BORDER_THICKNESS;
    area.position.y += Ui::BORDER_THICKNESS;
    area.size.width -= Ui::BORDER_THICKNESS * Ratio::integer(2);
    area.size.height -= Ui::BORDER_THICKNESS * Ratio::integer(2);
    area
}
