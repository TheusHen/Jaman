# Contributing to Jaman

First off, thank you for considering contributing to Jaman! It's people like you that make Jaman such a great tool.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Commit Message Guidelines](#commit-message-guidelines)

## Code of Conduct

This project and everyone participating in it is governed by our commitment to providing a welcoming and inspiring community for all.

### Our Standards

- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples**
- **Describe the behavior you observed and what you expected**
- **Include screenshots if applicable**
- **Include your environment details** (OS, Rust version, jaman version)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- **Use a clear and descriptive title**
- **Provide a detailed description of the suggested enhancement**
- **Explain why this enhancement would be useful**
- **List some examples of how it would be used**

### Your First Code Contribution

Unsure where to begin? You can start by looking through `good-first-issue` and `help-wanted` issues:

- **Good first issues** - issues that should only require a few lines of code
- **Help wanted issues** - issues that may be more involved

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code lints
6. Issue that pull request!

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- Git

### Setup Steps

```bash
# 1. Fork and clone the repository
git clone https://github.com/YOUR_USERNAME/jaman.git
cd jaman

# 2. Create a new branch
git checkout -b feature/my-new-feature

# 3. Install dependencies (automatic with cargo)
cargo build

# 4. Run tests
cargo test

# 5. Run the application
cargo run -- --help
```

### Development Workflow

```bash
# Format your code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Build release version
cargo build --release
```

## Pull Request Process

1. **Update Documentation**: Ensure any new features or changes are reflected in README.md
2. **Add Tests**: All new functionality should include appropriate tests
3. **Run All Checks**: 
   ```bash
   cargo fmt -- --check
   cargo clippy -- -D warnings
   cargo test --all-features
   ```
4. **Update CHANGELOG**: Add your changes under "Unreleased" section
5. **One Feature Per PR**: Keep pull requests focused on a single feature or fix
6. **Descriptive Title**: Use a clear and descriptive PR title
7. **Link Issues**: Reference any related issues in your PR description

### PR Title Format

Use conventional commit format:

- `feat: Add support for Oracle JDK`
- `fix: Resolve Windows PATH issue`
- `docs: Update installation instructions`
- `test: Add tests for detector module`
- `refactor: Simplify download logic`
- `perf: Optimize version detection`
- `chore: Update dependencies`

## Coding Standards

### Rust Style Guide

We follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/). Key points:

- **Use `rustfmt`**: All code must be formatted with `cargo fmt`
- **Use `clippy`**: Address all clippy warnings with `cargo clippy`
- **Naming Conventions**:
  - `snake_case` for functions, methods, variables
  - `CamelCase` for types, traits
  - `SCREAMING_SNAKE_CASE` for constants
- **Documentation**: Public APIs must have doc comments
- **Error Handling**: Use `Result` and `anyhow` for error propagation

### Code Organization

```rust
// 1. Imports (grouped and sorted)
use anyhow::Result;
use std::path::PathBuf;

// 2. Constants
const MAX_RETRIES: u32 = 3;

// 3. Type definitions
pub struct MyStruct {
    field: String,
}

// 4. Implementations
impl MyStruct {
    pub fn new(field: String) -> Self {
        Self { field }
    }
}

// 5. Tests (in separate module)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        // Test code
    }
}
```

### Documentation

All public items should have documentation comments:

```rust
/// Downloads a Java distribution from the specified URL.
///
/// # Arguments
///
/// * `url` - The URL to download from
/// * `dest` - The destination path
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the download fails.
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
/// 
/// let url = "https://example.com/java.zip";
/// let dest = PathBuf::from("./java.zip");
/// download_file(url, &dest).await?;
/// ```
pub async fn download_file(url: &str, dest: &PathBuf) -> Result<()> {
    // Implementation
}
```

## Testing Guidelines

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptive_name() {
        // Arrange
        let input = "test";
        
        // Act
        let result = function_to_test(input);
        
        // Assert
        assert_eq!(result, expected_value);
    }

    #[tokio::test]
    async fn test_async_function() {
        // Test async code
    }
}
```

### Test Coverage

- **Unit Tests**: Test individual functions and methods
- **Integration Tests**: Test component interactions
- **Edge Cases**: Test boundary conditions and error cases
- **Platform-Specific**: Use `#[cfg(target_os = "...")]` for OS-specific tests

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests for specific module
cargo test config::tests
```

## Commit Message Guidelines

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Type

- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, etc.)
- **refactor**: Code refactoring
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Maintenance tasks

### Examples

```
feat(downloader): Add support for GraalVM downloads

Implement functionality to download and install GraalVM distributions
from the official repository. Includes checksum verification and
platform-specific handling.

Closes #123
```

```
fix(path): Resolve Windows PATH update issue

Fixed issue where PATH wasn't being updated correctly on Windows 11
due to registry permission problems. Now uses proper API calls.

Fixes #456
```

## Project Structure

Understanding the codebase:

```
jaman/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── config.rs            # Configuration management
│   ├── detector.rs          # Java detection logic
│   ├── downloader.rs        # Download and installation
│   ├── path_manager.rs      # PATH management
│   └── commands/            # Command implementations
│       ├── list.rs
│       ├── install.rs
│       ├── activate.rs
│       ├── scan.rs
│       ├── doctor.rs
│       └── clean.rs
├── tests/                   # Integration tests
├── .github/
│   └── workflows/           # CI/CD workflows
└── docs/                    # Additional documentation
```

## Questions?

Feel free to open an issue with the `question` label if you have any questions about contributing!

## Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- GitHub contributors page

Thank you for contributing to Jaman! 🎉
