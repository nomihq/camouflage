use anyhow::{Context, Result};
use camouflage_core::{SignalConfig, SignalGenerator};
use hound::{WavSpec, WavWriter};
use std::path::Path;

/// Generate a pure ultrasonic audio file for testing
pub fn generate_pure_ultrasonic(
    output_path: &Path,
    duration_secs: f32,
    config: &SignalConfig,
) -> Result<()> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: config.sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(output_path, spec).context("Failed to create WAV writer")?;

    let mut generator = SignalGenerator::new(config.clone());
    let num_samples = (config.sample_rate as f32 * duration_secs) as usize;

    for _ in 0..num_samples {
        let sample = generator.next_sample();
        let sample_i16 = (sample * i16::MAX as f32) as i16;
        writer.write_sample(sample_i16)?;
    }

    writer.finalize()?;
    Ok(())
}

/// Mix audio with ultrasonic signal
pub fn mix_audio_with_ultrasonic(
    input_path: &Path,
    output_path: &Path,
    config: &SignalConfig,
    mix_ratio: f32,
) -> Result<()> {
    let mut reader = hound::WavReader::open(input_path)?;
    let spec = reader.spec();

    let mut writer = WavWriter::create(output_path, spec)?;
    let mut generator = SignalGenerator::new(config.clone());

    let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();

    for sample in samples {
        let original = sample as f32 / i16::MAX as f32;
        let ultrasonic = generator.next_sample();
        let mixed = original * (1.0 - mix_ratio) + ultrasonic * mix_ratio;
        let mixed_i16 = (mixed * i16::MAX as f32) as i16;
        writer.write_sample(mixed_i16)?;
    }

    writer.finalize()?;
    Ok(())
}
