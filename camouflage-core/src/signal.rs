//! Signal generation module for ultrasonic audio

use std::f32::consts::PI;

/// Configuration for ultrasonic signal generation
#[derive(Debug, Clone)]
pub struct SignalConfig {
    /// Base frequency in Hz (typically 20000-24000 for ultrasonic)
    pub frequency: f32,
    /// Sample rate in Hz (typically 48000)
    pub sample_rate: u32,
    /// Signal amplitude (0.0-1.0)
    pub amplitude: f32,
    /// Number of tones for multi-tone jamming
    pub num_tones: usize,
    /// Frequency spread between tones in Hz
    pub frequency_spread: f32,
}

impl Default for SignalConfig {
    fn default() -> Self {
        Self {
            frequency: 23000.0,
            sample_rate: 48000,
            amplitude: 0.25, // Optimized for inaudibility while maintaining effectiveness
            num_tones: 3,
            frequency_spread: 300.0,
        }
    }
}

/// Generator for ultrasonic signals
pub struct SignalGenerator {
    config: SignalConfig,
    phase: f32,
    tone_phases: Vec<f32>,
}

impl SignalGenerator {
    /// Create a new signal generator with the given configuration
    pub fn new(config: SignalConfig) -> Self {
        let tone_phases = vec![0.0; config.num_tones];
        Self {
            config,
            phase: 0.0,
            tone_phases,
        }
    }

    /// Generate the next sample
    pub fn next_sample(&mut self) -> f32 {
        if self.config.num_tones == 1 {
            // Single tone generation
            let sample = self.config.amplitude * (2.0 * PI * self.phase).sin();
            self.phase += self.config.frequency / self.config.sample_rate as f32;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
            }
            sample
        } else {
            // Multi-tone generation
            let mut sample = 0.0;
            let amplitude_per_tone = self.config.amplitude / self.config.num_tones as f32;

            for (i, phase) in self.tone_phases.iter_mut().enumerate() {
                let offset = (i as f32 - (self.config.num_tones as f32 - 1.0) / 2.0)
                    * self.config.frequency_spread;
                let freq = self.config.frequency + offset;

                sample += amplitude_per_tone * (2.0 * PI * *phase).sin();
                *phase += freq / self.config.sample_rate as f32;
                if *phase >= 1.0 {
                    *phase -= 1.0;
                }
            }
            sample
        }
    }

    /// Generate a buffer of samples
    pub fn generate_buffer(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            *sample = self.next_sample();
        }
    }

    /// Get the current configuration
    pub fn config(&self) -> &SignalConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: SignalConfig) {
        self.config = config;
        self.tone_phases = vec![0.0; self.config.num_tones];
        self.phase = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_generation() {
        let config = SignalConfig::default();
        let mut generator = SignalGenerator::new(config);

        let mut buffer = vec![0.0; 1000];
        generator.generate_buffer(&mut buffer);

        // Verify samples are within expected range
        for sample in &buffer {
            assert!(sample.abs() <= 1.0);
        }
    }

    #[test]
    fn test_multi_tone_generation() {
        let config = SignalConfig {
            num_tones: 5,
            ..Default::default()
        };
        let mut generator = SignalGenerator::new(config);

        let mut buffer = vec![0.0; 1000];
        generator.generate_buffer(&mut buffer);

        // Verify samples are within expected range
        for sample in &buffer {
            assert!(sample.abs() <= 1.0);
        }
    }
}
