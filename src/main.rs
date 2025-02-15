#![deny(clippy::pedantic)]
#![deny(clippy::module_name_repetitions)]

mod app;
mod audio;
mod cell;
mod chroma;
mod clip;
mod key;
mod lock;
mod locked_tree;
mod locked_vec;
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

    let terminal = ratatui::init();
    App::new().run(terminal);
    ratatui::restore();
}
