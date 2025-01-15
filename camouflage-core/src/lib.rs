//! Camouflage Core Library
//!
//! Core functionality for ultrasonic audio jamming.

mod signal;
mod jammer;

pub use signal::{SignalConfig, SignalGenerator};
pub use jammer::{SpeakerJammer, SystemJammer};
