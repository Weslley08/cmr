# Contributing to CMR

Thanks for contributing! CMR aims to remain lightweight, cross-platform, safe and transparent.

1. Fork the repository and create a feature branch.
2. Write tests for classification, routing and failure/fallback behavior.
3. Run `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `node --test plugins/model-router/tests/*.test.js`, and `node scripts/check-marketplace.mjs`.
4. Avoid dependencies that require collecting user prompts, credentials or personal data.
5. Keep model override opt-in and preserve existing permissions and RTK hooks.
6. Open a PR with the full test results and version notes.

Do not bypass checks to force a release. Real Claude Code integration tests must be distinguished from mock unit tests.
