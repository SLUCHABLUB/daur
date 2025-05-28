//! Types pertaining to [`Visitor`].

mod clicker;
mod dropper;
mod grabber;
mod scroller;

pub use clicker::Clicker;
pub use dropper::Dropper;
pub use grabber::Grabber;
pub use scroller::Scroller;

use crate::app::Action;
use crate::ui::{Colour, Length, Rectangle, Size, ThemeColour, Vector};
use crate::view::context::Menu;
use crate::view::{Alignment, DropAction, OnClick, Painter, RenderArea, SelectableItem};
use crate::{HoldableObject, Ratio, UserInterface, View};
use std::iter::zip;
use std::num::NonZeroU64;

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

    /// Visits a view that accepts objects.
    fn visit_object_acceptor(&mut self, area: Rectangle, action: &DropAction);

    /// Visits a rule.
    fn visit_rule(&mut self, area: Rectangle, index: isize, cells: NonZeroU64);

    /// Visits a selectable view.
    fn visit_selectable(&mut self, area: Rectangle, item: SelectableItem);

    /// Visits a selection box.
    fn visit_selection_box(&mut self, area: Rectangle);

    /// Visits a scrollable view.
    fn visit_scrollable(&mut self, area: Rectangle, action: fn(Vector) -> Action);

    /// Visits a solid colour.
    fn visit_solid(&mut self, area: Rectangle, colour: ThemeColour);

    /// Visits a text view.
    fn visit_text(&mut self, area: Rectangle, string: &str, alignment: Alignment);

    /// Visits a titled view.
    fn visit_titled(&mut self, area: Rectangle, title: &str, highlighted: bool);

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
    #[expect(clippy::too_many_lines, reason = "`View` is a large enum")]
    #[remain::check]
    pub fn accept<Ui: UserInterface, V: Visitor + ?Sized>(
        &self,
        visitor: &mut V,
        render_area: RenderArea,
    ) {
        #[sorted]
        match self {
            View::Bordered { thick, view } => compound!(
                visitor.visit_border(render_area.area, *thick),
                view.accept::<Ui, V>(visitor, inner_area::<Ui>(render_area)),
            ),
            View::Canvas {
                background,
                painter,
            } => visitor.visit_canvas(render_area.area, *background, painter),
            View::Clickable { on_click, view } => compound!(
                visitor.visit_clickable(render_area.area, on_click),
                view.accept::<Ui, V>(visitor, render_area),
            ),
            View::Contextual { menu, view } => compound!(
                visitor.visit_contextual(render_area.area, menu),
                view.accept::<Ui, V>(visitor, render_area),
            ),
            View::CursorWindow(window) => {
                if let Some(offset) = window.offset() {
                    visitor.visit_cursor_window(render_area.area, offset);
                }
            }
            View::Empty => (),
            View::Grabbable { object, view } => compound!(
                if let Some(object) = object(render_area) {
                    visitor.visit_grabbable(render_area.area, object);
                },
                view.accept::<Ui, V>(visitor, render_area),
            ),
            View::Hoverable { default, hovered } => {
                if render_area.is_hovered() {
                    hovered.accept::<Ui, V>(visitor, render_area);
                } else {
                    default.accept::<Ui, V>(visitor, render_area);
                }
            }
            View::Layers(layers) => {
                let visit = |layer: &View| layer.accept::<Ui, V>(visitor, render_area);

                if V::reverse_order() {
                    layers.iter().rev().for_each(visit);
                } else {
                    layers.iter().for_each(visit);
                }
            }
            View::ObjectAcceptor { drop, view } => compound!(
                visitor.visit_object_acceptor(render_area.area, drop),
                view.accept::<Ui, V>(visitor, render_area),
            ),
            View::Positioned { position, view } => {
                let max_size = Size {
                    width: render_area.area.size.width - position.x,
                    height: render_area.area.size.height - position.y,
                };
                let size = view.calculate_size::<Ui>(max_size, render_area);

                let position = render_area.area.position + *position;

                let area = Rectangle { position, size };

                if let Some(inner_area) = Rectangle::intersection(render_area.area, area) {
                    view.view
                        .accept::<Ui, V>(visitor, render_area.with_area(inner_area));
                }
            }
            View::Reactive(closure) => closure(render_area).accept::<Ui, V>(visitor, render_area),
            View::Rule { index, cells } => visitor.visit_rule(render_area.area, *index, *cells),
            View::Scrollable { action, view } => compound!(
                visitor.visit_scrollable(render_area.area, *action),
                view.accept::<Ui, V>(visitor, render_area),
            ),
            View::Selectable { item, view } => compound!(
                visitor.visit_selectable(render_area.area, *item),
                view.accept::<Ui, V>(visitor, render_area),
            ),
            View::SelectionBox => visitor.visit_selection_box(render_area.area),
            View::Shared(view) => view.accept::<Ui, V>(visitor, render_area),
            View::Solid(colour) => visitor.visit_solid(render_area.area, *colour),
            View::Stack { axis, elements } => {
                let rectangles = render_area.split::<Ui>(*axis, elements);

                for (area, quoted) in zip(rectangles, elements) {
                    quoted
                        .view
                        .accept::<Ui, V>(visitor, render_area.with_area(area));
                }
            }
            View::Text { string, alignment } => {
                visitor.visit_text(render_area.area, string, *alignment);
            }
            View::Titled {
                title,
                highlighted,
                view,
                ..
            } => {
                let titled_area = titled_area::<Ui>(render_area, title, view);

                if let View::Bordered { thick, view } = &**view {
                    let inner_area = inner_area::<Ui>(titled_area);
                    compound!(
                        visitor.visit_titled_bordered(
                            render_area.area,
                            titled_area.area,
                            title,
                            *highlighted,
                            *thick
                        ),
                        view.accept::<Ui, V>(visitor, inner_area),
                    );
                    return;
                }

                compound!(
                    visitor.visit_titled(render_area.area, title, *highlighted),
                    view.accept::<Ui, V>(visitor, titled_area),
                );
            }
        }
    }
}

fn titled_area<Ui: UserInterface>(
    mut render_area: RenderArea,
    title: &str,
    view: &View,
) -> RenderArea {
    let title_height = Ui::title_height(title, view);
    render_area.area.position.y += title_height;
    render_area.area.size.height -= title_height;
    render_area
}

fn inner_area<Ui: UserInterface>(mut render_area: RenderArea) -> RenderArea {
    render_area.area.position.x += Ui::BORDER_THICKNESS;
    render_area.area.position.y += Ui::BORDER_THICKNESS;
    render_area.area.size.width -= Ui::BORDER_THICKNESS * Ratio::integer(2);
    render_area.area.size.height -= Ui::BORDER_THICKNESS * Ratio::integer(2);
    render_area
}
