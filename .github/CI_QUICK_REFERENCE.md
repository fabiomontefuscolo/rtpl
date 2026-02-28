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
Push/PR → Code Quality → Tests → Build → Artifacts
           ├─ rustfmt      ├─ unit    ├─ Linux x86_64
           └─ clippy       └─ doc     └─ macOS ARM64
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
- `rtpl-linux-x86_64.zip`
- `rtpl-macos-aarch64.zip`

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

---

**Need help?** Check the full docs in `.github/workflows/README.md`
