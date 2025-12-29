use crate::Ratio;
use crate::UserInterface;
use crate::ui::Length;
use crate::ui::Offset;
use crate::ui::Point;
use crate::ui::Rectangle;
use crate::ui::Size;
use crate::ui::relative;
use crate::view::Axis;
use crate::view::Quotated;
use non_zero::non_zero;
use saturating_cast::SaturatingCast as _;
use std::cmp::min;
use std::num::NonZeroU64;

/// Information about the user interface that a reactive view may use.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub struct RenderArea {
    /// The area of the view.
    pub area: Rectangle,
    /// The position of the mouse cursor.
    pub mouse_position: Point,
}

impl RenderArea {
    /// Whether the area of the view contains the mouse.
    #[must_use]
    pub fn is_hovered(self) -> bool {
        self.area.contains(self.mouse_position)
    }

    /// Returns the position of the mouse cursor relative to the area.
    #[must_use]
    pub fn relative_mouse_position(self) -> Option<relative::Point> {
        self.is_hovered()
            .then_some(self.mouse_position.relative_to(self.area.position))
    }

    /// Returns the position of the mouse cursor relative to the area.
    /// If it is not within the area, it is snapped to an edge.
    #[must_use]
    pub fn saturated_mouse_position(self) -> relative::Point {
        let point = self.mouse_position.relative_to(self.area.position);

        relative::Point {
            x: min(point.x, self.area.size.width - Length::PIXEL),
            y: min(point.y, self.area.size.height - Length::PIXEL),
        }
    }

    /// Returns a moved copy of the rendering area.
    #[must_use]
    pub fn with_area(mut self, area: Rectangle) -> RenderArea {
        self.area = area;
        self
    }

    /// Splits the area.
    #[must_use]
    pub(crate) fn split<Ui: UserInterface>(
        self,
        axis: Axis,
        views: &[Quotated],
    ) -> impl DoubleEndedIterator<Item = Rectangle> + use<'_, Ui> {
        let count = views.len();

        // cache the sizes or None if Quotum::Remaining is used
        let sizes: Vec<Option<Length>> = views
            .iter()
            .map(|quotated| quotated.size_parallel_to::<Ui>(axis, self))
            .collect();

        let orthogonal = self.area.size.orthogonal_to(axis);

        // the size that will be allocated to the `Quotum::Remaining` quota
        let mut fill_size = self.area.size.parallel_to(axis);

        let mut fill_count: u64 = 0;

        for size in &sizes {
            if let Some(size) = *size {
                fill_size -= size;
            } else {
                fill_count = fill_count.saturating_add(1);
            }
        }

        // the space between elements
        let spacing;

        if let Some(fill_count) = NonZeroU64::new(fill_count) {
            fill_size *= Ratio::reciprocal_of(fill_count);
            spacing = Length::ZERO;
        } else {
            let space_count =
                NonZeroU64::new(count.saturating_sub(1).saturating_cast()).unwrap_or(non_zero!(1));
            spacing = fill_size * Ratio::reciprocal_of(space_count);
        }

        let mut offset = Offset::ZERO;

        let next_to_last = count.saturating_sub(2);
        let last_size = sizes
            .last()
            .copied()
            .unwrap_or_default()
            .unwrap_or(fill_size);

        sizes
            .into_iter()
            .enumerate()
            .filter_map(move |(index, size)| {
                let parallel = size.unwrap_or(fill_size);

                let position = self.area.position + axis * offset;

                offset += parallel;

                if index == next_to_last {
                    let last_spacing: Length =
                        self.area.size.parallel_to(axis) - last_size - offset;
                    offset += last_spacing;
                } else {
                    offset += spacing;
                }

                self.area.intersection(Rectangle {
                    position,
                    size: Size::from_parallel_orthogonal(parallel, orthogonal, axis),
                })
            })
    }
}
