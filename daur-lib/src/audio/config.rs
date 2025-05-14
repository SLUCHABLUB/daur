use crate::audio::{Player, SampleRate};
use crate::extension::OptionExt as _;
use crate::time::Instant;
use anyhow::Result;
use rodio::cpal::Host;
use rodio::cpal::traits::HostTrait as _;
use rodio::{Device, DeviceTrait as _, OutputStream, Sink};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("there are no available audio output devices")]
struct NoAvailableDevice;

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
    device: Device,
    stream_cache: Option<StreamCache>,
}

/// An audio stream cache.
struct StreamCache {
    #[expect(unused, reason = "if this is dropped, the sink will stop working")]
    stream: OutputStream,
    player: Player,
}

impl Config {
    fn initialise_device_config(&mut self) -> Result<&mut DeviceConfig> {
        let device = self.host.default_output_device().ok_or(NoAvailableDevice)?;

        Ok(self.device_config.get_or_insert(DeviceConfig {
            device,
            stream_cache: None,
        }))
    }

    pub(crate) fn player(&mut self) -> Result<Player> {
        let DeviceConfig {
            device,
            stream_cache: stream_config,
        } = self.initialise_device_config()?;

        stream_config
            .get_or_try_insert_with(|| {
                let (stream, handle) = OutputStream::try_from_device(device)?;
                let sink = Sink::try_new(&handle)?;
                let player = Player::from(sink);

                Ok(StreamCache { stream, player })
            })
            .map(|config| config.player.clone())
    }

    pub(crate) fn try_player(&self) -> Option<&Player> {
        Some(&self.device_config.as_ref()?.stream_cache.as_ref()?.player)
    }

    pub(crate) fn sample_rate(&mut self) -> Result<SampleRate> {
        Ok(self
            .initialise_device_config()?
            .device
            .default_output_config()?
            .sample_rate()
            .try_into()?)
    }

    pub(crate) fn is_player_playing(&self) -> bool {
        self.try_player().is_some_and(Player::is_playing)
    }

    pub(crate) fn player_position(&self) -> Option<Instant> {
        self.try_player().and_then(Player::position)
    }

    pub(crate) fn pause_player(&mut self) -> Option<Instant> {
        self.try_player().and_then(Player::pause)
    }
}
