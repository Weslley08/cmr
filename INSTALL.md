# Installation (CMR 0.2.0 experimental)

## Claude Code VS Code extension — recommended

Requires Claude Code 2.1.309 or later for reliable model alias routing.

1. Open the official Claude Code extension panel.
2. In `/plugin`, add `Weslley08/cmr` as a marketplace.
3. Install `model-router@cmr` at **user scope**.
4. Use `/reload-plugins` or restart the session.
5. Run `/cmr status`; then `/cmr mode balanced` and `/cmr auto`.
6. Try read-only and code-change prompts; use `/cmr stats` to inspect observed model families.

This installs a plugin from your own GitHub repository; it is not the Visual Studio Marketplace and does not require the standalone Claude Code CLI.

## Claude Code CLI (if available)

```sh
claude plugin marketplace add Weslley08/cmr
claude plugin install model-router@cmr --scope user
```

To develop and validate from a local clone:

```sh
claude plugin validate ./plugins/model-router
claude plugin test ./plugins/model-router
claude --plugin-dir ./plugins/model-router
```

## Optional Rust installer

Build from source until GitHub Releases are available:

```sh
git clone https://github.com/Weslley08/cmr.git
cd cmr
cargo test --all-targets
cargo build --release
```

Add the produced `target/release` directory to PATH, then:

```sh
cmr init -g --dry-run
cmr init -g
cmr doctor
cmr status
```

`cmr init` delegates to the official Claude Code plugin manager; **it does not inject a UserPromptSubmit hook** anymore. If `claude` is not found, it prints VS Code instructions and exits without installing. No Rust CLI is needed for VS Code setup.

## RTK

RTK is optional; it uses its own `PreToolUse` hook. CMR does not uninstall or patch RTK. To diagnose RTK separately, run `rtk init --show` and `rtk gain`.

## Disable, uninstall, rollback

In Claude Code, run `/cmr off` to disable routing until the session ends. Disable or uninstall `model-router@cmr` via `/plugin`; the CLI equivalent is:

```sh
claude plugin uninstall model-router@cmr --scope user
```

Existing legacy advisory hooks from CMR 0.1.0 are not removed automatically during installation. Remove such duplicates with care and preserve your RTK hook.

## Distribution

`winget install Weslley08.CMR` and `brew install weslley08/tap/cmr` are planned, not yet published.
