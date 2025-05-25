use crate::project::{bar, workspace};
use crate::{App, UserInterface, View};

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
        ),
    ]);

    let mut layers = vec![background];

    for instance in app.popup_manager.popups() {
        layers.push(instance.view());
    }

    if let Some(instance) = app.context_menu() {
        layers.push(instance.into_view());
    }

    View::Layers(layers)
}
