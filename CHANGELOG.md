# Changelog

## [Unreleased]

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
