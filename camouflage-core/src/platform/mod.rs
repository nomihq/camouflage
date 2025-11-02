pub mod macos;
pub mod linux;
pub mod windows;

use anyhow::Result;

/// Platform-specific system audio implementation
pub trait SystemAudio {
    /// Create a virtual audio device
    fn create_virtual_device(&self) -> Result<()>;

    /// Start capturing system audio
    fn start_capture(&mut self) -> Result<()>;

    /// Stop capturing system audio
    fn stop_capture(&mut self);

    /// Check if virtual device exists
    fn virtual_device_exists(&self) -> bool;

    /// Remove virtual audio device
    fn remove_virtual_device(&self) -> Result<()>;
}

/// Get the platform-specific system audio implementation
#[cfg(target_os = "macos")]
pub fn get_system_audio() -> Box<dyn SystemAudio> {
    Box::new(macos::MacOSSystemAudio::new())
}

#[cfg(target_os = "linux")]
pub fn get_system_audio() -> Box<dyn SystemAudio> {
    Box::new(linux::LinuxSystemAudio::new())
}

#[cfg(target_os = "windows")]
pub fn get_system_audio() -> Box<dyn SystemAudio> {
    Box::new(windows::WindowsSystemAudio::new())
}
