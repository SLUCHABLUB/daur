//! Items pertaining to the terminal.

use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::KeyboardEnhancementFlags;
use crossterm::event::PopKeyboardEnhancementFlags;
use crossterm::event::PushKeyboardEnhancementFlags;
use crossterm::execute;
use ratatui::DefaultTerminal;
use std::io::stdout;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;
use std::panic::resume_unwind;

/// Runs a function with the terminal initialised.
///
/// This will make sure that the terminal is cleaned up after running the function
/// (including if it panics) (but not if an [IO error](std::io::Error) occurs during cleanup).
pub(crate) fn with_terminal<F>(f: F) -> anyhow::Result<()>
where
    F: FnOnce(DefaultTerminal) -> anyhow::Result<()>,
{
    execute!(stdout(), EnableMouseCapture)?;
    let terminal = ratatui::init();
    execute!(
        stdout(),
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all())
    )?;

    let result = catch_unwind(AssertUnwindSafe(|| f(terminal)));

    execute!(stdout(), PopKeyboardEnhancementFlags)?;
    ratatui::restore();
    execute!(stdout(), DisableMouseCapture)?;

    match result {
        Ok(result) => result,
        Err(panic_payload) => {
            resume_unwind(panic_payload);
        }
    }
}
