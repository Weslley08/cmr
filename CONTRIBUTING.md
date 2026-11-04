# Contributing

CMR combines a cross-platform Rust CLI and a Claude Code Mod, installed via a GitHub marketplace. Changes must preserve permissions and independent RTK hooks.

Run locally:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release
node --check plugins/model-router/hooks/register.mjs
node --test plugins/model-router/tests/*.test.mjs
node scripts/check-mod.mjs
```

If the Claude Code CLI is available, also run `claude plugin validate plugins/model-router` and `claude plugin test plugins/model-router`; use `claude --plugin-dir ./plugins/model-router` for an integration test in an interactive session.

A Mod runs with the current user's permissions. No network classifier calls, secrets handling, tool permission overrides or bypassing security checks. Keep the cache and subagent behavior explicit, document model-routing limitations, and never claim measured savings without controlled benchmarks.

Open a PR with CI results for Windows, macOS and Linux before a stable release.
