use crate::ui::Size;
use crate::view::{Context, Direction};
use crate::{Colour, Ratio, UserInterface, View};
use itertools::Itertools as _;
use std::cmp::max;

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

    /// Constructs a new [stack](View::Stack) where all views are quotated equally.
    pub fn balanced_stack<Ui: UserInterface, E: IntoIterator<Item = Self>>(
        direction: Direction,
        elements: E,
    ) -> Self {
        let iter = elements.into_iter();
        let mut elements = Vec::new();
        let mut minimum_size = Size::ZERO;
        let mut count: u64 = 0;

        for element in iter {
            let size = element.minimum_size::<Ui>();
            minimum_size.width = max(minimum_size.width, size.width);
            minimum_size.height = max(minimum_size.height, size.height);
            count = count.saturating_add(1);

            elements.push(element.fill_remaining());
        }

        let count = Ratio::integer(count);

        match direction {
            Direction::Up | Direction::Down => minimum_size.height *= count,
            Direction::Left | Direction::Right => minimum_size.width *= count,
        }

        View::Sized {
            minimum_size,
            view: Box::new(View::Stack {
                direction,
                elements,
            }),
        }
    }

    /// Constructs a new [stack](View::Stack) where elements are quotated with their minimum size and spread out evenly.
    pub fn spaced_stack<Ui: UserInterface, E: IntoIterator<Item = Self>>(
        direction: Direction,
        elements: E,
    ) -> Self {
        View::Stack {
            direction,
            elements: elements
                .into_iter()
                .map(View::quotated_minimally::<Ui>)
                .intersperse_with(|| View::Empty.fill_remaining())
                .collect(),
        }
    }
}
