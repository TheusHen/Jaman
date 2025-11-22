# GitHub Workflows Documentation

This directory contains GitHub Actions workflows for Jaman's CI/CD pipeline.

## 📋 Workflows Overview

### 1. CI (Continuous Integration) - `ci.yml`

**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` or `develop`

**Jobs:**

#### `test`
Runs on: `ubuntu-latest`, `windows-latest`, `macos-latest`

Steps:
1. Checkout code
2. Install Rust (stable)
3. Cache cargo dependencies
4. Check code formatting (`cargo fmt -- --check`)
5. Run clippy linter (`cargo clippy`)
6. Run tests (`cargo test`)
7. Build release binary

#### `coverage`
Runs on: `ubuntu-latest`

Steps:
1. Generate code coverage using `cargo-tarpaulin`
2. Upload to Codecov

#### `security`
Runs on: `ubuntu-latest`

Steps:
1. Run security audit (`cargo audit`)

#### `docs`
Runs on: `ubuntu-latest`

Steps:
1. Generate documentation (`cargo doc`)
2. Deploy to GitHub Pages (if on main branch)

**Status Badge:**
```markdown
[![CI](https://github.com/TheusHen/jaman/workflows/CI/badge.svg)](https://github.com/TheusHen/jaman/actions/workflows/ci.yml)
```

---

### 2. Release - `release.yml`

**Triggers:**
- Git tags matching `v*` (e.g., `v0.1.0`)
- Manual workflow dispatch

**Jobs:**

#### `build`
Runs on: Multiple platforms

**Build Matrix:**

| OS | Target | Output |
|----|--------|--------|
| Windows | `x86_64-pc-windows-msvc` | `jaman-windows-x64.exe` |
| macOS | `x86_64-apple-darwin` | `jaman-macos-x64` |
| macOS | `aarch64-apple-darwin` | `jaman-macos-arm64` |
| Linux | `x86_64-unknown-linux-gnu` | `jaman-linux-x64` |
| Linux | `aarch64-unknown-linux-gnu` | `jaman-linux-arm64` |

Steps per platform:
1. Checkout code
2. Install Rust with target
3. Install cross-compilation tools (if needed)
4. Cache dependencies
5. Build release binary
6. Strip binary (Unix only)
7. Create archive (ZIP for Windows, TAR.GZ for Unix)
8. Upload artifact

#### `create-release`
Runs on: `ubuntu-latest`
Depends on: `build`

Steps:
1. Download all build artifacts
2. Generate SHA256 checksums
3. Create GitHub Release
4. Upload all artifacts and checksums

**Release Assets:**
- Binary archives for all platforms
- SHA256 checksums
- Automated release notes with install instructions

#### `publish-crates`
Runs on: `ubuntu-latest`
Depends on: `create-release`

Steps:
1. Publish to crates.io (optional, requires `CARGO_TOKEN` secret)

**Status Badge:**
```markdown
[![Release](https://github.com/TheusHen/jaman/workflows/Release/badge.svg)](https://github.com/TheusHen/jaman/actions/workflows/release.yml)
```

**Creating a Release:**

```bash
# 1. Update version in Cargo.toml
# version = "0.2.0"

# 2. Update CHANGELOG.md
# Add release notes

# 3. Commit changes
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"

# 4. Create and push tag
git tag v0.2.0
git push origin main
git push origin v0.2.0

# 5. GitHub Actions will automatically:
#    - Build for all platforms
#    - Create GitHub Release
#    - Upload binaries
#    - Publish to crates.io (if configured)
```

---

### 3. Documentation - `docs.yml`

**Triggers:**
- Push to `main` branch (only for `**.md` or `src/**` changes)
- Manual workflow dispatch

**Jobs:**

#### `build-docs`
Runs on: `ubuntu-latest`

Steps:
1. Checkout code
2. Install Rust
3. Generate Rust docs (`cargo doc`)
4. Create redirect index.html
5. Setup GitHub Pages
6. Upload documentation artifact

#### `deploy`
Runs on: `ubuntu-latest`
Depends on: `build-docs`

Steps:
1. Deploy to GitHub Pages

**Access Documentation:**
After deployment, docs are available at:
`https://TheusHen.github.io/jaman/`

---

## 🔐 Required Secrets

Configure these in your GitHub repository settings:

| Secret | Purpose | Required For |
|--------|---------|--------------|
| `GITHUB_TOKEN` | Automatically provided by GitHub | All workflows |
| `CARGO_TOKEN` | Publish to crates.io | Release workflow (optional) |
| `CODECOV_TOKEN` | Upload coverage reports | CI workflow (optional) |

**Setting Secrets:**
1. Go to: `Settings` → `Secrets and variables` → `Actions`
2. Click `New repository secret`
3. Add secret name and value

---

## 🎯 Workflow Best Practices

### For CI Workflow

**Before Pushing:**
```bash
# Local checks (same as CI)
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test --all-features
cargo build --release
```

**Fixing CI Failures:**
```bash
# Format code
cargo fmt

# Fix clippy warnings
cargo clippy --fix

# Run tests
cargo test
```

### For Release Workflow

**Pre-Release Checklist:**
- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] All tests pass locally
- [ ] Documentation is up to date
- [ ] No uncommitted changes
- [ ] On `main` branch

**Troubleshooting Releases:**

If build fails:
1. Check GitHub Actions logs
2. Test cross-compilation locally:
   ```bash
   rustup target add x86_64-pc-windows-msvc
   cargo build --target x86_64-pc-windows-msvc
   ```
3. Fix issues and create new tag

If release is missing artifacts:
1. Check workflow logs for failed jobs
2. Re-run failed jobs from GitHub Actions UI
3. If necessary, delete tag and recreate

---

## 📊 Monitoring Workflows

### View Workflow Runs
1. Go to `Actions` tab in GitHub
2. Select workflow from left sidebar
3. View recent runs

### Status Checks
All workflows appear as status checks on PRs:
- ✅ Green check = passed
- ❌ Red X = failed
- 🟡 Yellow dot = running

### Notifications
Configure in: `Settings` → `Notifications` → `Actions`

Options:
- Email on workflow failure
- GitHub notifications
- Slack/Discord webhooks (requires setup)

---

## 🛠️ Local Testing

### Test CI Checks Locally

```bash
# Install act (https://github.com/nektos/act)
# macOS
brew install act

# Windows
choco install act-cli

# Run workflows locally
act push
act pull_request
```

### Simulate Release Build

```bash
# Install cross for cross-compilation
cargo install cross

# Build for Windows
cross build --release --target x86_64-pc-windows-gnu

# Build for Linux
cross build --release --target x86_64-unknown-linux-gnu
```

---

## 🔧 Customization

### Adding New Platforms

Edit `release.yml`:

```yaml
matrix:
  include:
    - os: ubuntu-latest
      target: arm-unknown-linux-gnueabihf
      artifact_name: jaman
      asset_name: jaman-linux-armv7
```

### Adding New CI Checks

Edit `ci.yml`:

```yaml
jobs:
  new-job:
    name: My New Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run my check
        run: ./my-script.sh
```

### Caching Strategy

Current cache keys:
- `${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}`
- `${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}`
- `${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}`

To clear cache:
1. Update `Cargo.lock`
2. Or manually delete from GitHub Actions cache settings

---

## 📚 Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Cargo Build Documentation](https://doc.rust-lang.org/cargo/)
- [Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [GitHub Releases API](https://docs.github.com/en/rest/releases)

---

## 🆘 Support

If workflows fail or you need help:
1. Check workflow logs in GitHub Actions
2. Read error messages carefully
3. Search existing issues
4. Open new issue with workflow run link

---

**Last Updated**: November 22, 2025
