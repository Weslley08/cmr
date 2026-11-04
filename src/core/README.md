# Core

- `cli.rs` parses commands.
- `routing.rs` classifies local prompts for the Rust `recommend` and `launch` CLI commands.
- `settings.rs` detects the **legacy** advisory hook in read-only mode; CMR no longer modifies settings.json.
- `environment.rs` resolves local OS paths and invokes `claude --model` for new `cmr launch` sessions.

The actual automatic per-request model switch now lives in `plugins/model-router/hooks/register.mjs`. Normal `cmr init -g` no longer writes a native UserPromptSubmit hook.
