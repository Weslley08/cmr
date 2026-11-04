# Distribution plan

CMR is a standalone Rust executable. The repository configures Windows x64, Linux x64/ARM64 and macOS Intel/Apple Silicon native release builds and checksums.

## Tagging and GitHub Releases

After all tests pass, tag `vX.Y.Z` matching Cargo.toml. The release pipeline builds native archives and publishes assets with SHA256SUMS. A release has not yet been published.

## WinGet

After a real tagged release, run `scripts/generate-winget.ps1 -Version X.Y.Z -Sha256 <ACTUAL_RELEASE_SHA256>` and validate/submit the manifest to `microsoft/winget-pkgs`. The desired package ID `Weslley08.CMR` is **not registered** yet.

## Homebrew

Publish a formula in a separate `Weslley08/homebrew-tap` repository using verified SHA256s from real GitHub release archives. The tap does **not** exist yet.

No Mod, plugin marketplace or VS Code extension dependency.
