# Changelog

## [Unreleased]

## 0.2.2 — Unreleased (source)

### Fixed
- Prevent negated or quoted model names in prompts from acting as explicit overrides; only `/cmr force` requests a manual model.
- Fix Gitleaks CI scans on pull requests by checking out the required commit history.
- Update GitHub Actions used in CI and release workflows to Node 24-capable major versions and verify artifact upload/download round trips.

### Changed
- Version metadata synchronized for Rust CLI and Claude Code Mod to allow updating installed plugins without publishing a binary release.

## 0.2.1 — Unreleased (source)

### Fixed
- Negated write instructions (for example `Não modifique nada`) no longer disqualify simple read-only prompts from Haiku.
- Session statistics distinguish selected model aliases from models explicitly reported by the host; missing provider model is marked unverified instead of inferred.
- Added regression tests for read-only negations, mixed instructions, and unreported models.


### Changed
- CMR now supports real per-request model routing through a Claude Code Mod.
- Rust `cmr init -g` installs the Mod through Claude Code's plugin manager, rather than the old advisory hook.
- Adds a GitHub-hosted plugin marketplace, Mod tests and CI validation.
- Keeps legacy advisory hook code for backwards compatibility only.

### Known limitations
- Claude Code VS Code end-to-end model routing has not yet been verified.
- No public binary release, WinGet manifest approval or Homebrew tap release.

## 0.1.0
- Initial Rust CLI, advisory UserPromptSubmit hook and source-only cross-platform builds.
