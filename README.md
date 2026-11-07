# CMR — Claude Model Router

<p align="center"><strong>Automatic Haiku / Sonnet / Opus routing in Claude Code, with a cross-platform Rust CLI.</strong></p>

<p align="center">
<a href="https://github.com/Weslley08/cmr/actions/workflows/ci.yml"><img alt="CI" src="https://github.com/Weslley08/cmr/actions/workflows/ci.yml/badge.svg"></a>
<a href="https://github.com/Weslley08/cmr/releases"><img alt="Releases" src="https://img.shields.io/github/v/release/Weslley08/cmr"></a>
<a href="LICENSE"><img alt="MIT" src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
</p>

<p align="center"><a href="README_pt.md">Português</a> · <a href="INSTALL.md">Install</a> · <a href="docs/contributing/ARCHITECTURE.md">Architecture</a> · <a href="CHANGELOG.md">Changelog</a></p>

> **Experimental 0.2.2 source (not a published binary release).** The native Claude Code Mod can actually select a different model per main-conversation model request through the documented `turn.step` event. End-to-end operation in a real Claude Code session is not yet verified. No WinGet or Homebrew package has been published.

## What CMR does

- **Auto-route** scoped read-only prompts to Haiku, ordinary development to Sonnet and complex/critical work to Opus.
- Uses official Claude Code **Mods** capabilities (`prompt.submit` + `turn.step`) without Anthropic API keys, an external proxy or an extra classifier model call.
- Commands: `/cmr status`, `/cmr auto`, `/cmr off`, `/cmr mode economy|balanced|quality`, `/cmr force haiku|sonnet|opus|none`, `/cmr stats`.
- Uses one choice per prompt for the main conversation, leaving subagent model selection alone. Session-local usage counters (not a savings guarantee).
- **0.2.1:** distinguishes selected model aliases from provider-reported models (unknown when the host does not report a model); avoids misclassifying read-only prompts such as *do not modify anything* as code changes.
- **RTK remains optional and independent**. Its `PreToolUse` Bash hook is not modified.

**Caveat:** this is a rule-based router, not a model trained to predict task difficulty; cache invalidation and quality regressions can outweigh savings. The active IDE model picker may not display the per-request model. Available models remain constrained by your Claude account and provider.

## Install in VS Code (no standalone CLI required)

In the **official Claude Code extension** open `/plugin` and register our GitHub repository as a marketplace:

```text
/plugin marketplace add Weslley08/cmr
/plugin install model-router@cmr
```

Select **user** scope to enable it in all local projects. Use `/reload-plugins` or restart the Claude Code session. Then:

```text
/cmr status
/cmr mode balanced
/cmr auto
```

Routing is enabled by default once the plugin is loaded. `/cmr off` opts out for the current session.

The word *marketplace* here simply means a GitHub-hosted plugin catalog: **not a paid store or VS Code Marketplace listing**.

## Optional Rust CLI (RTK-style)

```sh
git clone https://github.com/Weslley08/cmr.git
cd cmr
cargo test --all-targets
cargo build --release
```

Add `target/release/` to PATH. The CLI manages the plugin when the standalone Claude Code `claude` command exists:

```sh
cmr init -g --dry-run
cmr init -g
cmr doctor
cmr status
cmr recommend "List files in src"
cmr launch "Implement an endpoint and tests"
cmr uninstall -g
```

When `claude` is **not** in PATH (as with some VS Code-only installs), `cmr init -g` prints the VS Code installation instructions and does not claim success. The old `cmr hook prompt` advisory command remains for backwards compatibility but is no longer installed by default.

After a tested GitHub Release, we intend to publish WinGet and Homebrew packages. The planned identifiers `Weslley08.CMR` and `weslley08/tap/cmr` **are not yet available**.

## Architecture

```text
cmr/
├── .claude-plugin/marketplace.json   # GitHub plugin catalog
├── plugins/model-router/
│   ├── .claude-plugin/plugin.json     # Mod manifest
│   ├── hooks/
│   │   ├── hooks.json                # Register one JS module
│   │   ├── policy.mjs                # No-network policy
│   │   └── register.mjs              # prompt.submit, turn.step, /cmr
│   └── tests/                        # JavaScript unit tests
├── src/
│   ├── main.rs
│   ├── core/                         # CLI and legacy advisory
│   ├── cmds/                         # Installation and diagnostics
│   └── analytics/
├── hooks/                            # Legacy hook documentation
├── tests/                            # Rust integration tests
├── docs/ and examples/
├── scripts/
├── packaging/
└── .github/workflows/
```

## Development and verification

Requirements: Rust stable; Node.js 22+ for Mod unit tests. Claude Code 2.1.309+ for model aliases in `turn.step`.

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
node --test plugins/model-router/tests/*.test.mjs
node scripts/check-mod.mjs
```

If you have the standalone `claude` binary, validate the **actual Mod API** in your installed build:

```sh
claude plugin validate plugins/model-router
claude plugin test plugins/model-router
claude --plugin-dir ./plugins/model-router
```

These tests are not a substitute for observing actual model selection and token usage in a Claude Code session.

## License and security

MIT licensed. Review [SECURITY.md](SECURITY.md) before installing third-party Mods. The Mod can run with your Claude Code user's permissions, although CMR currently makes no network requests or filesystem edits. 
