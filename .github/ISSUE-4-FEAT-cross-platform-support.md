
**Title:** Feat: Add Windows and Linux Support

**Description:**

Port both the Speaker Jammer and On-System Jammer modes to Windows and Linux.

**Acceptance Criteria:**

- **Windows:**
  - Speaker Jammer works using the default audio device.
  - On-System Jammer is implemented, likely using WASAPI loopback or a virtual audio driver.
- **Linux:**
  - Speaker Jammer works using the default audio device.
  - On-System Jammer is implemented, likely using PulseAudio or PipeWire loopback devices.
- The `Makefile` is updated to support building and testing on all three platforms.
