//! A simple terminal ui implementation of `daur`.

mod audio;
mod canvas;
mod controls;
mod convert;
mod draw;
mod event;
mod popup_handle;
mod tui;

use crate::audio::spawn_audio_thread;
use crate::controls::controls;
use crate::draw::spawn_draw_thread;
use crate::event::spawn_events_thread;
use crate::tui::Tui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use daur::App;
use std::io::{Result, stdout};
use std::panic::resume_unwind;
use std::sync::Arc;

fn main() -> Result<()> {
    execute!(stdout(), EnableMouseCapture)?;
    let terminal = ratatui::init();

    let app = &Arc::new(App::new(Tui::default()));

    app.controls.set_value(controls());

    let audio_thread = spawn_audio_thread(Arc::clone(app));
    let draw_thread = spawn_draw_thread(Arc::clone(app), terminal);
    let events_thread = spawn_events_thread(Arc::clone(app));

    app.ui.should_exit.wait_until();

    // TODO: save

    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)?;

    if audio_thread.is_finished() {
        let Err(error) = audio_thread.join();
        resume_unwind(error)
    }

    if draw_thread.is_finished() {
        match draw_thread.join() {
            Ok(error) => return Err(error),
            Err(error) => resume_unwind(error),
        };
    }

    if events_thread.is_finished() {
        match events_thread.join() {
            Ok(error) => return Err(error),
            Err(error) => resume_unwind(error),
        }
    }

    Ok(())
}
