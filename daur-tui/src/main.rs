//! A simple terminal ui implementation of `daur`.

mod audio;
mod canvas;
mod controls;
mod convert;
mod draw;
mod event;
mod tui;

use crate::audio::spawn_audio_thread;
use crate::controls::controls;
use crate::draw::redraw;
use crate::event::handle_event;
use crate::tui::Tui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, poll, read};
use crossterm::execute;
use daur::App;
use ratatui::DefaultTerminal;
use std::io;
use std::io::stdout;
use std::sync::Arc;
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
    let app = Arc::new(App::new(Tui::default()));

    // TODO: move to UserInterface
    app.controls.set_value(controls());

    // TODO: allocate more threads for faster rendering?
    spawn_audio_thread(Arc::clone(&app));

    let result = io_loop(&app, terminal);

    // TODO: save

    #[expect(clippy::let_and_return, reason = "see todo")]
    result
}

/// The main program loop that handles events and writes to the screen
/// This ensures that the project is saved if an error occurs.
fn io_loop(app: &App<Tui>, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !app.ui.should_exit.get() {
        if poll(Duration::ZERO)? {
            handle_event(&read()?, app);
        }

        if app.ui.should_redraw.get() {
            redraw(app, terminal)?;
        }
    }

    Ok(())
}
