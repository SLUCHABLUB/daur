use crate::View;
use crate::ui::{Colour, Size};
use crate::view::{Axis, Context, Quotated};

impl View {
    /// Constructs a new [canvas](View::Canvas).
    pub fn canvas<Painter>(background: Colour, painter: Painter) -> View
    where
        Painter: Fn(&mut dyn Context) + Send + Sync + 'static,
    {
        View::Canvas {
            background,
            painter: Box::new(painter),
        }
    }

    /// Constructs a new view from a [generator](View::Generator).
    pub fn generator<F: Fn() -> View + Send + Sync + 'static>(generator: F) -> Self {
        View::Generator(Box::new(generator))
    }

    /// Constructs a new [hoverable](View::Hoverable) view.
    pub fn hoverable(default: View, hovered: View) -> Self {
        View::Hoverable {
            default: Box::new(default),
            hovered: Box::new(hovered),
        }
    }

    /// Constructs a new [size-informed](View::SizeInformed) view.
    pub fn size_informed<F: Fn(Size) -> View + Send + Sync + 'static>(generator: F) -> Self {
        View::SizeInformed(Box::new(generator))
    }

    /// Constructs a new horizontal [stack](View::Stack).
    pub fn x_stack<E: IntoIterator<Item = Quotated>>(elements: E) -> Self {
        View::Stack {
            axis: Axis::X,
            elements: elements.into_iter().collect(),
        }
    }

    /// Constructs a new vertical [stack](View::Stack).
    pub fn y_stack<E: IntoIterator<Item = Quotated>>(elements: E) -> Self {
        View::Stack {
            axis: Axis::Y,
            elements: elements.into_iter().collect(),
        }
    }

    /// Constructs a new [stack](View::Stack) where all views are quotated equally.
    pub fn balanced_stack<E: IntoIterator<Item = Self>>(axis: Axis, elements: E) -> Self {
        View::Stack {
            axis,
            elements: elements.into_iter().map(View::fill_remaining).collect(),
        }
    }

    /// Constructs a new [stack](View::Stack) where elements are quotated with their minimum size.
    pub fn minimal_stack<E: IntoIterator<Item = Self>>(axis: Axis, elements: E) -> Self {
        View::Stack {
            axis,
            elements: elements.into_iter().map(View::quotated_minimally).collect(),
        }
    }
}
