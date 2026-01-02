//! A simple terminal ui implementation of `daur`.

mod canvas;
mod configuration;
mod convert;
mod draw;
mod event;
mod key;
mod tui;

use crate::draw::redraw;
use crate::event::handle_events;
use crate::key::Key;
use crate::tui::Tui;
use anyhow::Context as _;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyboardEnhancementFlags;
use crossterm::event::PopKeyboardEnhancementFlags;
use crossterm::event::PushKeyboardEnhancementFlags;
use crossterm::event::poll;
use crossterm::event::read;
use crossterm::execute;
use daur::App;
use directories::ProjectDirs;
use ratatui::DefaultTerminal;
use std::io;
use std::io::stdout;
use std::time::Duration;

// TODO: clean this up
fn main() -> anyhow::Result<()> {
    // The first two arguments are for the organisation name and domain.
    // However, since we don't have an organisation, they're fine to leave empty.
    let directories =
        ProjectDirs::from("", "", "daur").context("unable to determine project directories")?;

    execute!(stdout(), EnableMouseCapture)?;
    let terminal = &mut ratatui::init();
    execute!(
        stdout(),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all())
    )?;

    let result = in_terminal(terminal, &directories);

    execute!(stdout(), PopKeyboardEnhancementFlags)?;
    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)?;

    result
}

/// Runs the app in a given terminal.
/// This ensures that the terminal is properly closed if an error occurs.
fn in_terminal(terminal: &mut DefaultTerminal, directories: &ProjectDirs) -> anyhow::Result<()> {
    let tui = Tui::new(directories)?;

    let tui: &'static Tui = Box::leak(Box::new(tui));

    let mut app = App::new(tui);

    let result = io_loop(&mut app, terminal);

    // TODO: save

    #[expect(clippy::let_and_return, reason = "see todo")]
    result
}

/// The main program loop that handles events and writes to the screen
/// This ensures that the project is saved if an error occurs.
fn io_loop(app: &mut App<Tui>, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
    while !app.ui().should_exit.get() {
        handle_events(&available_events()?, app);

        if app.ui().should_redraw {
            redraw(app, terminal)?;
        }
    }

    Ok(())
}

/// Returns all available events without blocking.
fn available_events() -> io::Result<Vec<Event>> {
    let mut events = Vec::new();

    while poll(Duration::ZERO)? {
        events.push(read()?);
    }

    Ok(events)
}
