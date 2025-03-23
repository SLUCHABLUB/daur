use crate::tui::Tui;
use daur::{App, Popup};
use never::Never;
use rodio::cpal::SampleRate;
use rodio::{DeviceTrait as _, OutputStream, Sink};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hint::spin_loop;
use std::sync::Arc;
use std::thread::{JoinHandle, spawn};

pub fn spawn_audio_thread(app: Arc<App<Tui>>) -> JoinHandle<Never> {
    spawn(move || {
        let mut cache = None;

        loop {
            let start = loop {
                if let Some(start) = app.playback_start.get() {
                    break Some(start);
                }

                // TODO: render the audio instead of just spinning
                spin_loop();
            };

            let (sink, sample_rate) = match cache.as_ref() {
                Some((sink, sample_rate, _output_stream)) => (sink, *sample_rate),
                None => match get_sink(&app) {
                    Ok(new) => {
                        let (sink, sample_rate, _output_stream) = cache.insert(new);
                        (&*sink, *sample_rate)
                    }
                    Err(error) => {
                        app.popups.open(&error, &app.ui);
                        // Stop playback without moving the cursor
                        app.playback_start.set(None);
                        continue;
                    }
                },
            };

            sink.clear();
            sink.append(app.project.source(sample_rate, app.cursor.get()));
            sink.play();

            while app.playback_start.get() == start {
                spin_loop();
            }
        }
    })
}

#[derive(Debug)]
struct NoSelectedDevice;

impl Display for NoSelectedDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "no audio-output device is selected")
    }
}

impl Error for NoSelectedDevice {}

fn get_sink(app: &App<Tui>) -> Result<(Sink, u32, OutputStream), Popup> {
    let device = app.device.get().ok_or(NoSelectedDevice)?;
    let config = device.default_output_config()?;
    let SampleRate(sample_rate) = config.sample_rate();

    let (output_stream, handle) = OutputStream::try_from_device(&device)?;
    let sink = Sink::try_new(&handle)?;

    Ok((sink, sample_rate, output_stream))
}
