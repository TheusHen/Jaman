# Jaman Build & Development Guide

## Prerequisites

- **Rust**: 1.70 or later
- **Cargo**: Comes with Rust
- **Git**: For version control

## Initial Setup

1. **Install Rust** (if not already installed):
   ```bash
   # Windows
   # Download and run: https://rustup.rs/

   # macOS/Linux
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**:
   ```bash
   git clone https://github.com/TheusHen/jaman.git
   cd jaman
   ```

3. **Verify setup**:
   ```bash
   rustc --version
   cargo --version
   ```

## Building

### Debug Build (Development)

```bash
# Build in debug mode (faster compilation, slower runtime)
cargo build

# The binary will be at: target/debug/jaman.exe (Windows) or target/debug/jaman (Unix)
```

### Release Build (Production)

```bash
# Build with optimizations (slower compilation, faster runtime)
cargo build --release

# The binary will be at: target/release/jaman.exe (Windows) or target/release/jaman (Unix)
```

## Running

### Run Without Building

```bash
# Run directly with cargo
cargo run

# Run with arguments
cargo run -- list
cargo run -- install 21
cargo run -- --help
```

### Run Built Binary

```bash
# Debug build
./target/debug/jaman --version

# Release build
./target/release/jaman --version
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_config_load

# Run tests for specific module
cargo test config::
```

## Code Quality

### Format Code

```bash
# Format all code
cargo fmt

# Check formatting without changing files
cargo fmt -- --check
```

### Lint Code

```bash
# Run clippy linter
cargo clippy

# Run clippy with all features
cargo clippy --all-features

# Run clippy and deny warnings
cargo clippy -- -D warnings
```

### Check Code

```bash
# Fast check without building binary
cargo check

# Check with all features
cargo check --all-features
```

## Development Workflow

1. **Make changes** to the code
2. **Format** the code: `cargo fmt`
3. **Check** for issues: `cargo clippy`
4. **Test** your changes: `cargo test`
5. **Run** the application: `cargo run -- <command>`
6. **Commit** your changes

## Installation

### Install Locally

```bash
# Install to ~/.cargo/bin (added to PATH by rustup)
cargo install --path .

# Now you can use jaman from anywhere
jaman --version
```

### Uninstall

```bash
cargo uninstall jaman
```

## Cross-Compilation

### Build for Windows (from Linux/macOS)

```bash
# Add target
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

### Build for Linux (from Windows/macOS)

```bash
# Add target
rustup target add x86_64-unknown-linux-gnu

# Build
cargo build --release --target x86_64-unknown-linux-gnu
```

### Build for macOS (from Linux/Windows)

```bash
# Add target
rustup target add x86_64-apple-darwin

# Build (requires macOS SDK)
cargo build --release --target x86_64-apple-darwin
```

## Dependency Management

### Update Dependencies

```bash
# Update dependencies to latest compatible versions
cargo update

# Update specific dependency
cargo update tokio
```

### Add New Dependency

```bash
# Add a new dependency
cargo add <crate-name>

# Add with specific version
cargo add <crate-name>@1.0.0

# Add with features
cargo add tokio --features full
```

## Performance Profiling

### Build with Debug Info

```bash
# Release build with debug symbols
cargo build --release --profile release-with-debug
```

### Benchmark

```bash
# Run benchmarks (if configured)
cargo bench
```

## Troubleshooting

### Clean Build

```bash
# Remove build artifacts
cargo clean

# Rebuild from scratch
cargo clean && cargo build --release
```

### Update Rust

```bash
# Update rustup
rustup self update

# Update Rust
rustup update
```

### Check Project Health

```bash
# Verify everything compiles
cargo check --all-targets --all-features

# Run all tests
cargo test --all-features

# Check for outdated dependencies
cargo outdated
```

## Environment Variables

### Cargo Configuration

```bash
# Set custom build target directory
export CARGO_TARGET_DIR=/path/to/target

# Increase verbosity
export CARGO_TERM_VERBOSE=true

# Use specific linker
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=gcc
```

## Creating a Release

1. **Update version** in `Cargo.toml`
2. **Update CHANGELOG.md** with new features/fixes
3. **Test thoroughly**: `cargo test --all-features`
4. **Build release**: `cargo build --release`
5. **Create git tag**: `git tag v0.1.0`
6. **Push tag**: `git push origin v0.1.0`
7. **Create GitHub release** with binaries

## Documentation

### Generate Documentation

```bash
# Generate HTML documentation
cargo doc

# Generate and open in browser
cargo doc --open

# Include private items
cargo doc --document-private-items
```

## Continuous Integration

The project uses GitHub Actions for CI/CD. See `.github/workflows/` for configuration.

### Local CI Checks

```bash
# Run all CI checks locally
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test --all-features
cargo build --release
```

## Tips

- Use `cargo watch` for automatic rebuilds during development:
  ```bash
  cargo install cargo-watch
  cargo watch -x run
  ```

- Use `cargo expand` to see macro expansions:
  ```bash
  cargo install cargo-expand
  cargo expand
  ```

- Use `cargo tree` to visualize dependency tree:
  ```bash
  cargo tree
  ```

- Use `cargo audit` to check for security vulnerabilities:
  ```bash
  cargo install cargo-audit
  cargo audit
  ```
