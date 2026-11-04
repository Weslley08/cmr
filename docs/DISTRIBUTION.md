# Distribution: GitHub Releases, WinGet and Homebrew

The GitHub repository already exposes the **Mod marketplace**, which works without the Rust CLI.

## GitHub Actions releases

1. Merge the version changes after CI passes.
2. Bump `Cargo.toml` and `plugins/model-router/.claude-plugin/plugin.json`.
3. Push an annotated tag `vX.Y.Z`.
4. `release.yml` builds Windows x64, Linux x64/ARM64, and macOS x64/ARM64 native CLI binaries.
5. The publishing job creates a GitHub Release and a `SHA256SUMS` file.

Publishing is deliberately gated by a tag and successful build jobs. Nothing here means a release already exists.

## WinGet

After the first GitHub release, obtain the actual SHA256 of `cmr-x86_64-pc-windows-msvc.zip` from `SHA256SUMS`. On Windows:

```powershell
./scripts/generate-winget.ps1 -Version "0.1.0" -Sha256 "<ACTUAL_64_HEX_SHA256>"
winget validate --manifest "./packaging/winget/Weslley08.CMR/0.1.0"
```

Submit the three generated YAML manifests to `microsoft/winget-pkgs`. **Approval is external**. The ID `Weslley08.CMR` is proposed and not registered merely by this repository.

## Homebrew

After a release, use the release archives and verified SHA256s to populate a formula in your future `homebrew-tap` repository. `packaging/homebrew/README.md` documents a minimal tap formula. The tap does not yet exist, and `brew install weslley08/tap/cmr` is not currently available.

## Supply-chain and release discipline

- GitHub Actions upload/download artifacts are the only inputs to the release step.
- Release SHA256SUMS is generated from those exact artifacts.
- Workflow permissions are read-only except for the release creation job.
- A WinGet manifest must contain the genuine artifact checksum; never use placeholders for publication.
- Signed builds, SBOM and formal provenance are future enhancements; not currently claimed.
