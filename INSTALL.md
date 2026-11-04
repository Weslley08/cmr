# CMR installation guide

> **Preview only:** No WinGet/Homebrew package or signed CLI release is published yet. The plugin's live runtime compatibility remains unverified.

## VS Code — official Claude Code extension

1. In the Claude Code extension, open its plugin manager (the command may be `/plugin` or `/plugins`, depending on the version).
2. Add `Weslley08/cmr` as a Claude Code plugin **marketplace source**.
3. Install `model-router@weslley08-cmr` for your user.
4. Restart the session and run `/cmr status`, `/cmr mode balanced`, and `/cmr auto`.
5. If anything fails, disable/uninstall the plugin. The Mod does not uninstall RTK.

If you have standalone Claude Code CLI:

```sh
claude plugin marketplace add Weslley08/cmr
claude plugin install model-router@weslley08-cmr
```

## Build the optional Rust CLI

Install Rust and Git:

```sh
git clone https://github.com/Weslley08/cmr.git
cd cmr
cargo test --all-targets
cargo build --release
./target/release/cmr doctor
```

Windows PowerShell uses `./target/release/cmr.exe doctor`. The CLI invokes official `claude plugin` commands; if `claude` is unavailable, it provides VS Code instructions instead of touching settings.

## Future native install channels

- Windows: `winget install Weslley08.CMR` **only after** Microsoft accepts the manifest.
- macOS/Linux: `brew install weslley08/tap/cmr` **only after** a Homebrew tap exists and is published.
- GitHub Releases: downloadable verified CLI archives **after** a tag and successful build/release workflow.

None of these should be advertised as available today.

## Remove the Mod

Uninstall `model-router@weslley08-cmr` using the plugin manager, or:

```sh
claude plugin uninstall model-router@weslley08-cmr
```

CMR does not modify RTK's `PreToolUse` hook.
