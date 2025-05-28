use crate::project::{bar, workspace};
use crate::ui::relative;
use crate::{App, HoldableObject, UserInterface, View};

pub(super) fn view<Ui: UserInterface>(app: &App<Ui>) -> View {
    let background = View::y_stack([
        bar::<Ui>(
            app.project_manager.project(),
            app.audio_config.try_player().cloned(),
            app.edit_mode,
            app.piano_roll.is_open,
        )
        .quotated(app.project_bar_height.get()),
        workspace::<Ui>(
            app.project_manager.project(),
            app.selection,
            app.track_settings_width,
            app.negative_overview_offset,
            app.grid,
            app.cursor(),
            app.audio_config.try_player(),
        )
        .fill_remaining(),
        app.piano_roll.view::<Ui>(
            app.selection,
            app.project_manager.project(),
            app.grid,
            app.audio_config.try_player().cloned(),
            app.cursor,
            app.held_object,
            app.edit_mode,
        ),
    ]);

    let mut layers = vec![background];

    for instance in app.popup_manager.popups() {
        layers.push(instance.view());
    }

    if let Some(instance) = app.context_menu() {
        layers.push(instance.into_view());
    }

    if let Some(HoldableObject::SelectionBox { start }) = app.held_object {
        layers.push(View::reactive(move |render_area| {
            let start = start.relative_to(render_area.area.position);
            let Some(end) = render_area.relative_mouse_position() else {
                return View::Empty;
            };

            let area = relative::Rectangle::containing_both(start, end);

            View::SelectionBox.positioned(area)
        }));
    }

    View::Layers(layers)
}
