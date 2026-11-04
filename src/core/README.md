# Core

`cli.rs` parses commands; `routing.rs` classifies prompts locally; `settings.rs` merges `UserPromptSubmit` hooks without replacing RTK; `environment.rs` resolves local OS paths and invokes `claude --model` only for `cmr launch`.
