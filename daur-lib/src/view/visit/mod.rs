//! Types pertaining to [`Visitor`].

mod clicker;
mod grabber;

pub use clicker::Clicker;
pub use grabber::Grabber;

use crate::app::HoldableObject;
use crate::ui::{Length, Point, Rectangle};
use crate::view::context::Menu;
use crate::view::{Alignment, OnClick, Painter};
use crate::{Colour, Ratio, UserInterface, View};
use std::iter::zip;
use std::num::NonZeroU64;

/// A type that can visit a [view](View).
pub trait Visitor {
    /// The user interface in which the views should be visited.
    type Ui: UserInterface;

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
    pub fn accept<V: Visitor + ?Sized>(
        &self,
        visitor: &mut V,
        area: Rectangle,
        mouse_position: Point,
    ) {
        match self {
            View::Bordered { thick, view } => compound!(
                visitor.visit_border(area, *thick),
                view.accept(visitor, inner_area::<V::Ui>(area), mouse_position),
            ),
            View::Canvas {
                background,
                painter,
            } => visitor.visit_canvas(area, *background, painter),
            View::Clickable { on_click, view } => compound!(
                visitor.visit_clickable(area, on_click),
                view.accept(visitor, area, mouse_position),
            ),
            View::Contextual { menu, view } => compound!(
                visitor.visit_contextual(area, menu),
                view.accept(visitor, area, mouse_position),
            ),
            View::CursorWindow { offset } => visitor.visit_cursor_window(area, *offset),
            View::Empty => (),
            View::Generator(generator) => generator().accept(visitor, area, mouse_position),
            View::Grabbable { object, view } => compound!(
                if let Some(object) = object(area, mouse_position) {
                    visitor.visit_grabbable(area, object);
                },
                view.accept(visitor, area, mouse_position),
            ),
            View::Hoverable { default, hovered } => {
                if area.contains(mouse_position) {
                    hovered.accept(visitor, area, mouse_position);
                } else {
                    default.accept(visitor, area, mouse_position);
                }
            }
            View::Layers(layers) => {
                let visit = |layer: &View| layer.accept(visitor, area, mouse_position);

                if V::reverse_order() {
                    layers.iter().rev().for_each(visit);
                } else {
                    layers.iter().for_each(visit);
                }
            }
            View::Rule { index, cells } => visitor.visit_rule(area, *index, *cells),
            View::Sized {
                minimum_size: _,
                view,
            } => view.accept(visitor, area, mouse_position),
            View::SizeInformed(generator) => {
                generator(area.size).accept(visitor, area, mouse_position);
            }
            View::Solid(colour) => visitor.visit_solid(area, *colour),
            View::Stack {
                direction,
                elements,
            } => {
                let quota: Vec<_> = elements.iter().map(|quotated| quotated.quotum).collect();
                let rectangles = area.split(*direction, &quota);

                for (area, quoted) in zip(rectangles, elements) {
                    quoted.view.accept(visitor, area, mouse_position);
                }
            }
            View::Text { string, alignment } => visitor.visit_text(area, string, *alignment),
            View::Titled {
                title,
                highlighted,
                view,
            } => {
                let titled_area = titled_area::<V::Ui>(area, title, view);

                if let View::Bordered { thick, view } = &**view {
                    let inner_area = inner_area::<V::Ui>(titled_area);
                    compound!(
                        visitor.visit_titled_bordered(
                            area,
                            titled_area,
                            title,
                            *highlighted,
                            *thick
                        ),
                        view.accept(visitor, inner_area, mouse_position),
                    );
                    return;
                }

                compound!(
                    visitor.visit_titled(area, title, *highlighted),
                    view.accept(visitor, titled_area, mouse_position),
                );
            }
            View::Window {
                area: relative,
                view,
            } => {
                if let Some(area) = window_area(area, *relative) {
                    compound!(
                        visitor.visit_window(area),
                        view.accept(visitor, area, mouse_position),
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
