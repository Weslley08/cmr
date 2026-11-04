# Coding practices

- Keep one self-contained Rust executable; no plugin, Mod, marketplace or external classification API.
- Use `serde_json` to preserve existing Claude Code settings.
- Add tests for hook installation, idempotency and RTK coexistence.
- Never claim native hooks can switch the current model per prompt.
- Distinguish `cmr recommend` and `cmr launch`.
- CI must pass format, clippy, tests, builds and secret scanning.
- Do not log prompt content to disk or send telemetry.
