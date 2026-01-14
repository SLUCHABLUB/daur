//! Items pertaining to [`Config`].

use crate::audio::Player;
use crate::audio::sample;
use crate::extension::OptionExt as _;
use crate::time::Instant;
use rodio::Device;
use rodio::DeviceTrait as _;
use rodio::OutputStream;
use rodio::OutputStreamBuilder;
use rodio::Sink;
use rodio::cpal::Host;
use rodio::cpal::traits::HostTrait as _;
use thiserror::Error;

/// An error for when an audio device cannot be found.
#[derive(Debug, Error)]
#[error("there are no available audio output devices")]
struct NoAvailableOutputDevice;

/// An audio output configuration.
#[derive(Default)]
pub(crate) struct Config {
    /// The audio host
    host: Host,
    /// The device configuration
    device_config: Option<DeviceConfig>,
}

/// An audio device configuration.
struct DeviceConfig {
    /// The output device.
    device: Device,
    /// A cache of the output stream.
    stream_cache: Option<StreamCache>,
}

/// An audio stream cache.
struct StreamCache {
    /// The output stream to which audio is written.
    ///
    /// This is not written to directly, but via a [sink](`Sink`).
    #[expect(unused, reason = "if this is dropped, the sink will stop working")]
    stream: OutputStream,
    /// An audio player linked to the output stream.
    player: Player,
}

impl Config {
    /// Tries to to initialise a [`DeviceConfig`].
    ///
    /// # Errors
    ///
    /// An error is returned if there is no available output device.
    fn initialise_device_config(&mut self) -> anyhow::Result<&mut DeviceConfig> {
        let device = self
            .host
            .default_output_device()
            .ok_or(NoAvailableOutputDevice)?;

        Ok(self.device_config.get_or_insert(DeviceConfig {
            device,
            stream_cache: None,
        }))
    }

    /// Returns the saved audio player or tries to create a new one.
    ///
    /// # Errors
    ///
    /// If no player has been created, and a new one fails to be created, an error will be returned.
    pub(crate) fn player(&mut self) -> anyhow::Result<Player> {
        let DeviceConfig {
            device,
            stream_cache: stream_config,
        } = self.initialise_device_config()?;

        stream_config
            .get_or_try_insert_with(|| {
                let stream = OutputStreamBuilder::from_device(device.clone())?.open_stream()?;
                let sink = Sink::connect_new(stream.mixer());
                let player = Player::from(sink);

                Ok(StreamCache { stream, player })
            })
            .map(|config| config.player.clone())
    }

    /// Returns the saved audio player, or `None` if none is saved.
    pub(crate) fn try_player(&self) -> Option<&Player> {
        Some(&self.device_config.as_ref()?.stream_cache.as_ref()?.player)
    }

    /// Returns the preferred sample rate of the audio device.
    ///
    /// # Errors
    ///
    /// If no audio output device can be found, or it it will not report a sample rate,
    /// an error will be returned.
    pub(crate) fn sample_rate(&mut self) -> anyhow::Result<sample::Rate> {
        Ok(self
            .initialise_device_config()?
            .device
            .default_output_config()?
            .sample_rate()
            .try_into()?)
    }

    /// Returns whether the saved audio player is currently playing audio
    /// (provided that it exists).
    pub(crate) fn is_player_playing(&self) -> bool {
        self.try_player().is_some_and(Player::is_playing)
    }

    /// Returns the position of the saved audio player
    /// (provided that it exists).
    pub(crate) fn player_position(&self) -> Option<Instant> {
        self.try_player().and_then(Player::position)
    }

    /// Pauses the saved audio player
    /// (provided that it exists).
    pub(crate) fn pause_player(&mut self) -> Option<Instant> {
        self.try_player().and_then(Player::pause)
    }
}
