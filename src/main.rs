#![deny(clippy::pedantic)]

mod app;
mod chroma;
mod clip;
mod columns;
mod id;
mod key;
mod popup;
mod project;
mod sign;
mod time;
mod track;
mod widget;

use app::App;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    execute!(stdout(), EnableMouseCapture).ok();

    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
