# Legacy native hook

CMR 0.1.0 used `UserPromptSubmit` to display model recommendations. That setting hook **cannot actually change the active model**.

As of 0.2.0, the default is the **Claude Code Mod** under `plugins/model-router`. It changes `turn.step` requests before Claude Code sends them, using the official API.

The old `cmr hook prompt` CLI command remains available for users of the original experimental version, but `cmr init -g` does **not** register it.

RTK's separate `PreToolUse` hook is preserved. No native shell hook can replace the Mod's `turn.step` functionality.
