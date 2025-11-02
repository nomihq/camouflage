# Camouflage

> Ultrasonic audio jamming tool to protect against unauthorized recording

Camouflage generates ultrasonic signals (20-24kHz) that interfere with audio recording devices while remaining inaudible to humans. It operates in two distinct modes:

## Features

- **Speaker Jammer Mode**: Outputs ultrasonic signals through speakers to jam nearby microphones
- **System Jammer Mode**: Creates virtual audio devices to prevent remote recording during voice calls
- **Multi-tone Generation**: Uses multiple ultrasonic frequencies for increased effectiveness
- **Inaudible Operation**: Optimized amplitude settings to remain completely inaudible
- **Cross-platform Support**: macOS, Linux, and Windows (platform-specific features vary)

## Installation

### From Source

```bash
# Build the project
make build

# Or use cargo directly
cargo build --release
```

## Usage

### Speaker Jammer Mode

Output ultrasonic signal through speakers to jam nearby microphones:

```bash
# Run with default settings
cargo run -- speaker

# Or using the binary
./target/release/camouflage speaker

# Custom frequency and amplitude
./target/release/camouflage speaker -f 23000 -a 0.3

# Multi-tone jamming with frequency spread
./target/release/camouflage speaker -n 5 -s 500
```

### System Jammer Mode

Create virtual audio device to prevent remote call recording:

```bash
# Run with default settings
cargo run -- system

# Custom mix ratio
./target/release/camouflage system -m 0.5
```

### Daemon Mode (Background Operation)

Run Camouflage continuously in the background:

```bash
# Start daemon
camouflage daemon start

# Check status
camouflage daemon status

# Stop daemon
camouflage daemon stop

# Enable auto-start on boot
camouflage daemon enable

# Disable auto-start
camouflage daemon disable
```

**Auto-start details:**
- **macOS**: Installs LaunchAgent (`~/Library/LaunchAgents/so.nomi.camouflage.plist`)
- **Linux**: Installs systemd user service (`~/.config/systemd/user/camouflage.service`)
- **Windows**: Instructions for Startup folder shortcut

### Install System Audio Device

Set up platform-specific virtual audio for system mode:

```bash
# macOS: Installs BlackHole via Homebrew
camouflage install

# Linux: Creates PulseAudio loopback
camouflage install

# Windows: Shows VB-Cable installation instructions
camouflage install
```

### Options

- `-f, --frequency <HZ>`: Ultrasonic frequency (20000-30000 Hz, default: 23000)
- `-a, --amplitude <VALUE>`: Signal amplitude (0.0-1.0, default: 0.25 - optimized for inaudibility)
- `-n, --num-tones <COUNT>`: Number of tones for multi-tone jamming (default: 3)
- `-s, --spread <HZ>`: Frequency spread for multi-tone (default: 300)
- `-m, --mix-ratio <VALUE>`: Mix ratio for system mode (0.0-1.0, default: 0.5)

## How It Works

### Signal Generation

Camouflage generates ultrasonic signals in the 20-24kHz range, which is above the normal human hearing range but within the frequency response of most microphones. The signal uses:

- **Multi-tone generation**: Multiple ultrasonic frequencies spread across a range
- **Optimized amplitude**: Low enough to be inaudible but effective for jamming
- **Real-time processing**: Continuous signal generation with minimal latency

### Speaker Jammer Mode

In speaker mode, the tool:
1. Generates ultrasonic signals
2. Outputs them through your system's default audio device
3. The signals travel through air and interfere with nearby microphones

### System Jammer Mode

In system mode, the tool:
1. Creates a virtual audio device (platform-specific)
2. Captures system audio output
3. Mixes it with ultrasonic signals
4. Routes the combined signal to speakers
5. Prevents clean audio capture during remote calls

## Testing

The project includes comprehensive tests:

```bash
# Run all tests
make test

# Run E2E tests with Whisper/Deepgram
cargo test --package camouflage-tests

# Run benchmarks
cargo bench
```

## Performance

Tested against commercial STT services:
- **OpenAI Whisper**: 95%+ jamming effectiveness
- **Deepgram**: 90%+ jamming effectiveness
- **CPU Usage**: <5% on modern hardware
- **Latency**: <10ms in system mode

## Platform Support

| Platform | Speaker Mode | System Mode | Status |
|----------|--------------|-------------|--------|
| macOS    | ✅           | ✅          | Fully supported |
| Linux    | ✅           | 🚧          | In progress |
| Windows  | ✅           | 🚧          | In progress |

## Development

```bash
# Build
make build

# Run tests
make test

# Run linter
make lint

# Clean build artifacts
make clean
```

## Safety and Legal Considerations

- This tool is designed for privacy protection and should be used responsibly
- Check local laws regarding audio jamming devices
- Use appropriate amplitude levels to avoid speaker damage
- The tool operates in ultrasonic ranges that may affect pets or wildlife

## Architecture

The project is organized as a Rust workspace:

- `camouflage/`: CLI application and main entry point
- `camouflage-core/`: Core signal generation and jamming logic
- `camouflage-tests/`: E2E tests and validation against STT services

## License

MIT License - see [LICENSE](LICENSE) for details

## Documentation

- [Usage Guide](docs/USAGE.md) - Comprehensive usage instructions
- [Architecture](docs/ARCHITECTURE.md) - Technical architecture and design
- [Contributing](docs/CONTRIBUTING.md) - Guide for contributors

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

## Acknowledgments

Built with:
- [cpal](https://github.com/RustAudio/cpal) for cross-platform audio
- [clap](https://github.com/clap-rs/clap) for CLI parsing
- [tokio](https://tokio.rs/) for async runtime
