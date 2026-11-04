# Coding practices

- Keep `src/main.rs` minimal and feature code in `src/core`, `src/cmds`, and `plugins/model-router`.
- Prefer pure routing decisions with tests and explicit fallbacks.
- Never turn off existing Claude Code permissions or RTK hooks.
- Keep unit tests, Rust format/lint, Mod syntax and marketplace checks mandatory.
- Avoid credentials, prompt telemetry, secret logging and network classifiers.
- Do not claim savings or successful host routing without measured evidence.
- Package manager examples must be marked planned until accepted and published.
