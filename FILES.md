# Jaman Project Files

Complete list of all project files and their purposes.

## 📂 Repository Structure

```
jaman/
├── .github/                      # GitHub-specific files
│   ├── workflows/
│   │   ├── ci.yml               # Continuous Integration
│   │   ├── release.yml          # Release automation
│   │   └── docs.yml             # Documentation deployment
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md        # Bug report template
│   │   └── feature_request.md   # Feature request template
│   └── PULL_REQUEST_TEMPLATE.md # PR template
│
├── .vscode/                      # VSCode configuration
│   ├── settings.json            # Editor settings
│   ├── extensions.json          # Recommended extensions
│   └── tasks.json              # Build tasks
│
├── src/                          # Source code
│   ├── main.rs                  # Application entry point
│   ├── lib.rs                   # Library exports
│   ├── config.rs                # Configuration module
│   ├── detector.rs              # Java detection
│   ├── downloader.rs            # Download & install
│   ├── path_manager.rs          # PATH management
│   └── commands/                # CLI commands
│       ├── mod.rs
│       ├── list.rs
│       ├── install.rs
│       ├── activate.rs
│       ├── scan.rs
│       ├── doctor.rs
│       └── clean.rs
│
├── tests/                        # Test suite
│   ├── config_tests.rs
│   ├── detector_tests.rs
│   ├── downloader_tests.rs
│   └── path_manager_tests.rs
│
├── Cargo.toml                    # Rust package manifest
├── Cargo.lock                    # Dependency lock file (gitignored)
├── .gitignore                    # Git ignore rules
├── .editorconfig                 # Editor configuration
├── rustfmt.toml                  # Rust formatting config
├── Makefile                      # Development commands
│
├── README.md                     # Main documentation
├── QUICKSTART.md                 # Quick start guide
├── EXAMPLES.md                   # Usage examples
├── PROJECT_SUMMARY.md            # Project overview
├── BUILD.md                      # Build instructions
├── CONTRIBUTING.md               # Contributing guidelines
├── CODE_OF_CONDUCT.md            # Code of conduct
├── SECURITY.md                   # Security policy
├── CHANGELOG.md                  # Version history
├── LICENSE                       # MIT License
│
├── install.sh                    # Unix installer script
└── install.ps1                   # Windows installer script
```

## 📄 File Descriptions

### Core Source Files

| File | Lines | Purpose |
|------|-------|---------|
| `src/main.rs` | ~220 | CLI interface with clap |
| `src/lib.rs` | ~15 | Library exports |
| `src/config.rs` | ~150 | Config management |
| `src/detector.rs` | ~200 | Java version detection |
| `src/downloader.rs` | ~300 | Download & installation |
| `src/path_manager.rs` | ~250 | PATH/env management |
| `src/commands/list.rs` | ~120 | List command |
| `src/commands/install.rs` | ~100 | Install command |
| `src/commands/activate.rs` | ~90 | Activate command |
| `src/commands/scan.rs` | ~80 | Scan command |
| `src/commands/doctor.rs` | ~130 | Doctor command |
| `src/commands/clean.rs` | ~140 | Clean command |

**Total Source Code**: ~1,795 lines

### Test Files

| File | Lines | Purpose |
|------|-------|---------|
| `tests/config_tests.rs` | ~180 | Config module tests |
| `tests/detector_tests.rs` | ~100 | Detector tests |
| `tests/downloader_tests.rs` | ~120 | Downloader tests |
| `tests/path_manager_tests.rs` | ~100 | Path manager tests |

**Total Test Code**: ~500 lines

### Documentation Files

| File | Lines | Purpose |
|------|-------|---------|
| `README.md` | ~500 | Main documentation |
| `QUICKSTART.md` | ~150 | Quick start guide |
| `EXAMPLES.md` | ~450 | Usage examples |
| `PROJECT_SUMMARY.md` | ~400 | Project overview |
| `BUILD.md` | ~250 | Build instructions |
| `CONTRIBUTING.md` | ~400 | Contribution guide |
| `CODE_OF_CONDUCT.md` | ~150 | Code of conduct |
| `SECURITY.md` | ~100 | Security policy |
| `CHANGELOG.md` | ~50 | Version history |

**Total Documentation**: ~2,450 lines

### Configuration Files

| File | Purpose |
|------|---------|
| `Cargo.toml` | Rust package configuration & dependencies |
| `.gitignore` | Git ignore patterns |
| `.editorconfig` | Cross-editor configuration |
| `rustfmt.toml` | Rust code formatting rules |
| `Makefile` | Development task automation |
| `.vscode/settings.json` | VSCode editor settings |
| `.vscode/extensions.json` | Recommended VSCode extensions |
| `.vscode/tasks.json` | VSCode build tasks |

### CI/CD & Automation

| File | Purpose |
|------|---------|
| `.github/workflows/ci.yml` | Continuous integration pipeline |
| `.github/workflows/release.yml` | Release automation & builds |
| `.github/workflows/docs.yml` | Documentation deployment |
| `install.sh` | Unix/Linux/macOS installer |
| `install.ps1` | Windows PowerShell installer |

### Templates

| File | Purpose |
|------|---------|
| `.github/ISSUE_TEMPLATE/bug_report.md` | Bug report template |
| `.github/ISSUE_TEMPLATE/feature_request.md` | Feature request template |
| `.github/PULL_REQUEST_TEMPLATE.md` | Pull request template |

## 📊 Project Statistics

### Code Metrics
- **Total Lines of Code**: ~5,000
  - Source Code: ~1,800 (36%)
  - Tests: ~500 (10%)
  - Documentation: ~2,450 (49%)
  - Configuration: ~250 (5%)

### File Counts
- **Source Files**: 13
- **Test Files**: 4
- **Documentation Files**: 10
- **Configuration Files**: 10
- **Workflow Files**: 3
- **Template Files**: 3

### Dependencies
- **Direct Dependencies**: 25 crates
- **Dev Dependencies**: 1 (tempfile)
- **Total (with transitive)**: ~100+ crates

### Language Distribution
- **Rust**: 95%
- **YAML**: 2%
- **Markdown**: 2%
- **Shell/PowerShell**: 1%

## 🎯 Key Features by File

### `src/main.rs`
- CLI argument parsing with clap
- Command routing
- Status display
- Configuration management

### `src/config.rs`
- TOML-based configuration
- Version tracking
- Installation management
- Persistence layer

### `src/detector.rs`
- System scanning (Windows/macOS/Linux)
- Java version parsing
- LTS detection
- Architecture detection

### `src/downloader.rs`
- Adoptium API integration
- Async downloads with progress
- Checksum verification
- Archive extraction (ZIP/TAR.GZ)

### `src/path_manager.rs`
- Windows Registry manipulation
- Unix shell configuration
- Environment variable management
- PATH updates

## 🔧 Build Outputs

### Debug Build
- **Binary Size**: ~15-20 MB
- **Build Time**: ~2-3 minutes (first build)
- **Incremental**: ~10-30 seconds

### Release Build
- **Binary Size**: ~5-8 MB (stripped)
- **Build Time**: ~3-5 minutes
- **Optimizations**: LTO, single codegen unit

## 📦 Release Artifacts

For each release, the following files are generated:

| Artifact | Size | Platform |
|----------|------|----------|
| `jaman-windows-x64.exe.zip` | ~3 MB | Windows x64 |
| `jaman-macos-x64.tar.gz` | ~2.5 MB | macOS Intel |
| `jaman-macos-arm64.tar.gz` | ~2.3 MB | macOS Apple Silicon |
| `jaman-linux-x64.tar.gz` | ~2.5 MB | Linux x64 |
| `jaman-linux-arm64.tar.gz` | ~2.3 MB | Linux ARM64 |
| `*.sha256` | <1 KB | Checksums for all |

## 🌟 Notable Features

### Code Quality
- ✅ Formatted with `rustfmt`
- ✅ Linted with `clippy`
- ✅ Type-safe throughout
- ✅ Error handling with `anyhow`
- ✅ Comprehensive tests

### Documentation Quality
- ✅ Inline code documentation
- ✅ Usage examples
- ✅ Quick start guide
- ✅ Contributing guidelines
- ✅ Security policy

### CI/CD Quality
- ✅ Multi-platform testing
- ✅ Automated releases
- ✅ Security audits
- ✅ Documentation deployment
- ✅ Version management

## 🚀 Getting Started

1. **Clone Repository**
   ```bash
   git clone https://github.com/TheusHen/jaman.git
   cd jaman
   ```

2. **Build Project**
   ```bash
   cargo build --release
   ```

3. **Run Tests**
   ```bash
   cargo test
   ```

4. **Install Locally**
   ```bash
   cargo install --path .
   ```

5. **Use It**
   ```bash
   jaman --version
   ```

## 📝 File Maintenance

### Regular Updates Needed
- `CHANGELOG.md` - After each release
- `Cargo.toml` - Version bumps
- `README.md` - Feature additions
- Dependencies - Monthly security updates

### Generated Files (Don't Edit)
- `Cargo.lock` - Auto-generated by cargo
- `target/` - Build artifacts
- Generated docs in `target/doc/`

### One-Time Setup
- `LICENSE` - Rarely changes
- `CODE_OF_CONDUCT.md` - Stable
- `.editorconfig` - Stable
- `rustfmt.toml` - Stable

## 🔗 References

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Clap Documentation](https://docs.rs/clap/)
- [Tokio Documentation](https://tokio.rs/)
- [Adoptium API](https://api.adoptium.net/)

---

**Last Updated**: November 22, 2025
