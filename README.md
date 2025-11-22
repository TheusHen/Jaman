# Jaman - Java Manager

<div align="center">

![Jaman Logo](https://img.shields.io/badge/Jaman-Java%20Manager-blue?style=for-the-badge&logo=java)

A powerful, modern command-line tool for managing multiple Java Development Kit (JDK) installations on a single machine.

[![CI](https://github.com/TheusHen/jaman/workflows/CI/badge.svg)](https://github.com/TheusHen/jaman/actions/workflows/ci.yml)
[![Release](https://github.com/TheusHen/jaman/workflows/Release/badge.svg)](https://github.com/TheusHen/jaman/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)

</div>

## ✨ Features

- 🚀 **Fast & Efficient** - Built in Rust for maximum performance
- 📦 **Easy Installation** - Download and install any Java version with a single command
- 🔄 **Quick Switching** - Instantly switch between Java versions
- 🔍 **Auto-Detection** - Automatically finds existing Java installations on your system
- 🎨 **Beautiful CLI** - Rich terminal UI with progress bars and animations
- 🛠️ **Version Management** - List, install, activate, and remove Java versions
- 🏥 **Health Checks** - Built-in diagnostics to verify your Java setup
- 🧹 **Cleanup Tools** - Remove unused versions to free up disk space
- 🌐 **Multi-Source** - Downloads from Eclipse Adoptium (Temurin) and other vendors
- 💾 **LTS Support** - Easy filtering and identification of Long Term Support versions

## 📋 Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Commands](#commands)
- [Usage Examples](#usage-examples)
- [Configuration](#configuration)
- [Building from Source](#building-from-source)
- [Contributing](#contributing)
- [License](#license)

## 📚 Documentation

- [Quick Start Guide](QUICKSTART.md) - Get started in 5 minutes
- [Usage Examples](EXAMPLES.md) - Detailed usage scenarios
- [Build Instructions](BUILD.md) - How to build from source
- [Contributing Guide](CONTRIBUTING.md) - How to contribute
- [Code of Conduct](CODE_OF_CONDUCT.md) - Community guidelines
- [Security Policy](SECURITY.md) - Report security issues
- [Changelog](CHANGELOG.md) - Version history
- [Project Summary](PROJECT_SUMMARY.md) - Complete project overview

## 🔧 Installation

### Quick Install (Recommended)

#### Windows (PowerShell)

Run this command in PowerShell:

```powershell
irm https://raw.githubusercontent.com/TheusHen/jaman/main/install.ps1 | iex
```

The installer will:
- Download the latest version to `%USERPROFILE%\Downloads\jaman`
- Add it to your PATH automatically
- No need to move files or configure manually

#### macOS / Linux

Run this command in your terminal:

```bash
curl -fsSL https://raw.githubusercontent.com/TheusHen/jaman/main/install.sh | bash
```

The installer will:
- Download the latest version to `~/Downloads/jaman`
- Add it to your PATH automatically
- Update your shell configuration

### Manual Installation

#### From Binary

1. Download the appropriate binary for your platform from the [Releases](https://github.com/TheusHen/jaman/releases) page:
   - **Windows**: `jaman-windows-x64.exe.zip`
   - **macOS (Intel)**: `jaman-macos-x64.tar.gz`
   - **macOS (Apple Silicon)**: `jaman-macos-arm64.tar.gz`
   - **Linux (x64)**: `jaman-linux-x64.tar.gz`
   - **Linux (ARM64)**: `jaman-linux-arm64.tar.gz`

2. Extract to your preferred location (e.g., `Downloads` folder)

3. Run jaman for the first time - **it will automatically add itself to your system PATH!**
   ```bash
   # Windows
   .\jaman.exe --help
   
   # macOS/Linux
   ./jaman --help
   ```

4. Open a new terminal window and type:
   ```bash
   jaman --help
   ```

> **Note**: On the first run, jaman automatically adds itself to your system PATH. On Windows, this takes effect immediately in new terminal windows. On macOS/Linux, you may need to restart your terminal or run `source ~/.bashrc` (or `~/.zshrc`).

#### From Source

```bash
# Clone the repository
git clone https://github.com/TheusHen/jaman.git
cd jaman

# Build the project
cargo build --release

# Install globally (adds to ~/.cargo/bin)
cargo install --path .
```

### Verify Installation

```bash
jaman --version
```

**Note**: You may need to restart your terminal after installation.

## 🚀 Quick Start

```bash
# Show status and available commands
jaman

# Scan your system for existing Java installations
jaman scan

# List available Java versions to download
jaman list --available

# Install a specific Java version
jaman install 21

# Activate a Java version
jaman activate 21

# List installed versions
jaman list

# Check your setup
jaman doctor
```

## 📖 Commands

### `jaman` (no arguments)
Display current status, active version, and quick command reference.

```bash
jaman
```

### `jaman list` (alias: `ls`)
List installed Java versions.

```bash
# List all installed versions
jaman list

# List available versions for download
jaman list --available

# Filter by version number
jaman list 21

# Show only LTS versions
jaman list --lts

# Show only GraalVM versions
jaman list --graalvm

# Combine filters
jaman list --available --lts
```

### `jaman install` (alias: `i`)
Download and install a Java version.

```bash
# Interactive installation (shows version picker)
jaman install

# Install specific version
jaman install 21

# Install with partial match
jaman install 17.0.1
```

### `jaman activate` (alias: `use`)
Switch to a different Java version.

```bash
# Interactive activation (shows version picker)
jaman activate

# Activate specific version
jaman activate 21

# Activate with partial match
jaman activate 17
```

### `jaman scan`
Scan system for existing Java installations and add them to jaman.

```bash
jaman scan
```

**Enhanced scanning features:**
- Searches **all available disk drives** (C:, D:, E:, etc.) on Windows
- Detects Java installations via `java -version` and system PATH
- Uses `where java` (Windows) or `which java` (Unix) to find active installations
- Searches common installation directories:
  - **Windows**: All drives under `Program Files\Java`, `Program Files\Eclipse Adoptium`, etc.
  - **macOS**: `/Library/Java/JavaVirtualMachines`
  - **Linux**: `/usr/lib/jvm`, `/usr/java`, `/opt/java`

### `jaman doctor`
Run diagnostics to verify jaman configuration and Java setup.

```bash
jaman doctor
```

Checks:
- Configuration file validity
- Active Java version
- Java executable accessibility
- All tracked installations
- Installation directory

### `jaman clean`
Remove unused Java installations to free up disk space.

```bash
# Remove versions unused for 90 days (default)
jaman clean

# Remove versions unused for 30 days
jaman clean --days 30

# Skip confirmation prompt
jaman clean --force
```

### `jaman config`
Configure jaman settings.

```bash
# Show current configuration
jaman config --show

# Set custom installation directory
jaman config --set-install-dir "D:\Java\JDKs"

# Set custom download directory
jaman config --set-download-dir "D:\Java\Downloads"
```

## 💡 Usage Examples

### First Time Setup

```bash
# Install jaman and scan your system
jaman scan

# See what's available
jaman list --available --lts

# Install Java 21 LTS
jaman install 21

# Activate it
jaman activate 21

# Verify
java -version
```

### Managing Multiple Versions

```bash
# Install multiple versions
jaman install 8
jaman install 11
jaman install 17
jaman install 21

# List them
jaman list

# Switch between versions
jaman activate 11
java -version

jaman activate 21
java -version
```

### Project-Specific Java Versions

```bash
# Working on legacy project (Java 8)
cd my-legacy-project
jaman activate 8

# Working on modern project (Java 21)
cd my-modern-project
jaman activate 21
```

### Cleanup

```bash
# Check what's taking up space
jaman list

# Clean up old versions
jaman clean --days 60

# Remove specific version manually
# (then use jaman scan to resync)
```

## ⚙️ Configuration

Jaman stores its configuration in:
- **Windows**: `%APPDATA%\jaman\config.toml`
- **macOS/Linux**: `~/.config/jaman/config.toml`

Default installation directory:
- **Windows**: `%LOCALAPPDATA%\jaman\jdks`
- **macOS/Linux**: `~/.local/share/jaman/jdks`

### Configuration File Structure

```toml
installation_dir = "C:\\Users\\YourName\\AppData\\Local\\jaman\\jdks"
active_version = "21.0.1"
last_scan = "2025-11-22T10:30:00Z"

[[installed_versions]]
version = "21.0.1"
vendor = "Eclipse Temurin"
path = "C:\\Users\\YourName\\AppData\\Local\\jaman\\jdks\\Eclipse_Temurin-21.0.1"
is_lts = true
architecture = "x64"
auto_detected = false
last_used = "2025-11-22T10:30:00Z"
```

## 🏗️ Building from Source

### Development Build

```bash
# Clone and enter directory
git clone https://github.com/TheusHen/jaman.git
cd jaman

# Run in development mode
cargo run -- --help

# Run with specific command
cargo run -- list
```

### Release Build

```bash
# Build optimized binary
cargo build --release

# Binary will be at target/release/jaman
./target/release/jaman --version
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## 🏗️ Project Structure

```
jaman/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── config.rs            # Configuration management
│   ├── detector.rs          # Java installation detection
│   ├── downloader.rs        # Download and installation
│   ├── path_manager.rs      # PATH and environment management
│   └── commands/
│       ├── mod.rs
│       ├── list.rs          # List command
│       ├── install.rs       # Install command
│       ├── activate.rs      # Activate command
│       ├── scan.rs          # Scan command
│       ├── doctor.rs        # Doctor command
│       └── clean.rs         # Clean command
├── Cargo.toml
└── README.md
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Eclipse Adoptium](https://adoptium.net/) - For providing high-quality OpenJDK binaries
- [Rust](https://www.rust-lang.org/) - For an amazing systems programming language
- All the crate authors whose libraries make this project possible

## 📞 Support

If you encounter any issues or have questions:

- Open an [issue](https://github.com/TheusHen/jaman/issues)
- Check existing [discussions](https://github.com/TheusHen/jaman/discussions)

## 🗺️ Roadmap

- [ ] Support for additional JDK vendors (GraalVM, Amazon Corretto, Zulu, etc.)
- [ ] Automatic Java version detection per project (.java-version file)
- [ ] Integration with popular build tools (Maven, Gradle)
- [ ] Homebrew formula for macOS
- [ ] Chocolatey package for Windows
- [ ] GUI application
- [ ] Shell completion scripts