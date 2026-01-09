use crate::View;
use crate::ui::Colour;
use crate::view::Axis;
use crate::view::Context;
use crate::view::Quoted;
use crate::view::RenderArea;

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

    /// Constructs a new [hoverable](View::Hoverable) view.
    pub fn hoverable(default: View, hovered: View) -> View {
        View::Hoverable {
            default: Box::new(default),
            hovered: Box::new(hovered),
        }
    }

    /// Constructs a new [reactive view](View::Reactive).
    pub fn reactive<Closure>(closure: Closure) -> View
    where
        Closure: Fn(RenderArea) -> View + Send + Sync + 'static,
    {
        View::Reactive(Box::new(closure))
    }

    /// Constructs a new horizontal [stack](View::Stack).
    pub fn x_stack<E: IntoIterator<Item = Quoted>>(elements: E) -> View {
        View::Stack {
            axis: Axis::X,
            elements: elements.into_iter().collect(),
        }
    }

    /// Constructs a new vertical [stack](View::Stack).
    pub fn y_stack<E: IntoIterator<Item = Quoted>>(elements: E) -> View {
        View::Stack {
            axis: Axis::Y,
            elements: elements.into_iter().collect(),
        }
    }

    /// Constructs a new [stack](View::Stack) where all views are quoted equally.
    pub fn balanced_stack<E: IntoIterator<Item = View>>(axis: Axis, elements: E) -> View {
        View::Stack {
            axis,
            elements: elements.into_iter().map(View::fill_remaining).collect(),
        }
    }

    /// Constructs a new [stack](View::Stack) where elements are quoted with their minimum size.
    pub fn minimal_stack<E: IntoIterator<Item = View>>(axis: Axis, elements: E) -> View {
        View::Stack {
            axis,
            elements: elements.into_iter().map(View::quoted_minimally).collect(),
        }
    }
}
