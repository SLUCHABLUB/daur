#![allow(
    unused_crate_dependencies,
    reason = "they are used adn caught by the library"
)]
#![doc = include_str!("../README.md")]

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
