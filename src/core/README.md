# Core

- `cli.rs` parses commands.
- `routing.rs` classifies local prompts for the Rust `recommend` and `launch` CLI commands.
- `settings.rs` safely maintains the **legacy** advisory hook and remains for backward compatibility.
- `environment.rs` resolves local OS paths and invokes `claude --model` for new `cmr launch` sessions.

The actual automatic per-request model switch now lives in `plugins/model-router/hooks/register.mjs`. Normal `cmr init -g` no longer writes a native UserPromptSubmit hook.
