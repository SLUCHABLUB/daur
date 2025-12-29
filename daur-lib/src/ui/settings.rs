use crate::UserInterface;
use crate::ui::Length;
use crate::ui::NonZeroLength;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct Settings {
    /// The height of the project bar.
    pub project_bar_height: NonZeroLength,
    /// The width of the track settings.
    pub track_settings_width: NonZeroLength,
    /// How far to the left the overview has been moved.
    pub negative_overview_offset: Length,
}

impl Settings {
    pub fn default_in<Ui: UserInterface>() -> Settings {
        Settings {
            project_bar_height: Ui::PROJECT_BAR_HEIGHT,
            track_settings_width: Ui::TRACK_SETTINGS_WITH,
            negative_overview_offset: Length::ZERO,
        }
    }
}
