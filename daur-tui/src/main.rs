//! A simple terminal ui implementation of `daur`

mod audio;
mod canvas;
mod controls;
mod convert;
mod draw;
mod event;

use crate::audio::spawn_audio_thread;
use crate::controls::controls;
use crate::draw::spawn_draw_thread;
use crate::event::spawn_events_thread;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use daur::{App, Cell};
use std::hint::spin_loop;
use std::io::{Result, stdout};
use std::panic::resume_unwind;
use std::sync::Arc;

static SHOULD_EXIT: Cell<bool> = Cell::new(false);

// TODO: use async instead of threading?
fn main() -> Result<()> {
    execute!(stdout(), EnableMouseCapture)?;
    let terminal = ratatui::init();

    let app = &Arc::new(App::new());

    app.controls.set_value(controls());

    let audio_thread = spawn_audio_thread(Arc::clone(app));
    let draw_thread = spawn_draw_thread(Arc::clone(app), terminal);
    let events_thread = spawn_events_thread(Arc::clone(app));

    while !SHOULD_EXIT.get() {
        spin_loop();
    }

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
