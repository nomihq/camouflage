use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

/// Daemon configuration and control
pub struct DaemonConfig {
    pub mode: String,
    pub amplitude: f32,
    pub frequency: f32,
    pub auto_start: bool,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            mode: "speaker".to_string(),
            amplitude: 0.25,
            frequency: 23000.0,
            auto_start: true,
        }
    }
}

/// Get the daemon configuration directory
pub fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Failed to get config directory")?
        .join("camouflage");

    fs::create_dir_all(&config_dir)
        .context("Failed to create config directory")?;

    Ok(config_dir)
}

/// Get the PID file path
pub fn get_pid_file() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("camouflage.pid"))
}

/// Check if daemon is running
pub fn is_running() -> bool {
    if let Ok(pid_file) = get_pid_file() {
        if let Ok(pid_str) = fs::read_to_string(&pid_file) {
            if let Ok(pid) = pid_str.trim().parse::<i32>() {
                // Check if process exists
                #[cfg(unix)]
                {
                    use std::process::Command;
                    return Command::new("kill")
                        .args(["-0", &pid.to_string()])
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false);
                }

                #[cfg(windows)]
                {
                    use std::process::Command;
                    return Command::new("tasklist")
                        .args(["/FI", &format!("PID eq {}", pid)])
                        .output()
                        .map(|o| String::from_utf8_lossy(&o.stdout).contains(&pid.to_string()))
                        .unwrap_or(false);
                }
            }
        }
    }
    false
}

/// Save daemon PID
pub fn save_pid() -> Result<()> {
    let pid_file = get_pid_file()?;
    let pid = std::process::id();
    fs::write(&pid_file, pid.to_string())
        .context("Failed to write PID file")?;
    info!("Saved daemon PID: {}", pid);
    Ok(())
}

/// Remove daemon PID file
pub fn remove_pid() -> Result<()> {
    let pid_file = get_pid_file()?;
    if pid_file.exists() {
        fs::remove_file(&pid_file)
            .context("Failed to remove PID file")?;
    }
    Ok(())
}

/// Stop running daemon
pub fn stop_daemon() -> Result<()> {
    let pid_file = get_pid_file()?;

    if !pid_file.exists() {
        info!("No daemon running");
        return Ok(());
    }

    let pid_str = fs::read_to_string(&pid_file)
        .context("Failed to read PID file")?;
    let pid = pid_str.trim().parse::<i32>()
        .context("Invalid PID in file")?;

    info!("Stopping daemon (PID: {})...", pid);

    #[cfg(unix)]
    {
        use std::process::Command;
        let output = Command::new("kill")
            .arg(pid.to_string())
            .output()
            .context("Failed to kill process")?;

        if !output.status.success() {
            warn!("Failed to stop daemon: {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    #[cfg(windows)]
    {
        use std::process::Command;
        let output = Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/F"])
            .output()
            .context("Failed to kill process")?;

        if !output.status.success() {
            warn!("Failed to stop daemon: {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    remove_pid()?;
    info!("✓ Daemon stopped");
    Ok(())
}

/// Get daemon status
pub fn get_status() -> String {
    if is_running() {
        if let Ok(pid_file) = get_pid_file() {
            if let Ok(pid_str) = fs::read_to_string(&pid_file) {
                return format!("Running (PID: {})", pid_str.trim());
            }
        }
        "Running".to_string()
    } else {
        "Stopped".to_string()
    }
}
