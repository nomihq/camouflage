
# Camouflage: Implementation Plan

This document outlines the development plan for the Camouflage project, an application designed to disrupt audio recording through both on-system and physical speaker-based ultrasonic jamming.

## Core Technology

The primary mechanism for jamming is the generation of a high-frequency ultrasonic signal. This signal, inaudible to humans, interferes with the physical components of most microphones, rendering recordings unintelligible.

- **Signal Generation:** A pure sinusoidal tone will be generated in the 24kHz - 26kHz range. This will be implemented in a shared Go package.

## Application Modes

The application will operate in two distinct modes:

### 1. On-System Jammer (`--mode system`)

**Goal:** Prevent remote participants in a voice call from recording the user's audio.

**Implementation:**

- **Virtual Audio Device:** A virtual audio output device will be created on the host operating system. This is the most platform-dependent part of the project.
  - **macOS:** Utilize Core Audio to create an `AudioServerPlugin` or a virtual `AudioDevice`.
  - **Windows:** Use WASAPI loopback or create a virtual audio driver (e.g., using `ViGEm` as a template).
  - **Linux:** Leverage PulseAudio or PipeWire to create a null sink and loopback device.
- **Audio Mixing:** The application will capture the system's audio output, mix it with the generated ultrasonic signal, and then route the combined audio to the user's actual speakers or headphones.

### 2. Speaker Jammer (`--mode speaker`)

**Goal:** Prevent physical recording of conversations in a room by jamming nearby microphones.

**Implementation:**

- **Audio Output:** The generated ultrasonic signal will be sent directly to the default audio output device (speakers).
- **Multi-tone Output (Enhancement):** To increase effectiveness, we may explore generating multiple, slightly varying ultrasonic frequencies simultaneously.

## Development Phases

1.  **Phase 1: Core Signal Generation & Speaker Jammer (macOS)**
    - Implement the ultrasonic signal generation logic in Go.
    - Implement the Speaker Jammer mode for macOS as a proof-of-concept.

2.  **Phase 2: On-System Jammer (macOS)**
    - Tackle the virtual audio device implementation for macOS.
    - Implement the audio mixing and routing.

3.  **Phase 3: Cross-Platform Expansion**
    - Port the Speaker Jammer and On-System Jammer to Windows and Linux.

4.  **Phase 4: TDD & CI/CD**
    - Develop a comprehensive test suite for all components.
    - Set up GitHub Actions to automate builds, testing, and linting for all three platforms.

