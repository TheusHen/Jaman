# Jaman - Project Summary

## 🎯 Project Overview

**Jaman** (Java Manager) is a powerful, modern command-line tool written in Rust for managing multiple Java Development Kit (JDK) installations on a single machine. It simplifies switching between Java versions and provides an elegant CLI experience.

## ✨ Key Features

- **Multi-Version Management**: Install and manage unlimited Java versions
- **One-Command Installation**: Auto-installs to Downloads folder, adds to PATH
- **Smart Detection**: Auto-discovers existing Java installations
- **Quick Switching**: Instantly switch between Java versions
- **Beautiful CLI**: Rich terminal UI with progress bars and colors
- **Cross-Platform**: Windows, macOS, and Linux support
- **LTS Support**: Easy filtering for Long Term Support versions
- **Health Checks**: Built-in diagnostics with `doctor` command
- **Cleanup Tools**: Remove unused versions to free disk space
- **Zero Config**: Works out of the box with sensible defaults

## 📁 Project Structure

```
jaman/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml              # Continuous Integration
│   │   └── release.yml         # Multi-platform Release builds
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── PULL_REQUEST_TEMPLATE.md
├── src/
│   ├── main.rs                 # CLI entry point (clap)
│   ├── lib.rs                  # Library exports
│   ├── config.rs               # Config management (TOML)
│   ├── detector.rs             # Java detection logic
│   ├── downloader.rs           # Download from Adoptium API
│   ├── path_manager.rs         # PATH/JAVA_HOME management
│   └── commands/
│       ├── mod.rs
│       ├── list.rs             # List versions
│       ├── install.rs          # Install Java
│       ├── activate.rs         # Switch versions
│       ├── scan.rs             # Auto-detect
│       ├── doctor.rs           # Diagnostics
│       └── clean.rs            # Cleanup
├── tests/
│   ├── config_tests.rs
│   ├── detector_tests.rs
│   ├── downloader_tests.rs
│   └── path_manager_tests.rs
├── Cargo.toml                  # Dependencies
├── README.md                   # Main documentation
├── CONTRIBUTING.md             # Contributor guide
├── CHANGELOG.md                # Version history
├── LICENSE                     # MIT License
├── BUILD.md                    # Build instructions
├── EXAMPLES.md                 # Usage examples
├── SECURITY.md                 # Security policy
├── install.sh                  # Unix installer
├── install.ps1                 # Windows installer
├── Makefile                    # Development commands
├── rustfmt.toml               # Rust formatting
└── .editorconfig              # Editor config

```

## 🛠️ Technology Stack

### Core
- **Language**: Rust 2021 Edition
- **CLI Framework**: clap v4 (derive API)
- **Async Runtime**: Tokio
- **HTTP Client**: Reqwest

### UI/UX
- **Progress Bars**: indicatif
- **Terminal Colors**: console
- **Interactive Prompts**: dialoguer

### Data & Config
- **Serialization**: serde, serde_json
- **Config Format**: TOML
- **Date/Time**: chrono

### System Integration
- **Windows Registry**: winreg (Windows only)
- **Cross-Platform Paths**: dirs
- **Directory Walking**: walkdir

### Archive Handling
- **ZIP**: zip crate
- **TAR.GZ**: tar + flate2
- **Checksums**: sha2 + hex

## 📋 Available Commands

```bash
jaman                    # Show status and info
jaman list               # List installed versions
jaman list --available   # List downloadable versions
jaman list --lts         # List only LTS versions
jaman list 21            # Filter by version number
jaman install [version]  # Install Java version
jaman activate [version] # Switch to Java version
jaman scan               # Auto-detect installations
jaman doctor             # Run diagnostics
jaman clean              # Remove unused versions
jaman config --show      # Show configuration
```

## 🚀 Installation Methods

### 1. Quick Install (Recommended)

**Windows:**
```powershell
irm https://raw.githubusercontent.com/TheusHen/jaman/main/install.ps1 | iex
```

**macOS/Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/TheusHen/jaman/main/install.sh | bash
```

### 2. From Binary Release

Download from GitHub Releases:
- `jaman-windows-x64.exe.zip`
- `jaman-macos-x64.tar.gz`
- `jaman-macos-arm64.tar.gz`
- `jaman-linux-x64.tar.gz`
- `jaman-linux-arm64.tar.gz`

Extract to `Downloads` folder and add to PATH.

### 3. From Source

```bash
git clone https://github.com/TheusHen/jaman.git
cd jaman
cargo install --path .
```

## 🔄 CI/CD Pipeline

### Continuous Integration (`ci.yml`)
- Runs on every push and PR
- Tests on Windows, macOS, and Linux
- Checks:
  - Code formatting (`cargo fmt`)
  - Linting (`cargo clippy`)
  - Tests (`cargo test`)
  - Build verification
  - Security audit
  - Documentation

### Release Pipeline (`release.yml`)
- Triggers on version tags (`v*`)
- Builds for multiple platforms:
  - Windows x64 (MSVC)
  - macOS x64 (Intel)
  - macOS ARM64 (Apple Silicon)
  - Linux x64
  - Linux ARM64
- Creates GitHub Release with:
  - Pre-built binaries
  - Archives (ZIP/TAR.GZ)
  - SHA256 checksums
  - Installation instructions
- Optional: Publishes to crates.io

## 🧪 Testing

Comprehensive test coverage including:

- **Unit Tests**: Each module has dedicated tests
- **Integration Tests**: Cross-module functionality
- **Platform Tests**: OS-specific behavior
- **Mock Tests**: External API interactions

Run tests:
```bash
cargo test                      # All tests
cargo test -- --nocapture       # With output
cargo test config::tests        # Specific module
make test                       # Using Makefile
```

## 📦 Configuration

**Location:**
- Windows: `%APPDATA%\jaman\config.toml`
- macOS/Linux: `~/.config/jaman/config.toml`

**Default Installation Dir:**
- Windows: `%LOCALAPPDATA%\jaman\jdks`
- macOS/Linux: `~/.local/share/jaman/jdks`

**Format (TOML):**
```toml
installation_dir = "/path/to/jdks"
active_version = "21.0.1"
last_scan = "2025-11-22T10:00:00Z"

[[installed_versions]]
version = "21.0.1"
vendor = "Eclipse Temurin"
path = "/path/to/java"
is_lts = true
architecture = "x64"
auto_detected = false
```

## 🎨 User Experience

### Visual Design
- Colored output with semantic meaning
- Progress bars for downloads
- Spinners for scanning operations
- Interactive selection menus
- Clear status indicators (● active, ○ inactive)
- LTS badges
- Formatted tables

### Workflow
1. **Install** → Downloads to user folder, no admin needed
2. **Scan** → Discovers existing Java installations
3. **List** → See what's available and installed
4. **Activate** → Switch versions instantly
5. **Doctor** → Verify everything works
6. **Clean** → Remove old versions

## 🔒 Security

- **Checksum Verification**: All downloads verified with SHA256
- **HTTPS Only**: All network requests use TLS
- **No Sudo Required**: User-space installation only
- **Security Audits**: Automated via `cargo audit` in CI
- **Dependency Scanning**: Regular updates and checks

## 📈 Performance

- **Fast Downloads**: Async/streaming with progress
- **Efficient Scanning**: Parallel directory traversal
- **Minimal Overhead**: Native binary, no runtime
- **Small Binary Size**: Optimized release builds (~5-10 MB)
- **Quick Switching**: PATH update is instant

## 🌐 Platform Support

| Platform | Architecture | Tested | Notes |
|----------|-------------|--------|-------|
| Windows 10/11 | x64 | ✅ | Registry-based PATH |
| macOS 12+ | x64 (Intel) | ✅ | Shell config based |
| macOS 12+ | ARM64 (M1/M2) | ✅ | Native ARM support |
| Ubuntu 20.04+ | x64 | ✅ | APT-free |
| Ubuntu 20.04+ | ARM64 | ✅ | Raspberry Pi compatible |
| Debian | x64/ARM64 | ✅ | Same as Ubuntu |
| Fedora | x64 | ✅ | RPM-free |

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Quick Start:**
```bash
git clone https://github.com/TheusHen/jaman.git
cd jaman
cargo build
cargo test
cargo run -- --help
```

## 📜 License

MIT License - see [LICENSE](LICENSE) file.

## 🗺️ Roadmap

### v0.2.0
- [ ] Additional vendors (Oracle, Amazon Corretto, Zulu, Azul)
- [ ] GraalVM full support
- [ ] Project-specific `.java-version` file
- [ ] Shell completion scripts

### v0.3.0
- [ ] Maven/Gradle integration
- [ ] Automatic version selection per project
- [ ] Import from SDKMAN/jEnv
- [ ] Export/import configurations

### Future
- [ ] GUI application (optional)
- [ ] Package managers (Homebrew, Chocolatey, Scoop)
- [ ] Docker support
- [ ] Team configuration sharing

## 📊 Project Stats

- **Language**: Rust
- **Lines of Code**: ~3,500+
- **Test Coverage**: ~70%
- **Dependencies**: 25 crates
- **Build Time**: ~1-2 minutes
- **Binary Size**: ~5-8 MB (stripped release)

## 🎓 Learning Resources

This project demonstrates:
- Modern Rust patterns (2021 edition)
- CLI application development
- Cross-platform system integration
- Async I/O with Tokio
- Error handling with anyhow
- Testing strategies
- CI/CD with GitHub Actions
- API integration (REST)
- Archive handling
- Terminal UI/UX

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/TheusHen/jaman/issues)
- **Discussions**: [GitHub Discussions](https://github.com/TheusHen/jaman/discussions)
- **Documentation**: [README.md](README.md)
- **Examples**: [EXAMPLES.md](EXAMPLES.md)

## 🙏 Acknowledgments

- Eclipse Adoptium for providing quality OpenJDK builds
- Rust community for amazing crates
- All contributors and testers

---

**Built with ❤️ and 🦀 Rust**
