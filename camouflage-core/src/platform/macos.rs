use super::SystemAudio;
use anyhow::{Context, Result};
use std::process::Command;
use tracing::{info, warn};

/// macOS system audio implementation using BlackHole
#[allow(dead_code)]
pub struct MacOSSystemAudio {
    device_name: String,
}

impl Default for MacOSSystemAudio {
    fn default() -> Self {
        Self::new()
    }
}

impl MacOSSystemAudio {
    pub fn new() -> Self {
        Self {
            device_name: "BlackHole 2ch".to_string(),
        }
    }

    fn check_blackhole_installed(&self) -> bool {
        // Check if BlackHole is in the audio devices
        Command::new("system_profiler")
            .args(["SPAudioDataType"])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).contains("BlackHole"))
            .unwrap_or(false)
    }

    fn install_blackhole(&self) -> Result<()> {
        info!("BlackHole not found. Installing via Homebrew...");

        // Check if Homebrew is installed
        let brew_check = Command::new("which")
            .arg("brew")
            .output()
            .context("Failed to check for Homebrew")?;

        if !brew_check.status.success() {
            anyhow::bail!(
                "Homebrew not installed. Please install from https://brew.sh/\n\
                Or manually install BlackHole from https://existential.audio/blackhole/"
            );
        }

        // Install BlackHole via Homebrew
        let output = Command::new("brew")
            .args(["install", "blackhole-2ch"])
            .output()
            .context("Failed to install BlackHole")?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to install BlackHole: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        info!("✓ BlackHole installed successfully");
        Ok(())
    }

    fn create_multi_output_device(&self) -> Result<()> {
        info!("Creating Multi-Output Device with BlackHole...");

        // Note: Automated multi-output device creation requires GUI automation
        // For now, we provide instructions for manual setup

        warn!("⚠️  Manual setup required:");
        warn!("   1. Open Audio MIDI Setup (/Applications/Utilities/)");
        warn!("   2. Click '+' button → Create Multi-Output Device");
        warn!("   3. Check both 'BlackHole 2ch' and your speakers");
        warn!("   4. Set this as default output in System Preferences");
        warn!("");
        warn!("   Or use the automated installer:");
        warn!("   cargo run -- install-macos-audio");

        Ok(())
    }
}

impl SystemAudio for MacOSSystemAudio {
    fn create_virtual_device(&self) -> Result<()> {
        if !self.check_blackhole_installed() {
            self.install_blackhole()?;
        }

        self.create_multi_output_device()?;
        Ok(())
    }

    fn start_capture(&mut self) -> Result<()> {
        info!("Starting system audio capture on macOS");
        // In a full implementation, this would:
        // 1. Create audio tap on BlackHole
        // 2. Set up audio routing
        // 3. Start mixing thread

        warn!("System mode on macOS requires manual Multi-Output Device setup");
        warn!("Run: cargo run -- install-macos-audio");
        Ok(())
    }

    fn stop_capture(&mut self) {
        info!("Stopping system audio capture");
    }

    fn virtual_device_exists(&self) -> bool {
        self.check_blackhole_installed()
    }

    fn remove_virtual_device(&self) -> Result<()> {
        info!("To remove BlackHole: brew uninstall blackhole-2ch");
        Ok(())
    }
}
