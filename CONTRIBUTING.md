# Contributing

CMR is a native Rust CLI and Claude Code hook binary. **Do not add a plugin, marketplace, unsupported API proxy or credential interception.**

Run:
```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release
```

Preserve RTK and other user hooks, keep the installer idempotent and reversible, avoid sending prompts to any third-party service, and distinguish advisory selection from actual model switches in all documentation. Never claim measured token savings without benchmarks. Check Linux/macOS/Windows CI before release.
