use crate::ui::{Length, Size};
use crate::{Ratio, UserInterface, View};
use std::cmp::max;

/// See [`View::minimum_size`]
pub(super) fn minimum_size<Ui: UserInterface>(view: &View) -> Size {
    match view {
        View::Bordered { thick: _, content } => {
            let mut size = content.minimum_size::<Ui>();
            size.height += Ui::BORDER_THICKNESS * Ratio::integer(2);
            size.width += Ui::BORDER_THICKNESS * Ratio::integer(2);
            size
        }
        View::Button {
            on_click: _,
            content,
        } => content.minimum_size::<Ui>(),
        View::Canvas { .. }
        | View::CursorWindow { .. }
        | View::Empty
        | View::FileSelector { .. }
        | View::SizeInformed(_)
        | View::Solid(_) => Size::ZERO,
        View::Contextual { menu: _, view } => view.minimum_size::<Ui>(),
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
        View::Sized { minimum_size, .. } => *minimum_size,
        View::Stack {
            direction,
            elements,
        } => {
            let mut parallel = Length::ZERO;
            let mut orthogonal = Length::ZERO;

            for quoted in elements {
                let child = quoted.view.minimum_size::<Ui>();
                parallel += child.parallel_to(*direction);
                orthogonal = max(orthogonal, child.orthogonal_to(*direction));
            }

            Size::from_parallel_orthogonal(parallel, orthogonal, *direction)
        }
        View::Text {
            string,
            alignment: _,
        } => Size {
            width: Ui::string_width(string),
            height: Ui::string_height(string),
        },
        View::Titled {
            title,
            highlighted: _,
            view,
        } => {
            let mut size = view.minimum_size::<Ui>();

            // The title gets cropped if the view is narrower than it.
            size.height += Ui::title_height(title, view);

            size
        }
        View::Window { area: _, view } => view.minimum_size::<Ui>(),
    }
}
