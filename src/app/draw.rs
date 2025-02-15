use crate::app::{or_popup, App};
use crate::widget::Widget;
use ratatui::DefaultTerminal;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub fn spawn_draw_thread(app: Arc<App>, mut terminal: DefaultTerminal) -> JoinHandle<()> {
    spawn(move || loop {
        while !app.should_redraw.get() {}

        app.draw(&mut terminal);
        app.should_redraw.set(app.is_playing());
    })
}

impl App {
    fn draw(&self, terminal: &mut DefaultTerminal) {
        or_popup!(
            terminal.draw(|frame| {
                let area = frame.area();
                let buf = frame.buffer_mut();
                let mouse_position = self.cached_mouse_position.get();

                self.cached_area.set(area);

                self.render(area, buf, mouse_position);
            }),
            self
        );
    }
}
