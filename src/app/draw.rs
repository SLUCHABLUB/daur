use crate::app::reference::AppShare;
use crate::widget::Widget;
use ratatui::DefaultTerminal;
use std::io;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub fn spawn_draw_thread(
    app: Arc<AppShare>,
    mut terminal: DefaultTerminal,
) -> JoinHandle<io::Result<()>> {
    spawn(move || {
        while !app.should_exit() {
            if !app.should_redraw() {
                continue;
            }

            let result = terminal.draw(|frame| {
                let area = frame.area();
                let buf = frame.buffer_mut();
                let mouse_position = app.mouse_position();

                app.set_area(area);

                app.read_lock().render(area, buf, mouse_position);
            });

            if result.is_err() {
                app.set_exit();
                return result.map(|_| ());
            }
        }

        Ok(())
    })
}
