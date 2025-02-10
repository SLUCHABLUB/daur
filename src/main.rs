#![deny(clippy::pedantic)]

mod app;
mod audio;
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
use std::io::stdout;

fn main() {
    execute!(stdout(), EnableMouseCapture).ok();

    let mut terminal = ratatui::init();
    App::new().run(&mut terminal);
    ratatui::restore();
}
