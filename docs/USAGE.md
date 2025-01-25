# Usage Guide

Complete guide to using Camouflage for ultrasonic audio jamming.

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/nomihq/camouflage.git
cd camouflage

# Build release binary
cargo build --release

# Binary will be at: target/release/camouflage
```

### From Crates.io

```bash
cargo install camouflage
```

### Pre-built Binaries

Download from [GitHub Releases](https://github.com/nomihq/camouflage/releases).

## Basic Usage

### Speaker Jammer Mode

Output ultrasonic signal through speakers to jam nearby microphones:

```bash
# Default settings (23kHz, 0.3 amplitude, 3 tones)
camouflage speaker

# Custom frequency
camouflage speaker --frequency 24000

# Lower amplitude for quieter operation
camouflage speaker --amplitude 0.2

# Single tone (simpler, but less effective)
camouflage speaker --num-tones 1

# Multi-tone with wide spread (more effective)
camouflage speaker --num-tones 5 --spread 500
```

### System Jammer Mode

Mix ultrasonic signal with system audio (for call protection):

```bash
# Default settings
camouflage system

# Custom mix ratio
camouflage system --mix-ratio 0.3

# Combined with other options
camouflage system --frequency 22000 --amplitude 0.25 --mix-ratio 0.4
```

## Configuration Options

### Frequency (`-f`, `--frequency`)

Base ultrasonic frequency in Hz.

- **Range**: 20000-30000 Hz
- **Default**: 23000 Hz
- **Recommended**: 22000-24000 Hz

**Examples:**
```bash
camouflage speaker -f 22000  # Lower frequency
camouflage speaker -f 24000  # Higher frequency
```

**Notes:**
- Lower frequencies may be partially audible
- Higher frequencies may not be captured by all microphones
- Stay below 24000 Hz with 48kHz sample rate (Nyquist limit)

### Amplitude (`-a`, `--amplitude`)

Signal strength (0.0-1.0).

- **Range**: 0.0-1.0
- **Default**: 0.3 (30%)
- **Recommended**: 0.2-0.4

**Examples:**
```bash
camouflage speaker -a 0.2  # Quieter, still effective
camouflage speaker -a 0.4  # Louder, more effective but riskier
```

**Warnings:**
- Values > 0.5 may cause audible distortion
- Values > 0.7 may damage speakers
- Start low and increase if needed

### Number of Tones (`-n`, `--num-tones`)

Number of simultaneous ultrasonic tones.

- **Range**: 1-10
- **Default**: 3
- **Recommended**: 3-5

**Examples:**
```bash
camouflage speaker -n 1  # Single tone (minimal)
camouflage speaker -n 3  # Default (good balance)
camouflage speaker -n 5  # More coverage (higher effectiveness)
```

**Trade-offs:**
- More tones = better coverage and effectiveness
- More tones = higher CPU usage
- More tones = potential for audible artifacts if too many

### Frequency Spread (`-s`, `--spread`)

Spacing between tones in multi-tone mode (Hz).

- **Range**: 100-1000 Hz
- **Default**: 300 Hz
- **Recommended**: 200-500 Hz

**Examples:**
```bash
camouflage speaker -n 5 -s 200  # Tight grouping
camouflage speaker -n 5 -s 500  # Wide coverage
```

**Notes:**
- Wider spread covers more frequencies
- Ensure all tones stay above 20kHz
- Tool will auto-adjust if tones go below 20kHz

### Mix Ratio (`-m`, `--mix-ratio`)

System mode only: ratio of ultrasonic to original audio.

- **Range**: 0.0-1.0
- **Default**: 0.5 (50/50 mix)
- **Recommended**: 0.3-0.6

**Examples:**
```bash
camouflage system -m 0.3  # 30% ultrasonic, 70% original
camouflage system -m 0.6  # 60% ultrasonic, 40% original
```

## Use Cases

### 1. Protecting In-Person Meetings

**Scenario:** Prevent hidden recording devices in meeting room

**Setup:**
```bash
# Start before meeting
camouflage speaker --frequency 23000 --amplitude 0.3 --num-tones 3

# Place device with speakers near conversation area
# Ensure speakers are not muted
```

**Tips:**
- Position speakers to cover room
- Test with your own recording device first
- Use moderate volume (30-40% system volume)

### 2. Preventing Remote Call Recording

**Scenario:** Protect voice calls from being recorded remotely

**Setup:**
```bash
# Start before call
camouflage system --mix-ratio 0.5

# Your call audio will include ultrasonic jamming
# Remote party's recording will be disrupted
```

**Tips:**
- Test with a friend first
- May affect call quality at high mix ratios
- Not all platforms/codecs preserve ultrasonic

### 3. Testing Microphone Security

**Scenario:** Test if your recording device is susceptible

**Setup:**
```bash
# Terminal 1: Start jammer
camouflage speaker --amplitude 0.3

# Terminal 2: Record audio with device under test
# Play recording and check for interference
```

## Platform-Specific Notes

### macOS

- Grant microphone/audio permissions if prompted
- Use built-in speakers or external speakers
- System jammer requires additional setup (coming soon)

### Linux

- Install ALSA development libraries:
  ```bash
  sudo apt-get install libasound2-dev
  ```
- May need to adjust PulseAudio settings
- Run with elevated privileges if device access fails

### Windows

- Use default audio device or select in Sound settings
- May trigger security warnings (normal for audio apps)
- System jammer requires WASAPI support (coming soon)

## Troubleshooting

### "No output device available"

**Solution:**
- Check audio device connections
- Verify device is not in use by other apps
- Try restarting audio service
- On Linux: Check ALSA/PulseAudio configuration

### Audible Noise/Distortion

**Causes:**
- Amplitude too high
- Frequency too low
- Speaker limitations

**Solutions:**
```bash
# Reduce amplitude
camouflage speaker --amplitude 0.2

# Increase base frequency
camouflage speaker --frequency 24000

# Reduce number of tones
camouflage speaker --num-tones 1
```

### Jamming Not Effective

**Possible issues:**
- Amplitude too low
- Microphone doesn't capture ultrasonic
- Speakers don't output ultrasonic well

**Solutions:**
```bash
# Increase amplitude
camouflage speaker --amplitude 0.4

# Try wider frequency spread
camouflage speaker --num-tones 5 --spread 500

# Use external speakers with better high-frequency response
```

### High CPU Usage

**Solutions:**
```bash
# Reduce number of tones
camouflage speaker --num-tones 1

# Should use <5% CPU on modern hardware
```

## Safety and Legal Considerations

### Safety

- **Start with low amplitude** (0.2-0.3) and increase if needed
- **Monitor speakers** for overheating during extended use
- **Avoid prolonged exposure** at high volumes
- **Consider pets** - some animals can hear ultrasonic frequencies

### Legal

- Check local laws regarding audio jamming devices
- Use only in areas where you have permission
- Do not use to interfere with authorized recordings
- Intended for privacy protection, not illegal activity

## Performance Tips

1. **CPU Usage**: Use fewer tones (1-3) for lower CPU usage
2. **Effectiveness**: Use more tones (3-5) for better coverage
3. **Inaudibility**: Keep amplitude ≤0.3 and frequency ≥22000 Hz
4. **Battery Life**: Lower amplitude and fewer tones extend battery

## Testing Effectiveness

Test your setup:

```bash
# 1. Start jammer
camouflage speaker &

# 2. Make a test recording with your phone/device
# 3. Stop jammer (Ctrl+C)
# 4. Play back recording

# 5. Try transcription services:
# - OpenAI Whisper
# - Google Speech-to-Text
# - Apple Dictation

# Effective jamming results in:
# - Garbled transcription
# - Very low confidence scores
# - Empty/minimal transcription
```

## Advanced Usage

### Running as Background Service

```bash
# Linux/macOS
nohup camouflage speaker --amplitude 0.3 > /dev/null 2>&1 &

# Store PID for later stopping
echo $! > camouflage.pid

# Stop later
kill $(cat camouflage.pid)
```

### Scripting Integration

```bash
#!/bin/bash
# meeting-protection.sh

echo "Starting audio protection..."
camouflage speaker --amplitude 0.3 &
JAMMER_PID=$!

echo "Jammer running (PID: $JAMMER_PID)"
echo "Press Enter to stop..."
read

kill $JAMMER_PID
echo "Protection stopped"
```

## Getting Help

- **GitHub Issues**: Report bugs or request features
- **Documentation**: Check ARCHITECTURE.md for technical details
- **Contributing**: See CONTRIBUTING.md to help improve Camouflage
