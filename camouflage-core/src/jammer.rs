//! Jammer implementations for different modes

use crate::signal::{SignalConfig, SignalGenerator};
use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use std::sync::Arc;
use std::sync::Mutex;
use tracing::{debug, info};

/// Speaker jammer - outputs ultrasonic signal through speakers
///
/// This mode outputs ultrasonic signals directly to speakers, which then
/// interfere with nearby microphones through air conduction.
pub struct SpeakerJammer {
    generator: Arc<Mutex<SignalGenerator>>,
    stream: Option<Stream>,
    device: Device,
    config: StreamConfig,
}

impl SpeakerJammer {
    /// Create a new speaker jammer
    pub fn new(mut signal_config: SignalConfig) -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .context("No output device available")?;

        info!("Using audio device: {}", device.name()?);

        let config = device.default_output_config()?;
        info!("Default output config: {:?}", config);

        // Update signal config with actual sample rate
        signal_config.sample_rate = config.sample_rate().0;

        let generator = Arc::new(Mutex::new(SignalGenerator::new(signal_config)));

        Ok(Self {
            generator,
            stream: None,
            device,
            config: config.into(),
        })
    }

    /// Start jamming
    pub fn start(&mut self) -> Result<()> {
        let generator = Arc::clone(&self.generator);
        let channels = self.config.channels as usize;

        let stream = self.device.build_output_stream(
            &self.config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut gen = generator.lock().unwrap();
                for frame in data.chunks_mut(channels) {
                    let sample = gen.next_sample();
                    for channel in frame.iter_mut() {
                        *channel = sample;
                    }
                }
            },
            |err| {
                eprintln!("Audio stream error: {}", err);
            },
            None,
        )?;

        stream.play()?;
        self.stream = Some(stream);

        info!("Speaker jammer started");
        Ok(())
    }

    /// Stop jamming
    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
            info!("Speaker jammer stopped");
        }
    }
}

impl Drop for SpeakerJammer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// System jammer - creates virtual audio device for system-wide jamming
///
/// This mode is designed to prevent remote recording during voice calls by:
/// 1. Creating a virtual audio device (platform-specific implementation)
/// 2. Capturing system audio output
/// 3. Mixing it with ultrasonic jamming signal
/// 4. Routing the mixed signal to speakers
///
/// Current implementation uses SpeakerJammer as a fallback.
pub struct SystemJammer {
    speaker_jammer: SpeakerJammer,
    mix_ratio: f32,
}

impl SystemJammer {
    /// Create a new system jammer
    pub fn new(signal_config: SignalConfig, mix_ratio: f32) -> Result<Self> {
        debug!("Creating system jammer with mix ratio: {}", mix_ratio);

        // For now, system jammer uses the same implementation as speaker jammer
        // In a full implementation, this would:
        // 1. Create a virtual audio device (platform-specific)
        // 2. Capture system audio output
        // 3. Mix it with ultrasonic signal
        // 4. Route to speakers

        let speaker_jammer = SpeakerJammer::new(signal_config)?;

        Ok(Self {
            speaker_jammer,
            mix_ratio,
        })
    }

    /// Start jamming
    pub fn start(&mut self) -> Result<()> {
        info!(
            "Starting system jammer (mix ratio: {})",
            self.mix_ratio
        );
        self.speaker_jammer.start()
    }

    /// Stop jamming
    pub fn stop(&mut self) {
        self.speaker_jammer.stop();
    }
}

impl Drop for SystemJammer {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speaker_jammer_creation() {
        let config = SignalConfig::default();
        let result = SpeakerJammer::new(config);

        // This might fail in CI without audio devices
        if result.is_ok() {
            let mut jammer = result.unwrap();
            assert!(jammer.start().is_ok());
            jammer.stop();
        }
    }

    #[test]
    fn test_system_jammer_creation() {
        let config = SignalConfig::default();
        let result = SystemJammer::new(config, 0.5);

        // This might fail in CI without audio devices
        if result.is_ok() {
            let mut jammer = result.unwrap();
            assert!(jammer.start().is_ok());
            jammer.stop();
        }
    }
}
