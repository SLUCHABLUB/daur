use crate::ui::{Length, Size};
use crate::view::Quotum;
use crate::{Ratio, UserInterface, View};
use core::cmp::max;

impl View {
    /// Returns the minimum size required to fit the entire view.
    #[must_use]
    pub fn minimum_size<Ui: UserInterface>(&self) -> Size {
        minimum_size::<Ui>(self)
    }
}

/// See [`View::minimum_size`].
/// Used to minimise indentation.
fn minimum_size<Ui: UserInterface>(view: &View) -> Size {
    match view {
        View::Bordered { view, .. } => {
            let mut size = view.minimum_size::<Ui>();
            size.height += Ui::BORDER_THICKNESS * Ratio::integer(2);
            size.width += Ui::BORDER_THICKNESS * Ratio::integer(2);
            size
        }
        View::Canvas { .. }
        | View::CursorWindow { .. }
        | View::Empty
        | View::SizeInformed(_)
        | View::Solid(_) => Size::ZERO,
        View::Clickable { view, .. }
        | View::Contextual { view, .. }
        | View::Grabbable { view, .. }
        | View::Scrollable { view, .. } => view.minimum_size::<Ui>(),
        View::Generator(generator) => generator().minimum_size::<Ui>(),
        View::Hoverable { default, hovered } => {
            let default = default.minimum_size::<Ui>();
            let hovered = hovered.minimum_size::<Ui>();

            Size {
                width: max(default.width, hovered.width),
                height: max(default.height, hovered.height),
            }
        }
        View::Layers(layers) => {
            let mut size = Size::ZERO;

            for layer in layers {
                let layer_size = layer.minimum_size::<Ui>();
                size.width = max(size.width, layer_size.width);
                size.height = max(size.height, layer_size.height);
            }

            size
        }
        View::Rule { .. } => Size {
            width: Length::ZERO,
            height: Ui::RULER_HEIGHT.get(),
        },
        View::Stack { axis, elements } => {
            let mut parallel = Length::ZERO;
            let mut orthogonal = Length::ZERO;

            let mut fill_size = Length::ZERO;
            let mut fill_count: u64 = 0;

            for quoted in elements {
                let minimum = quoted.view.minimum_size::<Ui>();

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

            parallel += fill_size * Ratio::from(fill_count);

            Size::from_parallel_orthogonal(parallel, orthogonal, *axis)
        }
        View::Text { string, .. } => Size {
            width: Ui::string_width(string),
            height: Ui::string_height(string),
        },
        View::Titled { title, view, .. } => {
            let mut size = view.minimum_size::<Ui>();

            // The title gets cropped if the view is narrower than it.
            size.height += Ui::title_height(title, view);

            size
        }
        View::Window { view, .. } => view.minimum_size::<Ui>(),
    }
}
