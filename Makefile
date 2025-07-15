
# Makefile for the Camouflage project

.PHONY: all build test clean lint run-speaker run-system

# Go parameters
GOCMD=go
GOBUILD=$(GOCMD) build
GOCLEAN=$(GOCMD) clean
GOTEST=$(GOCMD) test
# The standard linter for Go is golangci-lint. It can be installed via:
# curl -sSfL https://raw.githubusercontent.com/golangci/golangci-lint/master/install.sh | sh -s -- -b $(go env GOPATH)/bin v1.55.2
GOLINT=golangci-lint run
GORUN=$(GOCMD) run

# Binary name
BINARY_NAME=camouflage

all: build

build:
	@echo "Building $(BINARY_NAME)..."
	$(GOBUILD) -o $(BINARY_NAME) ./cmd/camouflage

test:
	@echo "Running tests..."
	$(GOTEST) -v ./...

clean:
	@echo "Cleaning..."
	$(GOCLEAN)
	rm -f $(BINARY_NAME)

lint:
	@echo "Linting... (ensure golangci-lint is installed)"
	$(GOLINT)

# Run modes
run-speaker: build
	@echo "Running Speaker Jammer mode..."
	./$(BINARY_NAME) --mode speaker

run-system: build
	@echo "Running On-System Jammer mode..."
	./$(BINARY_NAME) --mode system

