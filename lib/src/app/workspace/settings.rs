//! Items pertaining to [`Settings`].

use crate::Holdable;
use crate::Project;
use crate::Ratio;
use crate::UserInterface;
use crate::View;
use crate::app::Workspace;
use crate::app::workspace::PianoRoll;
use crate::app::workspace::plugins::Plugins;
use crate::audio::Player;
use crate::metre::Instant;
use crate::metre::Quantisation;
use crate::select::Selection;
use crate::ui::Length;
use crate::ui::Vector;
use crate::view::Quoted;
use bon::bon;

/// Volatile settings for the workspace.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Settings {
    /// Settings for the piano roll.
    pub piano_roll: PianoRoll,
    /// Settings for the plugin workspace.
    pub plugins: Plugins,

    /// The height on the content inside the workspace pane.
    /// This is the height of the pane minus that of the title bar.
    pub content_height: Length,
    /// The currently open workspace, if any.
    pub currently_open: Option<Workspace>,
}

#[bon]
impl Settings {
    /// Returns the default workspace settings for a given ui.
    pub(crate) fn default_in<Ui>() -> Settings
    where
        Ui: UserInterface,
    {
        // The height of 3 octaves in the piano roll.
        let three_octaves = Ui::KEY_WIDTH.get() * Ratio::integer(3 * 12) + Ui::RULER_HEIGHT.get();

        Settings {
            piano_roll: PianoRoll::default_in::<Ui>(),
            plugins: Plugins,
            content_height: three_octaves,
            currently_open: None,
        }
    }

    /// The full view of the workspace that the settings configure.
    #[builder]
    pub(crate) fn view<Ui>(
        &self,
        cursor: Instant,
        edit_mode: bool,
        held_object: Option<Holdable>,
        player: Option<Player>,
        project: &Project,
        quantisation: Quantisation,
        selection: &Selection,
    ) -> Quoted
    where
        Ui: UserInterface,
    {
        let Some(currently_open) = self.currently_open else {
            return Quoted::EMPTY;
        };

        let (title, highlighted) = match currently_open {
            Workspace::PianoRoll => PianoRoll::title_and_highlight(project, selection),
            Workspace::Plugins => (Plugins::title(project, selection), true),
        };

        let content = match currently_open {
            Workspace::PianoRoll => self
                .piano_roll
                .content::<Ui>()
                .content_height(self.content_height)
                .cursor(cursor)
                .edit_mode(edit_mode)
                .maybe_held_object(held_object)
                .maybe_player(player)
                .project(project)
                .quantisation(quantisation)
                .selection(selection)
                .call(),
            Workspace::Plugins => self.plugins.content(),
        };

        let title_height = Ui::string_height(&title) + Ui::TITLE_PADDING * Ratio::integer(2);

        let title_bar = View::TitleBar { title, highlighted }.grabbable(Workspace::handle_grabber);

        View::y_stack([title_bar.quoted_minimally(), content.fill_remaining()])
            .quoted(self.content_height + title_height)
    }

    /// Moves the workspace.
    pub(crate) fn move_by<Ui>(&mut self, by: Vector)
    where
        Ui: UserInterface,
    {
        match self.currently_open {
            Some(Workspace::PianoRoll) => self.piano_roll.move_by_in::<Ui>(by, self.content_height),
            Some(Workspace::Plugins) => self.plugins.move_by(by),
            None => (),
        }
    }
}
