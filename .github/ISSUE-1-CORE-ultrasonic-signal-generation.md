
**Title:** Core: Implement Ultrasonic Signal Generation

**Description:**

Create a Go package responsible for generating a pure sinusoidal ultrasonic tone. This will be the core component used by both the On-System and Speaker Jammer modes.

**Acceptance Criteria:**

- A new package, `internal/ultrasonic`, is created.
- The package provides a function to generate a sine wave at a given frequency (e.g., 25kHz).
- The output should be raw audio data (e.g., a slice of floats or bytes) that can be consumed by an audio playback library.
- The implementation should be efficient and avoid unnecessary memory allocations.
- Add basic unit tests for the signal generation.
