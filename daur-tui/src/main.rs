//! A simple terminal ui implementation of `daur`

use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use daur::App;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    execute!(stdout(), EnableMouseCapture)?;

    let terminal = ratatui::init();
    App::new().run(terminal);
    ratatui::restore();

    Ok(())
}
