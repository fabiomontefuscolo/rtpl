# GitHub Actions Workflows

This directory contains the CI/CD workflows for the RTPL project.

## Overview

The project uses GitHub Actions for automated testing, code quality checks, and building release binaries for multiple platforms.

## Workflows

### CI Workflow (`ci.yml`)

**Trigger:** Runs on every push to `main`, on all pull requests, and on version tags (`v*`).

The main CI workflow consists of four jobs:

#### 1. Code Quality

Ensures code meets quality standards:

- **Formatting Check**: Uses `cargo fmt` to verify code is properly formatted
- **Linting**: Runs `clippy` with all warnings treated as errors (`-D warnings`)
- **Platform**: Ubuntu latest

```bash
# Run locally
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

#### 2. Test Suite

Runs the complete test suite:

- **Unit Tests**: Runs all tests with `cargo test --locked`
- **Doc Tests**: Validates documentation examples with `cargo test --doc`
- **Platform**: Ubuntu latest

```bash
# Run locally
cargo test --locked --verbose
cargo test --doc --locked --verbose
```

#### 3. Build

Creates release binaries for supported platforms:

**Platforms:**
- **Linux x86_64** (`x86_64-unknown-linux-gnu`)
  - Runs on: `ubuntu-latest`
  - Target: Intel/AMD 64-bit processors
  
- **macOS Apple Silicon** (`aarch64-apple-darwin`)
  - Runs on: `macos-latest` (M-series chips)
  - Target: Apple Silicon (M1, M2, M3, etc.)

**Build Process:**
1. Builds release binary with `--release --locked`
2. Strips debug symbols **only for tagged releases** (e.g., `v1.0.0`) to reduce binary size
   - Development builds (push to `main`) keep symbols for easier debugging
   - Release builds (tags) strip symbols for smaller downloads
3. Uploads binary as GitHub Actions artifact (automatically zipped by GitHub)

**Artifacts:**
- `rtpl-linux-x86_64.zip` (~7.6 MB with symbols, ~6.7 MB stripped)
- `rtpl-macos-aarch64.zip` (~7.6 MB with symbols, ~6.7 MB stripped)

**Note:** Artifacts from `main` branch builds include debug symbols. Artifacts from tagged releases are stripped.

#### 4. Release Check

Runs only on pushes to `main` branch:

- Downloads all build artifacts
- Verifies artifact structure
- Useful for debugging release pipelines

## Quick Build Workflow (`makefile.yml`)

**Trigger:** Manual only (`workflow_dispatch`)

A simplified workflow that uses the project's Makefile for quick builds and tests. Useful for manual testing and debugging.

```bash
# What it runs
make prepare
make test
make build
```

## Caching Strategy

All workflows use GitHub Actions caching to speed up builds:

- **Cargo Registry Cache**: Caches downloaded crate metadata
- **Cargo Git Cache**: Caches git dependencies
- **Build Cache**: Caches compiled dependencies

Cache keys are based on:
- Operating system
- Target platform (for builds)
- `Cargo.lock` hash

## Accessing Build Artifacts

### From Pull Requests or Commits

1. Go to the [Actions tab](https://github.com/fabiomontefuscolo/rtpl/actions)
2. Click on the workflow run
3. Scroll down to "Artifacts" section
4. Download the appropriate tarball for your platform

### Extract and Use

```bash
# Linux
unzip rtpl-linux-x86_64.zip
chmod +x rtpl
./rtpl --help

# macOS
unzip rtpl-macos-aarch64.zip
chmod +x rtpl
./rtpl --help
```

## Dependencies

The workflows use the following GitHub Actions:

- `actions/checkout@v4`: Checkout repository code
- `actions-rust-lang/setup-rust-toolchain@v1`: Install Rust toolchain
- `actions/cache@v4`: Cache dependencies
- `actions/upload-artifact@v4`: Upload build artifacts
- `actions/download-artifact@v4`: Download build artifacts

## Local Development

To run the same checks locally before pushing:

```bash
# Check formatting
cargo fmt --all -- --check

# Fix formatting
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --locked --verbose

# Build for your platform
cargo build --release --locked

# Cross-compile (requires target installed)
rustup target add x86_64-unknown-linux-gnu
cargo build --release --locked --target x86_64-unknown-linux-gnu
```

## Adding New Platforms

To add support for additional platforms:

1. Edit `.github/workflows/ci.yml`
2. Add a new entry to the `matrix.platform` array in the `build` job:

```yaml
- os: windows-latest
  target: x86_64-pc-windows-msvc
  name: windows-x86_64
```

3. Adjust the strip steps for Windows if needed (Windows uses `.exe` extension)
4. Update this README and the main README.md

## Troubleshooting

### Build Fails on macOS

- Ensure you're using `macos-latest` runners which have Apple Silicon
- The `aarch64-apple-darwin` target is only available on M-series macs

### Creating a Release

To create an official release with stripped binaries:

```bash
# Tag the release
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

The workflow will automatically run and produce stripped binaries for the release.

### Cache Issues

If you suspect cache corruption:

1. Go to Actions → Caches
2. Delete the relevant cache
3. Re-run the workflow

### Clippy Warnings

All clippy warnings are treated as errors. To fix:

```bash
cargo clippy --all-targets --all-features --fix
```

### Format Issues

```bash
cargo fmt --all
```

## Future Improvements

Potential enhancements to the CI/CD pipeline:

- [ ] Add code coverage reporting
- [ ] Add security audit (`cargo audit`)
- [ ] Add dependency update checks (Dependabot)
- [ ] Create GitHub Releases automatically on tags
- [ ] Add benchmarking
- [ ] Add Windows x86_64 support
- [ ] Add ARM Linux support