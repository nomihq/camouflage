use super::SystemAudio;
use anyhow::Result;
use tracing::{info, warn};

/// Windows system audio implementation
#[allow(dead_code)]
pub struct WindowsSystemAudio {
    device_name: String,
}

impl Default for WindowsSystemAudio {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowsSystemAudio {
    pub fn new() -> Self {
        Self {
            device_name: "Camouflage Virtual Audio".to_string(),
        }
    }
}

impl SystemAudio for WindowsSystemAudio {
    fn create_virtual_device(&self) -> Result<()> {
        warn!("⚠️  Windows system mode requires manual setup:");
        warn!("");
        warn!("   Option 1: VB-Audio Virtual Cable (Recommended)");
        warn!("   1. Download from: https://vb-audio.com/Cable/");
        warn!("   2. Install VB-Audio Virtual Cable");
        warn!("   3. Set 'CABLE Input' as default playback device");
        warn!("   4. CABLE Output will capture the audio");
        warn!("");
        warn!("   Option 2: VoiceMeeter");
        warn!("   1. Download from: https://vb-audio.com/Voicemeeter/");
        warn!("   2. Install and configure virtual audio routing");
        warn!("");
        warn!("   After setup, run: camouflage system");

        Ok(())
    }

    fn start_capture(&mut self) -> Result<()> {
        info!("System audio capture mode on Windows");
        warn!("Ensure virtual audio cable is installed and configured");
        Ok(())
    }

    fn stop_capture(&mut self) {
        info!("Stopping Windows system audio capture");
    }

    fn virtual_device_exists(&self) -> bool {
        // Check for VB-Cable or VoiceMeeter
        // This is a simplified check
        false
    }

    fn remove_virtual_device(&self) -> Result<()> {
        info!("To remove virtual audio: Uninstall VB-Audio Cable from Control Panel");
        Ok(())
    }
}
