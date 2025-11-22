.PHONY: help build release test fmt clippy clean run install dev check all ci

# Default target
help:
	@echo "Jaman - Available Commands:"
	@echo ""
	@echo "  make build      - Build in debug mode"
	@echo "  make release    - Build in release mode"
	@echo "  make test       - Run all tests"
	@echo "  make fmt        - Format code"
	@echo "  make clippy     - Run clippy linter"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make run        - Run the application"
	@echo "  make install    - Install jaman globally"
	@echo "  make dev        - Run in development mode"
	@echo "  make check      - Run all checks (fmt, clippy, test)"
	@echo "  make ci         - Run CI checks"
	@echo "  make all        - Format, check, test, and build"

# Build debug version
build:
	cargo build

# Build release version
release:
	cargo build --release

# Run tests
test:
	cargo test --verbose

# Run tests with output
test-output:
	cargo test --verbose -- --nocapture

# Format code
fmt:
	cargo fmt

# Check formatting
fmt-check:
	cargo fmt -- --check

# Run clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Clean build artifacts
clean:
	cargo clean

# Run the application
run:
	cargo run

# Run with arguments
run-args:
	cargo run -- $(ARGS)

# Install globally
install:
	cargo install --path .

# Uninstall
uninstall:
	cargo uninstall jaman

# Development mode (watch for changes)
dev:
	cargo watch -x run

# Check everything
check: fmt-check clippy test

# CI checks
ci: fmt-check clippy test build

# Build and run all checks
all: fmt check test release
	@echo "All checks passed!"

# Update dependencies
update:
	cargo update

# Audit dependencies for security issues
audit:
	cargo audit

# Generate documentation
doc:
	cargo doc --no-deps --all-features

# Generate and open documentation
doc-open:
	cargo doc --no-deps --all-features --open

# Benchmark (if benchmarks are configured)
bench:
	cargo bench

# Check for outdated dependencies
outdated:
	cargo outdated

# Run specific test
test-one:
	cargo test $(TEST) -- --nocapture

# Build for all targets (requires cross)
build-all:
	cargo build --release --target x86_64-pc-windows-msvc
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-apple-darwin

# Size optimization check
bloat:
	cargo bloat --release

# Create release artifacts
package-release:
	mkdir -p releases
	cargo build --release --target x86_64-pc-windows-msvc
	cargo build --release --target x86_64-unknown-linux-gnu
	@echo "Release binaries created in target/<target>/release/"
