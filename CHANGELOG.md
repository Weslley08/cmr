# Changelog

## [Unreleased]

### Changed
- Replaced the experimental Claude Code Mod/plugin and marketplace with a **native Rust hook executable**.
- Separate advisory classification from effective CLI model selection (`cmr launch`).
- Added safe, idempotent JSON hook installation that preserves RTK `PreToolUse`.

### Removed
- Plugin marketplace, JavaScript Mod, `/cmr` slash commands and mocked Mod routing claims.

## [0.1.0] — initial experimental source baseline

- Cross-platform Rust CLI, local task classification, optional RTK coexistence.
- CI and release workflow scaffolding. No package-manager listing published yet.
