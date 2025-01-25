# Contributing to Camouflage

Thank you for your interest in contributing to Camouflage! This document provides guidelines and information for contributors.

## Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help maintain a positive community

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Platform-specific audio libraries:
  - **Linux**: `libasound2-dev` (ALSA)
  - **macOS**: No additional dependencies
  - **Windows**: No additional dependencies

### Development Setup

1. Clone the repository:
```bash
git clone https://github.com/nomihq/camouflage.git
cd camouflage
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

4. Run linter:
```bash
cargo clippy --all-targets --all-features
cargo fmt --check
```

## Development Workflow

### 1. Code Style

We follow standard Rust conventions:

- Use `cargo fmt` for formatting
- Follow `cargo clippy` recommendations
- Write idiomatic Rust code
- Add documentation for public APIs

### 2. Testing

All contributions should include tests:

- **Unit tests**: For individual functions/modules
- **Integration tests**: For component interaction
- **E2E tests**: For user-facing features (when applicable)

Run tests:
```bash
# All tests
cargo test

# Specific workspace
cargo test -p camouflage-core

# With logging
RUST_LOG=debug cargo test -- --nocapture
```

### 3. Benchmarks

Performance-critical changes should include benchmarks:

```bash
cargo bench
```

Benchmarks live in `camouflage-tests/benches/`.

### 4. Documentation

- Add doc comments (`///`) for public APIs
- Update README.md for user-facing changes
- Update ARCHITECTURE.md for design changes
- Include examples in documentation

## Contribution Process

### 1. Find or Create an Issue

- Check existing issues for similar work
- Create a new issue describing the problem/enhancement
- Wait for maintainer feedback before starting large changes

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

### 3. Make Changes

- Write clean, well-documented code
- Follow existing patterns and conventions
- Add tests for new functionality
- Update documentation as needed

### 4. Test Locally

```bash
# Run all tests
cargo test

# Run linter
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Run benchmarks (if applicable)
cargo bench
```

### 5. Commit Changes

Use conventional commits:

```
feat: add new feature
fix: resolve bug
docs: update documentation
test: add tests
perf: improve performance
refactor: restructure code
ci: update workflows
```

### 6. Push and Create PR

```bash
git push origin your-branch-name
```

Create a pull request with:
- Clear description of changes
- Link to related issue
- Test results (if applicable)
- Breaking changes (if any)

### 7. Code Review

- Address reviewer feedback
- Update PR as needed
- Maintain clean commit history

## Areas for Contribution

### High Priority

1. **Virtual Audio Device Implementation**
   - macOS: Core Audio AudioServerPlugin
   - Windows: WASAPI loopback
   - Linux: PulseAudio/PipeWire integration

2. **Enhanced Testing**
   - More E2E scenarios
   - Additional STT service testing
   - Cross-platform validation

3. **Documentation**
   - Usage examples
   - Platform-specific guides
   - Video tutorials

### Medium Priority

1. **Performance Optimization**
   - Reduce CPU usage
   - Optimize buffer management
   - SIMD optimizations

2. **Additional Features**
   - Frequency hopping
   - Adaptive jamming
   - Configuration presets

3. **User Experience**
   - Better error messages
   - Progress indicators
   - Interactive mode

### Low Priority

1. **Platform Support**
   - BSD variants
   - ARM optimizations
   - Mobile platforms

## Platform-Specific Development

### macOS

Virtual audio device implementation requires:
- Core Audio framework knowledge
- AudioServerPlugin development
- Code signing and entitlements

### Linux

System jammer requires:
- PulseAudio or PipeWire API knowledge
- D-Bus interaction
- Permission management

### Windows

System jammer requires:
- WASAPI expertise
- COM interface knowledge
- Admin privilege handling

## Testing Guidelines

### Unit Tests

Located in source files:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test code
    }
}
```

### Integration Tests

Located in `tests/` directories:
```rust
#[test]
fn test_integration() {
    // Integration test
}
```

### E2E Tests

Marked with `#[ignore]` and require API keys:
```rust
#[tokio::test]
#[ignore]
async fn test_e2e() {
    // E2E test requiring external services
}
```

Run with:
```bash
cargo test -- --ignored
```

## Release Process

Releases are automated via GitHub Actions:

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create and push tag:
```bash
git tag v0.x.0
git push origin v0.x.0
```

4. GitHub Actions will:
   - Build for all platforms
   - Create GitHub release
   - Publish to crates.io

## Questions?

- Open an issue for questions
- Check existing documentation
- Review closed issues/PRs for context

Thank you for contributing to Camouflage!
