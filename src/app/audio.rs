use crate::app::action::Action;
use crate::app::macros::or_popup;
use crate::app::App;
use crate::popup::Popup;
use rodio::cpal::traits::HostTrait;
use rodio::{DeviceTrait, OutputStream, Sink};
use std::hint::spin_loop;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};

// TODO: base upon the device
const SAMPLE_RATE: u32 = 44_100;

pub fn spawn_audio_thread(app: Arc<App>) -> JoinHandle<()> {
    spawn(move || {
        loop {
            // TODO: render the audio instead of just spinning
            while !app.is_playing() {
                spin_loop();
            }

            let Some((sink, _output_stream)) = app.sink() else {
                app.stop_playback();
                continue;
            };

            sink.clear();
            sink.append(app.project.to_source(SAMPLE_RATE, app.cursor.get()));
            sink.play();

            while app.is_playing() {
                spin_loop();
            }
        }
    })
}

impl App {
    // TODO: cache output stream and sink
    fn sink(&self) -> Option<(Sink, OutputStream)> {
        // Taking here temporarily is fine sine we are the only thread reading this value
        if let Some(device) = self.device.take() {
            self.device.set(Some(device.clone()));

            let (output_stream, handle) = or_popup!(OutputStream::try_from_device(&device), self);
            let sink = or_popup!(Sink::try_new(&handle), self);
            return Some((sink, output_stream));
        }

        let devices = or_popup!(self.host.output_devices(), self);
        self.popups.push(Popup::buttons(devices.map(|device| {
            (
                device.name().unwrap_or_else(|error| error.to_string()),
                Action::SetDevice(device),
            )
        })));

        None
    }
}
