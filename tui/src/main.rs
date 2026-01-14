//! A simple terminal ui implementation of [`daur`].

mod canvas;
mod configuration;
mod convert;
mod draw;
mod event;
mod key;
mod terminal;
mod tui;

pub(crate) use configuration::Configuration;
pub(crate) use key::Key;
pub(crate) use tui::Tui;

use crate::draw::redraw;
use crate::event::handle_events;
use crate::terminal::with_terminal;
use anyhow::Context as _;
use crossterm::event::Event;
use crossterm::event::poll;
use crossterm::event::read;
use daur::App;
use directories::ProjectDirs;
use ratatui::DefaultTerminal;
use std::io;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // The first two arguments are for the organisation name and domain.
    // However, since we don't have an organisation, they're fine to leave empty.
    let directories =
        ProjectDirs::from("", "", "daur").context("unable to determine project directories")?;

    let tui = Tui::new(&directories)?;

    let tui: &'static Tui = Box::leak(Box::new(tui));

    let mut app = App::new(tui);

    with_terminal(|mut terminal| io_loop(&mut app, &mut terminal))
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
