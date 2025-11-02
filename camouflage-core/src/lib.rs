//! Camouflage Core Library
//!
//! Core functionality for ultrasonic audio jamming.

pub mod daemon;
mod jammer;
pub mod platform;
mod signal;

pub use daemon::{get_status, is_running, remove_pid, save_pid, stop_daemon, DaemonConfig};
pub use jammer::{SpeakerJammer, SystemJammer};
pub use platform::SystemAudio;
pub use signal::{SignalConfig, SignalGenerator};
