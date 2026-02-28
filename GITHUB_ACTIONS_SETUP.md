# GitHub Actions Setup Summary

This document summarizes the GitHub Actions CI/CD setup for the RTPL project.

## What Was Set Up

### 1. Main CI Workflow (`.github/workflows/ci.yml`)

A comprehensive continuous integration workflow that runs on every push to `main` and on all pull requests.

**Jobs:**

1. **Code Quality** - Runs on Ubuntu
   - Checks code formatting with `rustfmt`
   - Runs linting with `clippy` (all warnings treated as errors)
   
2. **Test Suite** - Runs on Ubuntu
   - Executes all unit tests
   - Runs documentation tests
   
3. **Build** - Multi-platform matrix build
   - **Linux x86_64** (`x86_64-unknown-linux-gnu`) on Ubuntu
   - **macOS Apple Silicon** (`aarch64-apple-darwin`) on macOS (M1/M2/M3)
   - Produces optimized release binaries
   - Strips debug symbols **only for tagged releases** (e.g., `v1.0.0`)
   - Uploads artifacts (automatically zipped by GitHub)

4. **Release Check** - Runs on `main` branch pushes only
   - Downloads and verifies all build artifacts
   - Useful for future automated release workflows

### 2. Quick Build Workflow (`.github/workflows/makefile.yml`)

A simplified workflow for manual testing that uses the project's Makefile. Triggered manually via `workflow_dispatch`.

### 3. Configuration Files

- **`rustfmt.toml`**: Code formatting rules (stable options only)
- **`clippy.toml`**: Linting configuration and thresholds
- **`.github/workflows/README.md`**: Detailed workflow documentation

### 4. Documentation Updates

- Added CI badge to main `README.md`
- Added development and CI/CD sections
- Created comprehensive workflow documentation

## Features

### Caching
All workflows use intelligent caching to speed up builds:
- Cargo registry cache
- Cargo git dependencies cache
- Build artifacts cache
- Cache keys based on OS, target, and `Cargo.lock` hash

### Artifacts
Build artifacts are automatically uploaded and available for 90 days:
- `rtpl-linux-x86_64.zip` - Linux binary (~7.6 MB with symbols, ~6.7 MB stripped)
- `rtpl-macos-aarch64.zip` - macOS Apple Silicon binary (~7.6 MB with symbols, ~6.7 MB stripped)

**Note:** Binaries from `main` branch builds include debug symbols for easier debugging. Tagged releases (e.g., `v1.0.0`) are stripped for smaller downloads.

### Quality Gates
- Code must be properly formatted (`cargo fmt`)
- No clippy warnings allowed
- All tests must pass
- Builds must succeed for all platforms

## How to Use

### Viewing Workflow Status

1. Go to the [Actions tab](https://github.com/fabiomontefuscolo/rtpl/actions)
2. Click on any workflow run to see details
3. View logs for each job
4. Download build artifacts from the bottom of the workflow page

### Downloading Build Artifacts

From any successful workflow run:
1. Scroll to the "Artifacts" section
2. Download the appropriate `.zip` file
3. Extract and use:
   ```bash
   unzip rtpl-linux-x86_64.zip
   chmod +x rtpl
   ./rtpl --help
   ```

### Running Checks Locally

Before pushing, ensure your code passes all checks:

```bash
# Check and fix formatting
cargo fmt --all

# Verify formatting (what CI runs)
cargo fmt --all -- --check

# Run clippy (what CI runs)
cargo clippy --all-targets --all-features -- -D warnings

# Run tests (what CI runs)
cargo test --locked --verbose
cargo test --doc --locked --verbose

# Build release binary locally
cargo build --release --locked
```

### Manual Workflow Trigger

To manually run the quick build workflow:
1. Go to Actions → Quick Build (Makefile)
2. Click "Run workflow"
3. Select the branch
4. Click "Run workflow"

## CI Workflow Behavior

### On Pull Requests
- All jobs run (code-quality, test, build)
- Must pass before merging
- Build artifacts available for testing

### On Push to Main
- All jobs run
- Build artifacts uploaded **with debug symbols** (not stripped)
- Release check verifies artifact structure

### On Version Tags (e.g., v1.0.0)
- All jobs run
- Build artifacts uploaded **stripped** (smaller size)
- Ready for distribution to end users

### On Other Branches
- Only runs if you open a PR from that branch

## Platform Support

### Current Platforms
- ✅ **Linux x86_64** - Standard Intel/AMD 64-bit Linux
- ✅ **macOS Apple Silicon** - M1, M2, M3, and future M-series chips

### Potential Future Platforms
To add more platforms, edit `.github/workflows/ci.yml` and add entries to the build matrix:

```yaml
- os: windows-latest
  target: x86_64-pc-windows-msvc
  name: windows-x86_64
  
- os: ubuntu-latest
  target: aarch64-unknown-linux-gnu
  name: linux-aarch64
```

## Troubleshooting

### If Code Quality Fails

**Formatting issues:**
```bash
cargo fmt --all
git add -u
git commit -m "Fix formatting"
```

**Clippy warnings:**
```bash
cargo clippy --all-targets --all-features -- -D warnings
# Fix the issues reported
git commit -am "Fix clippy warnings"
```

### If Tests Fail

Run tests locally with verbose output:
```bash
cargo test --locked --verbose
```

Review the failures and fix the code or tests as needed.

### If Build Fails

Check the specific platform's build log in the Actions tab. Common issues:
- Missing dependencies
- Platform-specific code issues
- Cargo.lock out of sync (run `cargo update`)

### Creating a Release

To create an official release with stripped binaries:

```bash
# Create and push a version tag
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

The workflow will automatically run and produce stripped binaries optimized for distribution.

### Cache Issues

If builds are failing due to corrupted cache:
1. Go to Settings → Actions → Caches
2. Delete the problematic cache
3. Re-run the workflow

## Next Steps and Improvements

### Recommended Next Steps

1. **Set up branch protection rules**
   - Require CI to pass before merging
   - Require code review
   - Settings → Branches → Add rule for `main`

2. **Configure Dependabot** (already supported by GitHub)
   - Automatic dependency updates
   - Security vulnerability alerts

3. **Add code coverage** (optional)
   - Tools like `cargo-tarpaulin` or `cargo-llvm-cov`
   - Upload to Codecov or Coveralls

4. **Set up automated releases** (optional)
   - Create releases automatically on version tags
   - Attach build artifacts to releases

### Potential Workflow Enhancements

- [ ] Add security audit with `cargo audit`
- [ ] Add benchmarking suite
- [ ] Create release workflow for tags
- [ ] Add Windows x86_64 support
- [ ] Add Linux ARM64 support
- [ ] Add code coverage reporting
- [ ] Add performance regression testing
- [ ] Set up nightly builds

## Files Created/Modified

### New Files
- `.github/workflows/ci.yml` - Main CI workflow
- `.github/workflows/README.md` - Workflow documentation
- `rustfmt.toml` - Formatting configuration
- `clippy.toml` - Linting configuration
- `GITHUB_ACTIONS_SETUP.md` - This file

### Modified Files
- `.github/workflows/makefile.yml` - Updated to manual trigger only
- `README.md` - Added CI badge and development sections

## Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [actions-rust-lang/setup-rust-toolchain](https://github.com/actions-rust-lang/setup-rust-toolchain)
- [Rust CI Best Practices](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
- [rustfmt Configuration](https://rust-lang.github.io/rustfmt/)
- [clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

## Status

✅ **Ready to use!** The CI/CD pipeline is fully configured and operational.

All local tests pass:
- ✅ Code formatting check passed
- ✅ Clippy linting passed (no warnings)
- ✅ All 10 tests passed

The workflows will run automatically on your next push or pull request.