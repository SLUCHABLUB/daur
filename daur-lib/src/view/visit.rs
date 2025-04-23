use crate::ui::{Length, Point, Rectangle};
use crate::view::context::Menu;
use crate::view::{Alignment, OnClick, Painter};
use crate::{ArcCell, Colour, Ratio, UserInterface, View};
use std::iter::zip;
use std::num::NonZeroU64;
use std::path::Path;

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

    /// Visits a clickable view.
    fn visit_button(&mut self, area: Rectangle, on_click: &OnClick);

    /// Visits a canvas.
    fn visit_canvas(&mut self, area: Rectangle, background: Colour, painter: &Painter);

    /// Visits a view with a context menu.
    fn visit_contextual(&mut self, area: Rectangle, menu: &Menu);

    /// Visits a cursor window.
    fn visit_cursor_window(&mut self, area: Rectangle, offset: Length);

    /// Visits a file selector.
    fn visit_file_selector(&mut self, area: Rectangle, selected_file: &ArcCell<Path>);

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
            View::Bordered { thick, content } => compound!(
                visitor.visit_border(area, *thick),
                content.accept(visitor, inner_area::<V::Ui>(area), mouse_position),
            ),
            View::Button { on_click, content } => compound!(
                visitor.visit_button(area, on_click),
                content.accept(visitor, area, mouse_position),
            ),
            View::Canvas {
                background,
                painter,
            } => visitor.visit_canvas(area, *background, painter),
            View::Contextual { menu, view } => compound!(
                visitor.visit_contextual(area, menu),
                view.accept(visitor, area, mouse_position),
            ),
            View::CursorWindow { offset } => visitor.visit_cursor_window(area, *offset),
            View::Empty => (),
            View::FileSelector { selected_file } => {
                visitor.visit_file_selector(area, selected_file);
            }
            View::Generator(generator) => generator().accept(visitor, area, mouse_position),
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
                view,
                minimum_size: _,
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

                if let View::Bordered { thick, content } = &**view {
                    let inner_area = inner_area::<V::Ui>(titled_area);
                    compound!(
                        visitor.visit_titled_bordered(
                            area,
                            titled_area,
                            title,
                            *highlighted,
                            *thick
                        ),
                        content.accept(visitor, inner_area, mouse_position),
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
