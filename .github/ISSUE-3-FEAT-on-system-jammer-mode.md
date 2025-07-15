
**Title:** Feat: Implement On-System Jammer Mode (macOS)

**Description:**

Implement the `--mode system` functionality for macOS. This involves creating a virtual audio device to intercept system audio, mix it with the ultrasonic signal, and output it.

**This is a research-heavy task.**

**Acceptance Criteria:**

- **[Spike]** Research and document the best approach for creating a virtual audio device on macOS using Go. This may require Cgo and interacting with Core Audio APIs.
- Create a virtual audio output device named "Camouflage".
- When "Camouflage" is set as the system's audio output, the application captures the audio stream.
- The captured audio is mixed with the ultrasonic signal from the `ultrasonic` package.
- The final, mixed audio is outputted to the actual, physical speakers.
