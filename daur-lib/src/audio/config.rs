use crate::Lock;
use crate::audio::{Player, SampleRate};
use crate::extension::{GuardExt as _, OptionExt as _};
use crate::time::real::Instant;
use anyhow::Result;
use parking_lot::{MappedRwLockWriteGuard, RwLockWriteGuard};
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
    host_config: Lock<HostConfig>,
}

#[derive(Default)]
struct HostConfig {
    host: Host,
    device_config: Option<DeviceConfig>,
}

struct DeviceConfig {
    device: Device,
    stream_config: Option<StreamConfig>,
}

struct StreamConfig {
    #[expect(unused, reason = "if this is dropped, the sink will stop working")]
    stream: OutputStream,
    player: Player,
}

impl Config {
    fn initialise_device_config(&self) -> Result<MappedRwLockWriteGuard<DeviceConfig>> {
        RwLockWriteGuard::map_result(self.host_config.write(), |host_config| {
            let device = host_config
                .host
                .default_output_device()
                .ok_or(NoAvailableDevice)?;

            Ok(host_config.device_config.get_or_insert(DeviceConfig {
                device,
                stream_config: None,
            }))
        })
    }

    pub(crate) fn player(&self) -> Result<Player> {
        let mut device_config = self.initialise_device_config()?;

        let DeviceConfig {
            device,
            stream_config,
        } = &mut *device_config;

        stream_config
            .get_or_try_insert_with(|| {
                let (stream, handle) = OutputStream::try_from_device(device)?;
                let sink = Sink::try_new(&handle)?;
                sink.pause();
                let player = Player::from(sink);

                Ok(StreamConfig { stream, player })
            })
            .map(|config| config.player.clone())
    }

    pub(crate) fn sample_rate(&self) -> Result<SampleRate> {
        Ok(self
            .initialise_device_config()?
            .device
            .default_output_config()?
            .sample_rate()
            .try_into()?)
    }

    // --- convenience methods ---

    pub(crate) fn is_player_playing(&self) -> bool {
        self.player().is_ok_and(|player| player.is_playing())
    }

    pub(crate) fn player_position(&self) -> Option<Instant> {
        self.player().ok()?.position()
    }

    pub(crate) fn pause_player(&self) -> Option<Instant> {
        self.player().ok()?.pause()
    }
}
