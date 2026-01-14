//! File for the [`minimum_size`] function.

use crate::Ratio;
use crate::UserInterface;
use crate::View;
use crate::ui::Length;
use crate::ui::Size;
use crate::view::Quotum;
use crate::view::RenderArea;
use std::cmp::max;

impl View {
    /// Returns the minimum size required to fit the entire view.
    #[must_use]
    pub fn minimum_size<Ui: UserInterface>(&self, render_area: RenderArea) -> Size {
        minimum_size::<Ui>(self, render_area)
    }
}

/// See [`View::minimum_size`].
/// Used to minimise indentation.
#[remain::check]
fn minimum_size<Ui: UserInterface>(view: &View, render_area: RenderArea) -> Size {
    #[sorted]
    match view {
        View::Bordered { view, .. } => {
            let mut size = view.minimum_size::<Ui>(render_area);
            size.height += Ui::BORDER_THICKNESS * Ratio::integer(2);
            size.width += Ui::BORDER_THICKNESS * Ratio::integer(2);
            size
        }
        View::Canvas { .. }
        | View::CursorWindow { .. }
        | View::Empty
        | View::SelectionBox
        | View::Solid(_) => Size::ZERO,
        View::Clickable { view, .. }
        | View::Contextual { view, .. }
        | View::Grabbable { view, .. }
        | View::Selectable { view, .. }
        | View::Scrollable { view, .. }
        | View::ObjectAcceptor { view, .. } => view.minimum_size::<Ui>(render_area),
        View::Hoverable { default, hovered } => {
            let default = default.minimum_size::<Ui>(render_area);
            let hovered = hovered.minimum_size::<Ui>(render_area);

            Size {
                width: max(default.width, hovered.width),
                height: max(default.height, hovered.height),
            }
        }
        View::Layers(layers) => {
            let mut size = Size::ZERO;

            for layer in layers {
                let layer_size = layer.minimum_size::<Ui>(render_area);
                size.width = max(size.width, layer_size.width);
                size.height = max(size.height, layer_size.height);
            }

            size
        }
        View::Positioned { position, view } => {
            let size = view.view.minimum_size::<Ui>(render_area);
            Size {
                width: position.x + size.width,
                height: position.y + size.height,
            }
        }
        View::Reactive(reactive) => reactive(render_area).minimum_size::<Ui>(render_area),
        View::Rule {
            left_crop, width, ..
        } => Size {
            width: *width - *left_crop,
            height: Ui::RULER_HEIGHT.get(),
        },
        View::Shared(view) => view.minimum_size::<Ui>(render_area),
        View::Stack { axis, elements } => {
            let mut parallel = Length::ZERO;
            let mut orthogonal = Length::ZERO;

            let mut fill_size = Length::ZERO;
            let mut fill_count: u64 = 0;

            for quoted in elements {
                let minimum = quoted.view.minimum_size::<Ui>(render_area);

                match quoted.quotum {
                    Quotum::Remaining => {
                        fill_size = max(fill_size, minimum.parallel_to(*axis));
                        fill_count = fill_count.saturating_add(1);
                    }
                    Quotum::Exact(length) => parallel += length,
                    Quotum::Minimum => parallel += minimum.parallel_to(*axis),
                }

                orthogonal = max(orthogonal, minimum.orthogonal_to(*axis));
            }

            parallel += fill_size * Ratio::integer(fill_count);

            Size::from_parallel_orthogonal(parallel, orthogonal, *axis)
        }
        View::Text { string, .. } => Size {
            width: Ui::string_width(string),
            height: Ui::string_height(string),
        },
        View::TitleBar { title, .. } => Size {
            width: Ui::string_width(title) + Ui::TITLE_PADDING * Ratio::integer(2),
            height: Ui::string_height(title) + Ui::TITLE_PADDING * Ratio::integer(2),
        },
    }
}
