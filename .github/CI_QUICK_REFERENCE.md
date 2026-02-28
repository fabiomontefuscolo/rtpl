# CI/CD Quick Reference Card

## 🚀 Pre-Push Checklist

Run these commands locally before pushing:

```bash
# 1. Format code
cargo fmt --all

# 2. Run linter
cargo clippy --all-targets --all-features -- -D warnings

# 3. Run tests
cargo test --locked

# 4. Build (optional)
cargo build --release --locked
```

## 📋 CI Workflow Overview

```
Push/PR/Tag → Code Quality → Tests → Build → Artifacts
              ├─ rustfmt      ├─ unit    ├─ Linux x86_64
              └─ clippy       └─ doc     └─ macOS ARM64

Note: Binaries stripped only on tags (e.g., v1.0.0)
```

## ✅ Status Badge

[![CI](https://github.com/fabiomontefuscolo/rtpl/actions/workflows/ci.yml/badge.svg)](https://github.com/fabiomontefuscolo/rtpl/actions/workflows/ci.yml)

## 🔧 Quick Fixes

### Format Issues
```bash
cargo fmt --all
```

### Clippy Warnings
```bash
cargo clippy --all-targets --all-features --fix --allow-dirty
```

### Update Dependencies
```bash
cargo update
```

## 📦 Build Artifacts

After CI runs, download from Actions → Workflow Run → Artifacts:
- `rtpl-linux-x86_64.zip` (~7.6 MB with symbols, ~6.7 MB stripped)
- `rtpl-macos-aarch64.zip` (~7.6 MB with symbols, ~6.7 MB stripped)

**Note:** Main branch builds include debug symbols. Tagged releases are stripped.

Extract and run:
```bash
unzip rtpl-*.zip
./rtpl --help
```

## 🎯 Supported Platforms

| Platform | Target | Runner |
|----------|--------|--------|
| Linux x86_64 | `x86_64-unknown-linux-gnu` | `ubuntu-latest` |
| macOS Apple Silicon | `aarch64-apple-darwin` | `macos-latest` |

## 🔍 Common Issues

| Problem | Solution |
|---------|----------|
| Format check fails | Run `cargo fmt --all` |
| Clippy warnings | Fix warnings or add `#[allow(...)]` |
| Tests fail | Run `cargo test --verbose` locally |
| Cache issues | Delete cache in Settings → Actions |
| Need stripped binary | Create a version tag (e.g., `v1.0.0`) |

## 📚 Files

| File | Purpose |
|------|---------|
| `.github/workflows/ci.yml` | Main CI workflow |
| `rustfmt.toml` | Format rules |
| `clippy.toml` | Lint config |

## 🔗 Links

- [Workflows](https://github.com/fabiomontefuscolo/rtpl/actions)
- [Workflow Docs](.github/workflows/README.md)
- [Setup Guide](../GITHUB_ACTIONS_SETUP.md)

## ⚡ Manual Trigger

Go to: Actions → Quick Build (Makefile) → Run workflow

## 🏷️ Creating a Release

To create a stripped release build:

```bash
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

This triggers CI with stripped binaries (smaller downloads).

---

**Need help?** Check the full docs in `.github/workflows/README.md`
