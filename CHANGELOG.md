# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Auto PATH registration**: Jaman now automatically adds itself to the system PATH on first run
- **Configurable download directory**: New `download_dir` configuration option for custom download locations
- **Enhanced system scanning**: 
  - Scans all available disk drives on Windows (not just C:)
  - Detects Java installations via `java -version` command
  - Uses `where java` (Windows) / `which java` (Unix) to find installations in PATH
- New command option: `jaman config --set-download-dir` to configure download directory
- Initial release of Jaman
- Multi-version Java management support
- Download and install Java from Eclipse Adoptium
- Auto-detection of existing Java installations
- Interactive version selection with beautiful CLI
- PATH and JAVA_HOME management
- System diagnostics with `doctor` command
- Cleanup unused installations with `clean` command
- LTS version filtering
- Progress bars and animations for downloads
- Cross-platform support (Windows, macOS, Linux)
- Configuration management
- Comprehensive test suite
- CI/CD workflows for automated builds

### Changed
- Improved `jaman scan` to search more thoroughly across system
- Updated configuration structure to include `download_dir`

### Fixed
- Better detection of Java installations in non-standard locations

## [0.1.0] - 2025-11-22

### Added
- Initial beta release
- Core functionality for Java version management
- Command-line interface with multiple commands
- Documentation and build instructions

[Unreleased]: https://github.com/TheusHen/jaman/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/TheusHen/jaman/releases/tag/v0.1.0
