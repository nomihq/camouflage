# Architecture

This document describes the architecture and design of Camouflage.

## Overview

Camouflage is a Rust-based ultrasonic audio jamming tool designed to protect against unauthorized audio recording. The project is organized as a workspace with three main components:

```
camouflage/
├── camouflage/          # CLI application
├── camouflage-core/     # Core signal generation and jamming logic
└── camouflage-tests/    # E2E tests and validation
```

## Core Components

### Signal Generation (camouflage-core)

The signal generation module is responsible for creating ultrasonic signals:

- **SignalGenerator**: Core signal generation engine
  - Supports single and multi-tone generation
  - Configurable frequency, amplitude, and tone spread
  - Real-time phase tracking for smooth waveforms

- **SignalConfig**: Configuration for signal parameters
  - Base frequency (typically 20-24 kHz)
  - Sample rate (44.1 kHz, 48 kHz, etc.)
  - Amplitude (0.0-1.0, recommended 0.3 or lower)
  - Number of tones and frequency spread

### Jamming Modes

#### Speaker Jammer Mode

Outputs ultrasonic signals directly through speakers to jam nearby microphones:

1. Initialize audio output device using cpal
2. Generate continuous ultrasonic signal
3. Output to default audio device
4. Signal propagates through air to interfere with microphones

**Use Cases:**
- Protecting conversations from hidden recording devices
- Preventing unauthorized recording in meetings
- Testing microphone susceptibility

#### System Jammer Mode

Designed to prevent remote recording during voice calls:

1. Create virtual audio device (platform-specific)
2. Capture system audio output
3. Mix with ultrasonic signal
4. Route mixed signal to speakers

**Current Status:** Fallback implementation using SpeakerJammer

**Planned Implementation:**
- macOS: Core Audio AudioServerPlugin
- Windows: WASAPI loopback device
- Linux: PulseAudio/PipeWire null sink

### CLI Application (camouflage)

The CLI provides a user-friendly interface:

- **Clap-based CLI**: Subcommand-based interface
- **Configuration**: Frequency, amplitude, tone settings
- **Real-time feedback**: Status updates and warnings
- **Safety checks**: Validates ultrasonic range and amplitude

## Signal Processing

### Multi-Tone Generation

Multi-tone jamming increases effectiveness by spreading energy across frequencies:

```
For num_tones = 3, spread = 300 Hz:
- Tone 1: base_freq - 300 Hz
- Tone 2: base_freq
- Tone 3: base_freq + 300 Hz
```

Each tone is generated independently with phase tracking to prevent aliasing.

### Amplitude Management

Amplitude is carefully controlled:
- Default: 0.3 (30% of maximum)
- Warning threshold: 0.5 (50%)
- Maximum: 1.0 (not recommended)

Lower amplitudes reduce audibility while maintaining jamming effectiveness.

## Testing Strategy

### Unit Tests

- Signal generation correctness
- Configuration validation
- Audio device initialization

### Integration Tests

- End-to-end jammer operation
- Multi-platform audio output
- Error handling

### E2E Validation

Tests against commercial STT services:

1. **OpenAI Whisper**: State-of-the-art ASR
2. **Deepgram**: Commercial STT service

Test procedure:
1. Generate clean audio with OpenAI TTS
2. Generate ultrasonic-jammed audio
3. Transcribe with STT services
4. Validate jamming effectiveness (word count, confidence)

### Performance Benchmarks

Criterion-based benchmarks measure:
- Signal generation throughput
- Impact of tone count on performance
- Sample rate effects
- Buffer size optimization

## Platform Support

### macOS
- Full speaker jammer support via cpal + Core Audio
- System jammer planned with AudioServerPlugin

### Linux
- Speaker jammer via cpal + ALSA/PulseAudio
- System jammer planned with PulseAudio null sink

### Windows
- Speaker jammer via cpal + WASAPI
- System jammer planned with WASAPI loopback

## Dependencies

- **cpal**: Cross-platform audio I/O
- **clap**: CLI parsing
- **anyhow**: Error handling
- **tokio**: Async runtime (for tests)
- **hound**: WAV file I/O (for tests)
- **reqwest**: HTTP client (for E2E tests)

## Security Considerations

1. **Amplitude Limits**: Prevent speaker damage with warnings
2. **Frequency Validation**: Ensure signals stay ultrasonic
3. **Graceful Shutdown**: Clean audio device cleanup
4. **API Key Security**: Tests require environment variables

## Performance Characteristics

Based on benchmarks:

- **CPU Usage**: <5% on modern hardware (1-3 tones)
- **Latency**: <10ms buffer processing
- **Memory**: ~2MB base + audio buffers
- **Throughput**: 48000 samples/sec @ 48kHz

## Future Enhancements

1. **Virtual Audio Devices**: Complete system jammer implementation
2. **Adaptive Jamming**: Automatically adjust parameters
3. **Frequency Hopping**: Dynamic frequency changes
4. **Remote Control**: Web/app interface
5. **Detection Avoidance**: Randomization and noise shaping
