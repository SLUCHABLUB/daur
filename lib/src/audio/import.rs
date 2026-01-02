use crate::Audio;
use crate::audio::Sample;
use crate::audio::sample;
use crate::audio::sample::ZeroRateError;
use anyhow::Result;
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::io::MediaSourceStreamOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_codecs;
use symphonia::default::get_probe;
use thiserror::Error;

/// An error when importing audio from a file.
#[derive(Debug, Error)]
pub enum ImportError {
    /// An error when reading the file.
    #[error("Error reading the audio file: {0}")]
    Io(#[from] io::Error),
    /// AN error when decoding the file.
    #[error("{0}")]
    Symphonia(#[from] SymphoniaError),
    /// The file had a sample rate of 0.
    #[error("{0}")]
    ZeroSampleRate(#[from] ZeroRateError),
    /// The packets in the file had different sample rates.
    #[error("audio packets had different sample rates")]
    SampleRateInconsistency,
    /// The audio file contained no audio packets.
    #[error("no audio packets")]
    NoPackets,
}

impl Audio {
    pub(crate) fn read_from_file<P: AsRef<Path>>(file: P) -> Result<Audio, ImportError> {
        read_from_file(file.as_ref())
    }
}

fn read_from_file(file: &Path) -> Result<Audio, ImportError> {
    let extension = file.extension().and_then(OsStr::to_str);

    let file = File::open(file)?;

    let stream = MediaSourceStream::new(Box::new(file), MediaSourceStreamOptions::default());

    let probe = get_probe();

    let mut hint = Hint::new();

    if let Some(extension) = extension {
        hint.with_extension(extension);
    }

    let mut format = probe
        .format(
            &hint,
            stream,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?
        .format;

    let no_tracks = || ImportError::Symphonia(SymphoniaError::DecodeError("no tracks"));

    let mut track = format.default_track().ok_or(no_tracks())?;

    let track_id = track.id;

    let codecs = get_codecs();
    let decoder_options = DecoderOptions { verify: true };

    let mut decoder = codecs.make(&track.codec_params, &decoder_options)?;

    let mut sample_rate = None;

    let mut left_channel = Vec::new();
    let mut right_channel = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::ResetRequired) => {
                track = format.default_track().ok_or(no_tracks())?;
                decoder = codecs.make(&track.codec_params, &decoder_options)?;
                continue;
            }
            Err(SymphoniaError::IoError(error)) if error.kind() == ErrorKind::UnexpectedEof => {
                break;
            }
            error => error?,
        };

        if packet.track_id() != track_id {
            continue;
        }

        let audio_ref = decoder.decode(&packet)?;

        let rate = sample::Rate::try_from(audio_ref.spec().rate)?;

        match sample_rate {
            None => sample_rate = Some(rate),
            Some(sample_rate) => {
                if sample_rate != rate {
                    return Err(ImportError::SampleRateInconsistency);
                }
            }
        }

        let mut audio_ref_32 = audio_ref.make_equivalent();
        audio_ref.convert::<f32>(&mut audio_ref_32);

        let audio_ref = audio_ref_32;

        let planes = audio_ref.planes();
        let planes = planes.planes();

        let empty: &[f32] = &[];

        let [left, right] = match planes {
            [] => [empty, empty],
            [mono] => [*mono, *mono],
            [left, right, ..] => [*left, *right],
        };

        for sample in left {
            left_channel.push(Sample::new(*sample));
        }

        for sample in right {
            right_channel.push(Sample::new(*sample));
        }
    }

    Ok(Audio {
        sample_rate: sample_rate.ok_or(ImportError::NoPackets)?,
        channels: [left_channel, right_channel],
    })
}
