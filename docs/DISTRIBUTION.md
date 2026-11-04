# Distribution plan

CMR is **two distributable components**: a native Rust CLI and a Claude Code Mod housed in the same GitHub repository.

## Mod (already hosted as source)

The repo root includes `.claude-plugin/marketplace.json`; the Mod lives in `plugins/model-router`.
Users can install via the official Claude Code manager:
```text
/plugin marketplace add Weslley08/cmr
/plugin install model-router@cmr
```
No WinGet package or standalone Claude CLI is required for a local VS Code extension installation. This is a GitHub plugin catalog, not a paid store.

## Rust CLI binary (not released)

GitHub Actions is configured to build Windows x64, Linux x64/ARM64 and macOS Intel/Apple Silicon native releases with checksums. A tagged release has not yet been published.

## WinGet

After a real tagged release, run `scripts/generate-winget.ps1 -Version X.Y.Z -Sha256 <ACTUAL_RELEASE_SHA256>`, validate and submit the manifests to `microsoft/winget-pkgs`. The desired package ID `Weslley08.CMR` is **not registered** yet.

## Homebrew

Publish an updated formula in a separate `Weslley08/homebrew-tap` repository using verified SHA256 hashes from actual release archives. This tap does **not** exist yet.

Keep Mod manifest and Cargo package versions aligned before tagging. Mod unit tests and marketplace validation are release blockers; live Claude Code routing is still awaiting validation.
