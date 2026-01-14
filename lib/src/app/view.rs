//! File for the [`view`] function.

use crate::App;
use crate::UserInterface;
use crate::View;
use crate::project::bar;
use crate::project::workspace;

/// Constructs a [view](View) of the [app](App).
pub(super) fn view<Ui: UserInterface>(app: &App<Ui>) -> View {
    let background = View::y_stack([
        bar::<Ui>(
            app.project_manager.project(),
            app.cursor(),
            app.audio_config.try_player().cloned(),
            app.edit_mode,
            app.piano_roll.is_open(),
        )
        .quoted(app.ui_settings.project_bar_height),
        workspace::<Ui>(
            app.project_manager.project(),
            &app.selection,
            app.ui_settings,
            app.quantisation,
            app.cursor(),
            app.audio_config.try_player(),
            app.held_object,
        )
        .fill_remaining(),
        app.piano_roll
            .view::<Ui>()
            .selection(&app.selection)
            .project(app.project_manager.project())
            .quantisation(app.quantisation)
            .cursor(app.cursor())
            .maybe_player(app.audio_config.try_player().cloned())
            .maybe_held_object(app.held_object)
            .edit_mode(app.edit_mode)
            .call(),
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
