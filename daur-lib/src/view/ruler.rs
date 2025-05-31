use crate::UserInterface;
use crate::metre::OffsetMapping;
use crate::ui::{Direction, Length, Offset};
use crate::view::{View, feed};
use non_zero::non_zero;
use std::num::NonZeroU64;

/// A ruler of musical time
pub fn ruler<Ui: UserInterface>(offset: Length, offset_mapping: OffsetMapping) -> View {
    // TODO: don't use feed
    feed::<Ui, _>(Direction::Right, Offset::negative(offset), move |index| {
        if let Ok(measure_index) = usize::try_from(index) {
            let measure = offset_mapping.time_signature().measure_n(measure_index);
            let measure_width = measure.width(offset_mapping.quantisation());

            let cells = (measure_width / offset_mapping.quantisation().cell_width).ceil();
            let Some(cells) = NonZeroU64::new(cells) else {
                return View::Empty.quotated(measure_width);
            };

            View::Rule { index, cells }.quotated(measure_width)
        } else {
            let width = offset_mapping
                .time_signature()
                .first_measure()
                .width(offset_mapping.quantisation());

            View::Rule {
                index,
                cells: non_zero!(1),
            }
            .quotated(width)
        }
    })
}
