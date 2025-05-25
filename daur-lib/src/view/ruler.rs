use crate::ui::{Direction, Grid, Length, Offset};
use crate::view::{View, feed};
use crate::{UserInterface, project};
use non_zero::non_zero;
use std::num::NonZeroU64;

/// A ruler of musical time
pub fn ruler<Ui: UserInterface>(
    offset: Length,
    project_settings: project::Settings,
    grid: Grid,
) -> View {
    // TODO: don't use feed
    feed::<Ui, _>(Direction::Right, Offset::negative(offset), move |index| {
        if let Ok(bar_index) = usize::try_from(index) {
            let bar = project_settings.time_signature.bar_n(bar_index);
            let bar_width = bar.width(grid);

            let cells = (bar_width / grid.cell_width).ceil();
            let Some(cells) = NonZeroU64::new(cells) else {
                return View::Empty.quotated(bar_width);
            };

            View::Rule { index, cells }.quotated(bar_width)
        } else {
            let width = project_settings.time_signature.first_bar().width(grid);

            View::Rule {
                index,
                cells: non_zero!(1),
            }
            .quotated(width)
        }
    })
}
