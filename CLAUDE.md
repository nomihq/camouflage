# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Camouflage is a Go application that generates ultrasonic signals (24-26kHz) to disrupt audio recording via two distinct modes:

1. **Speaker Jammer Mode** (`--mode speaker`) - Outputs ultrasonic signals through speakers to jam nearby microphones
2. **On-System Jammer Mode** (`--mode system`) - Creates virtual audio devices to prevent remote recording during voice calls

## Common Development Commands

### Build and Run
- `make build` - Build the camouflage binary
- `make run-speaker` - Build and run in speaker jammer mode
- `make run-system` - Build and run in on-system jammer mode
- `go run ./cmd/camouflage --mode speaker` - Run speaker mode without building
- `go run ./cmd/camouflage --mode system` - Run system mode without building

### Testing and Quality
- `make test` - Run all tests with verbose output (`go test -v ./...`)
- `make lint` - Run golangci-lint (requires golangci-lint installation)
- `make clean` - Clean build artifacts

### Linter Installation
```bash
curl -sSfL https://raw.githubusercontent.com/golangci/golangci-lint/master/install.sh | sh -s -- -b $(go env GOPATH)/bin v1.55.2
```

## Architecture

### Current Structure
- `cmd/camouflage/main.go` - Entry point with CLI flag parsing for mode selection
- Single binary with two execution modes controlled by `--mode` flag
- Built as Go module `github.com/nomihq/camouflage` targeting Go 1.24.5

### Development Phases (from IMPLEMENTATION_PLAN.md)
1. **Phase 1**: Core signal generation + speaker jammer (macOS)
2. **Phase 2**: Virtual audio device implementation (macOS) 
3. **Phase 3**: Cross-platform support (Windows, Linux)
4. **Phase 4**: TDD & CI/CD implementation

### Platform-Specific Components (Planned)
- **macOS**: Core Audio AudioServerPlugin/AudioDevice for virtual devices
- **Windows**: WASAPI loopback or virtual audio drivers
- **Linux**: PulseAudio/PipeWire null sinks and loopback devices

## Key Implementation Notes

- Ultrasonic signal generation will be implemented in a shared Go package
- On-system mode requires audio mixing: capture system output → mix with ultrasonic → route to speakers
- Speaker mode outputs ultrasonic signal directly to default audio device
- Multi-tone output enhancement planned for increased effectiveness

## GitHub Integration

The repository uses Claude Code GitHub Actions integration triggered by `@claude` mentions in:
- Issue comments
- PR review comments  
- Issue bodies/titles
- PR reviews

## Module Information
- Go module: `github.com/nomihq/camouflage`
- Go version: 1.24.5
- No external dependencies currently (standard library only)