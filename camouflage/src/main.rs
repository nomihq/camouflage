use camouflage_core::{get_status, is_running, save_pid, stop_daemon};
use camouflage_core::{SignalConfig, SpeakerJammer, SystemJammer};
use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::path::PathBuf;
use tracing::info;

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

    /// Run in daemon mode (background process)
    Daemon {
        #[command(subcommand)]
        command: DaemonCommand,
    },

    /// Install system mode audio device for your platform
    Install,
}

#[derive(Subcommand)]
enum DaemonCommand {
    /// Start daemon in speaker mode
    Start {
        /// Daemon mode (speaker or system)
        #[arg(short, long, default_value = "speaker")]
        mode: String,
    },

    /// Stop running daemon
    Stop,

    /// Check daemon status
    Status,

    /// Enable auto-start on boot
    Enable,

    /// Disable auto-start on boot
    Disable,
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
        eprintln!(
            "⚠️  Warning: High amplitude ({}) may cause audible distortion.",
            config.amplitude
        );
        eprintln!(
            "   Recommended: Use amplitude 0.25 or lower for completely inaudible operation."
        );
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
        let needed_frequency =
            20000.0 + (config.num_tones as f32 / 2.0) * config.frequency_spread + 500.0;
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
        Mode::Daemon { command } => run_daemon_command(command, config)?,
        Mode::Install => run_install()?,
    }

    Ok(())
}

fn run_daemon_command(command: DaemonCommand, config: SignalConfig) -> anyhow::Result<()> {
    match command {
        DaemonCommand::Start { mode } => {
            if is_running() {
                println!("❌ Daemon is already running");
                println!("   Use 'camouflage daemon stop' to stop it first");
                return Ok(());
            }

            println!("🚀 Starting daemon in {} mode...", mode);

            // Daemonize the process
            #[cfg(unix)]
            {
                unsafe {
                    let pid = libc::fork();
                    if pid < 0 {
                        anyhow::bail!("Failed to fork process");
                    }
                    if pid > 0 {
                        // Parent process
                        println!("✓ Daemon started (PID: {})", pid);
                        return Ok(());
                    }
                    // Child process continues
                    libc::setsid();
                }
            }

            // Save PID
            save_pid()?;

            // Run the jammer
            if mode == "speaker" {
                let mut jammer = SpeakerJammer::new(config)?;
                jammer.start()?;

                // Run forever
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(60));
                }
            } else {
                let mut jammer = SystemJammer::new(config, 0.5)?;
                jammer.start()?;

                // Run forever
                loop {
                    std::thread::sleep(std::time::Duration::from_secs(60));
                }
            }
        }

        DaemonCommand::Stop => {
            println!("🛑 Stopping daemon...");
            stop_daemon()?;
            println!("✓ Daemon stopped");
        }

        DaemonCommand::Status => {
            let status = get_status();
            println!("Daemon status: {}", status);
        }

        DaemonCommand::Enable => {
            println!("⚙️  Enabling auto-start...");
            install_autostart()?;
            println!("✓ Auto-start enabled");
        }

        DaemonCommand::Disable => {
            println!("⚙️  Disabling auto-start...");
            uninstall_autostart()?;
            println!("✓ Auto-start disabled");
        }
    }

    Ok(())
}

fn run_install() -> anyhow::Result<()> {
    println!("🔧 Installing system audio device...\n");

    #[cfg(target_os = "macos")]
    {
        use camouflage_core::platform;
        let audio = platform::get_system_audio();
        audio.create_virtual_device()?;
    }

    #[cfg(target_os = "linux")]
    {
        use camouflage_core::platform;
        let audio = platform::get_system_audio();
        audio.create_virtual_device()?;
    }

    #[cfg(target_os = "windows")]
    {
        use camouflage_core::platform;
        let audio = platform::get_system_audio();
        audio.create_virtual_device()?;
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn install_autostart() -> anyhow::Result<()> {
    use std::fs;
    use std::path::PathBuf;

    let home = std::env::var("HOME")?;
    let launch_agents_dir = PathBuf::from(&home).join("Library/LaunchAgents");
    fs::create_dir_all(&launch_agents_dir)?;

    let plist_path = launch_agents_dir.join("so.nomi.camouflage.plist");
    let exe_path = std::env::current_exe()?;

    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>so.nomi.camouflage</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>daemon</string>
        <string>start</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/camouflage.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/camouflage.err</string>
</dict>
</plist>"#,
        exe_path.display()
    );

    fs::write(&plist_path, plist_content)?;

    // Load the agent
    std::process::Command::new("launchctl")
        .args(["load", plist_path.to_str().unwrap()])
        .output()?;

    println!("✓ LaunchAgent installed: {}", plist_path.display());
    Ok(())
}

#[cfg(target_os = "linux")]
fn install_autostart() -> anyhow::Result<()> {
    use std::fs;
    use std::path::PathBuf;

    let home = std::env::var("HOME")?;
    let systemd_dir = PathBuf::from(&home).join(".config/systemd/user");
    fs::create_dir_all(&systemd_dir)?;

    let service_path = systemd_dir.join("camouflage.service");
    let exe_path = std::env::current_exe()?;

    let service_content = format!(
        r#"[Unit]
Description=Camouflage Audio Jammer
After=sound.target

[Service]
Type=simple
ExecStart={} daemon start
Restart=always
RestartSec=5

[Install]
WantedBy=default.target"#,
        exe_path.display()
    );

    fs::write(&service_path, service_content)?;

    // Enable and start service
    std::process::Command::new("systemctl")
        .args(["--user", "enable", "camouflage.service"])
        .output()?;

    std::process::Command::new("systemctl")
        .args(["--user", "start", "camouflage.service"])
        .output()?;

    println!("✓ systemd service installed: {}", service_path.display());
    Ok(())
}

#[cfg(target_os = "windows")]
fn install_autostart() -> anyhow::Result<()> {
    println!("⚠️  Windows auto-start:");
    println!("   1. Press Win+R");
    println!("   2. Type: shell:startup");
    println!("   3. Create shortcut to camouflage.exe with arguments: daemon start");
    println!("   4. Place shortcut in the Startup folder");
    Ok(())
}

#[cfg(target_os = "macos")]
fn uninstall_autostart() -> anyhow::Result<()> {
    let home = std::env::var("HOME")?;
    let plist_path = PathBuf::from(&home).join("Library/LaunchAgents/so.nomi.camouflage.plist");

    if plist_path.exists() {
        std::process::Command::new("launchctl")
            .args(["unload", plist_path.to_str().unwrap()])
            .output()?;

        std::fs::remove_file(&plist_path)?;
        println!("✓ LaunchAgent removed");
    } else {
        println!("Auto-start was not enabled");
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn uninstall_autostart() -> anyhow::Result<()> {
    std::process::Command::new("systemctl")
        .args(["--user", "stop", "camouflage.service"])
        .output()?;

    std::process::Command::new("systemctl")
        .args(["--user", "disable", "camouflage.service"])
        .output()?;

    let home = std::env::var("HOME")?;
    let service_path = PathBuf::from(&home).join(".config/systemd/user/camouflage.service");

    if service_path.exists() {
        std::fs::remove_file(&service_path)?;
        println!("✓ systemd service removed");
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn uninstall_autostart() -> anyhow::Result<()> {
    println!("Remove the shortcut from shell:startup folder");
    Ok(())
}

fn run_speaker_jammer(config: SignalConfig) -> anyhow::Result<()> {
    info!("=== Speaker Jammer Mode ===");
    info!("Frequency: {} Hz", config.frequency);
    info!(
        "Amplitude: {} (optimized for inaudibility)",
        config.amplitude
    );
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
