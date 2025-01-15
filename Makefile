# Makefile for the Camouflage project

.PHONY: all build test clean lint run-speaker run-system bench

# Rust parameters
CARGO=cargo
CARGO_BUILD=$(CARGO) build
CARGO_TEST=$(CARGO) test
CARGO_CLEAN=$(CARGO) clean
CARGO_RUN=$(CARGO) run
CARGO_BENCH=$(CARGO) bench

# Binary name
BINARY_NAME=camouflage

all: build

build:
	@echo "Building $(BINARY_NAME)..."
	$(CARGO_BUILD) --release

test:
	@echo "Running tests..."
	$(CARGO_TEST) --workspace --verbose

clean:
	@echo "Cleaning..."
	$(CARGO_CLEAN)

lint:
	@echo "Linting..."
	$(CARGO) clippy --all-targets --all-features -- -D warnings
	$(CARGO) fmt --check

fmt:
	@echo "Formatting..."
	$(CARGO) fmt

# Run modes
run-speaker: build
	@echo "Running Speaker Jammer mode..."
	./target/release/$(BINARY_NAME) speaker

run-system: build
	@echo "Running System Jammer mode..."
	./target/release/$(BINARY_NAME) system

# Development runs (without release build)
dev-speaker:
	@echo "Running Speaker Jammer mode (dev)..."
	$(CARGO_RUN) -- speaker

dev-system:
	@echo "Running System Jammer mode (dev)..."
	$(CARGO_RUN) -- system

# Benchmarks
bench:
	@echo "Running benchmarks..."
	$(CARGO_BENCH)

# Install binary to system
install:
	@echo "Installing $(BINARY_NAME)..."
	$(CARGO) install --path camouflage
