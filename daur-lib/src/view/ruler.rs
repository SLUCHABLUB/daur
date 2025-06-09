use crate::metre::OffsetMapping;
use crate::ui::Length;
use crate::view::{Axis, RenderArea, View};

/// A ruler of musical time
pub fn ruler(offset: Length, offset_mapping: OffsetMapping) -> View {
    View::reactive(move |render_area| reactive_ruler(offset, &offset_mapping, render_area))
}

fn reactive_ruler(offset: Length, offset_mapping: &OffsetMapping, render_area: RenderArea) -> View {
    let quantisation = offset_mapping.quantisation();
    let mut measures = offset_mapping.time_signature().measures();

    let mut first_index: usize = 0;
    let mut crop = offset;

    for measure in &mut measures {
        let width = measure.width(quantisation);

        if crop < width {
            break;
        }

        crop -= width;
        first_index = first_index.wrapping_add(1);
    }

    let first_measure = measures.next().unwrap_or_default();
    let first_width = first_measure.width(quantisation);

    let first_index = first_index;
    let measures = measures;
    let crop = crop;

    let mut remaining = render_area.area.size.width + crop - first_width;

    let first_rule = View::Rule {
        index: first_index,
        cells: first_measure.cell_count(quantisation),
        left_crop: crop,
        width: first_width,
    };

    let mut rules = vec![first_rule];

    for (index, measure) in measures.clone().enumerate() {
        let index = first_index.wrapping_add(1).wrapping_add(index);
        let width = measure.width(quantisation);

        rules.push(View::Rule {
            index,
            cells: measure.cell_count(quantisation),
            left_crop: Length::ZERO,
            width: measure.width(quantisation),
        });

        remaining -= width;

        if remaining <= Length::ZERO {
            break;
        }
    }

    View::minimal_stack(Axis::X, rules)
}
