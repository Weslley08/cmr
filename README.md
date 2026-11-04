# CMR — Claude Model Router

[![CI](https://github.com/Weslley08/cmr/actions/workflows/ci.yml/badge.svg)](https://github.com/Weslley08/cmr/actions/workflows/ci.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Experimental v0.1.0** — model selection for Claude Code, with a lightweight cross-platform CLI and a native **Mod**. CMR does not need the Anthropic API key, send prompts to a third-party classifier or change RTK settings.

> **Status:** Code, tests and GitHub Actions are in this repository. Real routing in Claude Code 2.1.318 and the VS Code extension must be validated before calling this a stable release. WinGet and Homebrew listings are **not published yet**.

## Features

- Rule-based selection between **Haiku**, **Sonnet** and **Opus** via Claude Code's documented `turn.step` event.
- Modes: `economy`, `balanced` (default), and `quality`.
- **Off by default**. Run `/cmr auto` to enable. `/cmr off` disables routing.
- Explicit model overrides with `/cmr force sonnet` (or haiku/opus/none).
- Preserves subagent model choices, Claude Code permissions and existing **RTK** hooks.
- No classifier API, token, telemetry collection, background agent, or shell interception.

The rule engine is heuristic. It **does not currently** detect a failed build and automatically escalate the model, guarantee superior quality or prove subscription quota savings. Session counters are approximate.

## Install directly in the VS Code extension (no CLI required)

1. In the **official Claude Code** panel, open `/plugin` (some versions expose `/plugins`).
2. Add marketplace: `Weslley08/cmr`.
3. Install `model-router@weslley08-cmr` for **user** scope.
4. Restart/reload the Claude Code session.
5. Run `/cmr status`, `/cmr mode balanced`, then `/cmr auto`.

Alternatively, if the Claude Code CLI is available:

```sh
claude plugin marketplace add Weslley08/cmr
claude plugin install model-router@weslley08-cmr
```

The GitHub repository itself is the Claude Code marketplace. A Rust executable is **not required** to install the Mod.

## CMR CLI

The optional Rust CLI helps install and diagnose CMR; the model routing itself executes in the Claude Code Mod. After a release is published, use the matching binary from [GitHub Releases](https://github.com/Weslley08/cmr/releases).

For developers with Rust installed:

```sh
cargo build --release
cargo run -- init -g
cargo run -- doctor
cargo run -- status
```

`cmr init -g` uses the **official Claude Code plugin commands** and user scope. If `claude` is missing, it prints VS Code installation instructions rather than silently editing your Claude configuration.

### Distribution roadmap

| OS | Planned channel | State |
|---|---|---|
| Windows x64 | GitHub Releases + WinGet `Weslley08.CMR` | CI/build configured; WinGet approval pending first release |
| macOS Intel/Apple Silicon | GitHub Releases + Homebrew tap | CI/build configured; tap not published |
| Linux x64/ARM64 | GitHub Releases `.tar.gz` | CI/build configured; install script planned |

A WinGet package cannot be installed by `winget install Weslley08.CMR` until the corresponding manifest has been accepted into the community index. Do **not** treat that command as currently available.

## Commands inside Claude Code

```
/cmr status
/cmr mode economy
/cmr mode balanced
/cmr mode quality
/cmr auto
/cmr force opus
/cmr force none
/cmr stats
/cmr off
```

Routing decisions are per user prompt and sticky across the model requests for that prompt. CMR skips subagents. A more advanced escalation engine is a proposed future release.

## RTK compatibility

RTK is **optional**, independent of the CMR Mod. RTK's `PreToolUse` Bash hook compacts command output; CMR's `turn.step` hook chooses a model. Neither component installs, uninstalls or modifies the other.

## Development

Requirements: stable Rust toolchain; Node.js 20+.

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
node --test plugins/model-router/tests/*.test.js
node scripts/check-marketplace.mjs
```

Run the authoritative Claude Code validation locally with its CLI, if installed:

```sh
claude plugin validate .
claude plugin validate plugins/model-router
claude plugin test plugins/model-router
```

Unit tests test the local routing logic, **not** a live Claude Code subscription or model invocation.

## Releases and package managers

Tag releases as `vX.Y.Z` after updating `Cargo.toml` and the Mod's `plugin.json`. GitHub Actions builds native archives and generates `SHA256SUMS` after its package jobs succeed. See [Distribution](docs/DISTRIBUTION.md).

## Security

Routing is opt-in; no outgoing requests are made from the Mod. It does not bypass tool permissions, change existing hooks or copy prompts to external services. See [SECURITY.md](SECURITY.md) before installing third-party mods.

## License

[MIT](LICENSE).
