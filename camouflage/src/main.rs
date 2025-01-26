use camouflage_core::{SignalConfig, SpeakerJammer, SystemJammer};
use clap::{Parser, Subcommand};
use std::io::{self, Write};
use tracing::info;
use tracing_subscriber;

#[derive(Parser)]
#[command(name = "camouflage")]
#[command(about = "Ultrasonic audio jamming tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    mode: Mode,

    /// Ultrasonic frequency in Hz (20000-30000)
    #[arg(short, long, default_value = "23000")]
    frequency: f32,

    /// Signal amplitude (0.0-1.0) - Optimized default for complete inaudibility
    #[arg(short, long, default_value = "0.25")]
    amplitude: f32,

    /// Number of tones for multi-tone jamming
    #[arg(short, long, default_value = "3")]
    num_tones: usize,

    /// Frequency spread for multi-tone in Hz
    #[arg(short, long, default_value = "300")]
    spread: f32,
}

#[derive(Subcommand)]
enum Mode {
    /// Output ultrasonic signal through speakers to jam nearby microphones
    Speaker,

    /// Create virtual audio device to prevent remote call recording
    System {
        /// Mix ratio of ultrasonic signal (0.0-1.0)
        #[arg(short, long, default_value = "0.5")]
        mix_ratio: f32,
    },
}

fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let cli = Cli::parse();

    // Build signal configuration
    let mut config = SignalConfig {
        frequency: cli.frequency,
        sample_rate: 48000, // Will be updated by audio device
        amplitude: cli.amplitude,
        num_tones: cli.num_tones,
        frequency_spread: cli.spread,
    };

    // Warn if amplitude is too high (can cause audible distortion)
    if config.amplitude > 0.4 {
        eprintln!("⚠️  Warning: High amplitude ({}) may cause audible distortion.", config.amplitude);
        eprintln!("   Recommended: Use amplitude 0.25 or lower for completely inaudible operation.");
        eprintln!("   The signal remains highly effective at lower amplitudes.\n");
    }

    // Validate that all tones will be in ultrasonic range
    let min_freq = config.frequency - (config.num_tones as f32 / 2.0) * config.frequency_spread;
    let max_freq = config.frequency + (config.num_tones as f32 / 2.0) * config.frequency_spread;

    if min_freq < 20000.0 {
        eprintln!("⚠️  Warning: Some tones below 20kHz (audible range)!");
        eprintln!("   Lowest tone: {:.0} Hz", min_freq);
        eprintln!("   Adjusting to keep all tones above 20kHz...\n");

        // Adjust configuration to keep all tones ultrasonic
        let needed_frequency = 20000.0 + (config.num_tones as f32 / 2.0) * config.frequency_spread + 500.0;
        config.frequency = needed_frequency;
    }

    if max_freq > 24000.0 && config.sample_rate == 48000 {
        eprintln!("⚠️  Warning: Some tones near Nyquist limit (may alias)!");
        eprintln!("   Highest tone: {:.0} Hz", max_freq);
        eprintln!("   Consider reducing frequency spread.\n");
    }

    match cli.mode {
        Mode::Speaker => run_speaker_jammer(config)?,
        Mode::System { mix_ratio } => run_system_jammer(config, mix_ratio)?,
    }

    Ok(())
}

fn run_speaker_jammer(config: SignalConfig) -> anyhow::Result<()> {
    info!("=== Speaker Jammer Mode ===");
    info!("Frequency: {} Hz", config.frequency);
    info!("Amplitude: {} (optimized for inaudibility)", config.amplitude);
    info!("Number of tones: {}", config.num_tones);

    let mut jammer = SpeakerJammer::new(config)?;
    jammer.start()?;

    println!("\n✓ Speaker jammer is now active!");
    println!("  Ultrasonic signal is being transmitted through your speakers.");
    println!("  This will interfere with nearby microphones.");
    println!("\nPress Enter to stop...");

    wait_for_enter();

    jammer.stop();
    println!("Jammer stopped.");

    Ok(())
}

fn run_system_jammer(config: SignalConfig, mix_ratio: f32) -> anyhow::Result<()> {
    info!("=== System Jammer Mode ===");
    info!("Frequency: {} Hz", config.frequency);
    info!("Amplitude: {}", config.amplitude);
    info!("Number of tones: {}", config.num_tones);
    info!("Mix ratio: {}", mix_ratio);

    let mut jammer = SystemJammer::new(config, mix_ratio)?;
    jammer.start()?;

    println!("\n✓ System jammer is now active!");
    println!("  Ultrasonic signal is being mixed with system audio.");
    println!("  This will interfere with remote call recording.");
    println!(
        "  Note: Full virtual audio device support requires platform-specific implementation."
    );
    println!("\nPress Enter to stop...");

    wait_for_enter();

    jammer.stop();
    println!("Jammer stopped.");

    Ok(())
}

fn wait_for_enter() {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
}
