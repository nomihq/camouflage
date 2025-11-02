use super::SystemAudio;
use anyhow::{Context, Result};
use std::process::Command;
use tracing::{info, warn};

/// Linux system audio implementation using PulseAudio/PipeWire
pub struct LinuxSystemAudio {
    sink_name: String,
}

impl Default for LinuxSystemAudio {
    fn default() -> Self {
        Self::new()
    }
}

impl LinuxSystemAudio {
    pub fn new() -> Self {
        Self {
            sink_name: "camouflage_sink".to_string(),
        }
    }

    fn check_pulseaudio(&self) -> bool {
        Command::new("pactl")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn check_pipewire(&self) -> bool {
        Command::new("pw-cli")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    fn create_pulseaudio_loopback(&self) -> Result<()> {
        info!("Creating PulseAudio null sink and loopback...");

        // Create null sink
        let output = Command::new("pactl")
            .args([
                "load-module",
                "module-null-sink",
                &format!("sink_name={}", self.sink_name),
                "sink_properties=device.description=Camouflage_Virtual_Output",
            ])
            .output()
            .context("Failed to create null sink")?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to create null sink: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        // Create loopback from null sink to default output
        let output = Command::new("pactl")
            .args([
                "load-module",
                "module-loopback",
                &format!("source={}.monitor", self.sink_name),
                "latency_msec=1",
            ])
            .output()
            .context("Failed to create loopback")?;

        if !output.status.success() {
            warn!(
                "Loopback creation warning: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        info!("✓ PulseAudio virtual device created");
        info!(
            "  Set '{}' as default output in your system settings",
            self.sink_name
        );

        Ok(())
    }
}

impl SystemAudio for LinuxSystemAudio {
    fn create_virtual_device(&self) -> Result<()> {
        if self.check_pulseaudio() {
            self.create_pulseaudio_loopback()?;
        } else if self.check_pipewire() {
            warn!("PipeWire detected - using PulseAudio compatibility layer");
            self.create_pulseaudio_loopback()?;
        } else {
            anyhow::bail!(
                "Neither PulseAudio nor PipeWire found. Please install one:\n\
                Ubuntu/Debian: sudo apt-get install pulseaudio\n\
                Fedora: sudo dnf install pulseaudio\n\
                Arch: sudo pacman -S pulseaudio"
            );
        }

        Ok(())
    }

    fn start_capture(&mut self) -> Result<()> {
        info!("System audio capture active on Linux");
        Ok(())
    }

    fn stop_capture(&mut self) {
        info!("Stopping Linux system audio capture");

        // Remove null sink and loopback
        let _ = Command::new("pactl")
            .args(["unload-module", "module-null-sink"])
            .output();

        let _ = Command::new("pactl")
            .args(["unload-module", "module-loopback"])
            .output();
    }

    fn virtual_device_exists(&self) -> bool {
        Command::new("pactl")
            .args(["list", "sinks", "short"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains(&self.sink_name))
            .unwrap_or(false)
    }

    fn remove_virtual_device(&self) -> Result<()> {
        // Remove null sink and loopback
        let _ = Command::new("pactl")
            .args(["unload-module", "module-null-sink"])
            .output();

        let _ = Command::new("pactl")
            .args(["unload-module", "module-loopback"])
            .output();

        info!("✓ Virtual audio device removed");
        Ok(())
    }
}
