# Native hook integration

**CMR uses the same native hook configuration system as RTK, but a different event.** No Claude Code plugin or marketplace.

- `cmr hook prompt`: `UserPromptSubmit` — receives a prompt, applies local rules and returns a **recommendation-only** `systemMessage`.
- `rtk hook claude`: `PreToolUse` matcher `Bash` — later intercepts compatible tool calls.

Claude Code CLI and the local VS Code Claude Code extension use the global `~/.claude/settings.json` native hook registry.

Official hooks do **not** expose a field to overwrite the active model at `UserPromptSubmit`. `PreModelSwitch` can allow/deny a requested switch, not originate one. Use `cmr launch` to set an initial model for a new CLI session.
