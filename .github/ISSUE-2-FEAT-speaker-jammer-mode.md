
**Title:** Feat: Implement Speaker Jammer Mode (macOS)

**Description:**

Implement the `--mode speaker` functionality for macOS. This mode will generate the ultrasonic signal and play it through the default system speakers.

**Acceptance Criteria:**

- The application correctly handles the `--mode speaker` flag.
- It uses the `ultrasonic` package to generate the signal.
- It uses a Go audio library (e.g., `oto`) to play the generated signal on the default audio output device on macOS.
- The application runs continuously until manually terminated.
