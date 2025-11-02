//! Camouflage Core Library
//!
//! Core functionality for ultrasonic audio jamming.

mod signal;
mod jammer;
pub mod platform;
pub mod daemon;

pub use signal::{SignalConfig, SignalGenerator};
pub use jammer::{SpeakerJammer, SystemJammer};
pub use platform::SystemAudio;
pub use daemon::{DaemonConfig, is_running, save_pid, remove_pid, stop_daemon, get_status};
