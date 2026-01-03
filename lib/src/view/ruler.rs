use crate::View;
use crate::app::Action;
use crate::metre::OffsetMapping;
use crate::ui::Length;
use crate::view::Axis;
use crate::view::OnClick;
use crate::view::RenderArea;

/// A ruler of musical time
pub fn ruler(offset: Length, offset_mapping: OffsetMapping) -> View {
    let view = {
        let offset_mapping = offset_mapping.clone();

        View::reactive(move |render_area| reactive_ruler(offset, &offset_mapping, render_area))
    };

    let on_click = OnClick::new(move |render_area, actions| {
        let Some(mouse_position) = render_area.relative_mouse_position() else {
            return;
        };

        let offset_from_ruler_start = offset + mouse_position.x;

        let instant = offset_mapping.quantised_instant(offset_from_ruler_start);

        actions.push(Action::MoveCursor(instant));
    });

    view.on_click(on_click)
}

fn reactive_ruler(offset: Length, offset_mapping: &OffsetMapping, render_area: RenderArea) -> View {
    let quantisation = offset_mapping.quantisation();
    let mut measures = offset_mapping.time_signature().measures();

    let mut index_of_first_rule: usize = 0;
    let mut crop = offset;

    for measure in &mut measures {
        let width = measure.width(quantisation);

        if crop < width {
            break;
        }

        crop -= width;
        index_of_first_rule = index_of_first_rule.wrapping_add(1);
    }

    let first_measure = measures.next().unwrap_or_default();
    let first_width = first_measure.width(quantisation);

    let index_of_first_rule = index_of_first_rule;
    let measures = measures;
    let crop = crop;

    let mut remaining = render_area.area.size.width + crop - first_width;

    let first_rule = View::Rule {
        index: index_of_first_rule,
        cells: first_measure.cell_count(quantisation),
        left_crop: crop,
        width: first_width,
    };

    let mut rules = vec![first_rule];

    for (index, measure) in measures.clone().enumerate() {
        let index_offset = index.wrapping_add(1);

        let index = index_of_first_rule.wrapping_add(index_offset);
        let width = measure.width(quantisation);

        rules.push(View::Rule {
            index,
            cells: measure.cell_count(quantisation),
            left_crop: Length::ZERO,
            width,
        });

        remaining -= width;

        if remaining <= Length::ZERO {
            break;
        }
    }

    View::minimal_stack(Axis::X, rules)
}
