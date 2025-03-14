use crate::app::{or_popup, App};
use crate::ui::Rectangle;
use crate::widget::Widget as _;
use never::Never;
use ratatui::DefaultTerminal;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

pub fn spawn_draw_thread(app: Arc<App>, mut terminal: DefaultTerminal) -> JoinHandle<Never> {
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
                let area = Rectangle::from_rect(frame.area());
                let buffer = frame.buffer_mut();
                let mouse_position = self.las_mouse_position.get();

                self.last_size.set(area.size);

                self.render(area, buffer, mouse_position);

                for popup in self.popups.to_stack() {
                    let area = popup.area_in_window(area);
                    popup.render(area, buffer, mouse_position);
                }
            }),
            self
        );
    }
}
