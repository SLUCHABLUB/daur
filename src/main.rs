#![doc = include_str!("../README.md")]

mod app;
mod audio;
mod cell;
mod chroma;
mod clip;
mod interval;
mod key;
mod keyboard;
mod length;
mod lock;
mod note;
mod notes;
mod pitch;
mod popup;
mod project;
mod ratio;
mod sign;
mod time;
mod track;
mod widget;

use app::App;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    execute!(stdout(), EnableMouseCapture)?;

    let terminal = ratatui::init();
    App::new().run(terminal);
    ratatui::restore();

    Ok(())
}
