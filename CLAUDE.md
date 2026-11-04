# CMR contributor instructions

- CMR consists of a Rust CLI and an opt-in Claude Code Mod.
- Keep CLI, model routing and RTK hook responsibilities separate.
- Do not modify user settings or bypass Claude Code permissions.
- Run `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `node --test plugins/model-router/tests/*.test.js`, and `node scripts/check-marketplace.mjs`.
- Never claim the Mod is fully functional in production before live tests.
- Update README.md, README_pt.md, INSTALL.md and CHANGELOG.md with behavior changes.
