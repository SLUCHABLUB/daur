//! A simple terminal ui implementation of `daur`.

mod canvas;
mod controls;
mod convert;
mod draw;
mod event;
mod tui;

use crate::draw::redraw;
use crate::event::handle_events;
use crate::tui::Tui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, poll, read};
use crossterm::execute;
use daur::App;
use ratatui::DefaultTerminal;
use std::io;
use std::io::stdout;
use std::time::Duration;

fn main() -> io::Result<()> {
    execute!(stdout(), EnableMouseCapture)?;
    let terminal = &mut ratatui::init();

    let result = in_terminal(terminal);

    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)?;

    result
}

/// Runs the app in a given terminal.
/// This ensures that the terminal is properly closed if an error occurs.
fn in_terminal(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let app = App::new(Tui::default());

    let result = io_loop(&app, terminal);

    // TODO: save

    #[expect(clippy::let_and_return, reason = "see todo")]
    result
}

/// The main program loop that handles events and writes to the screen
/// This ensures that the project is saved if an error occurs.
fn io_loop(app: &App<Tui>, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !app.ui().should_exit() {
        handle_events(&available_events()?, app);

        if app.ui().should_redraw() {
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
