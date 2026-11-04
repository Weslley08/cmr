# CMR integrations

CMR does not need an RTK-like shell `PreToolUse` hook to choose models.

Its Claude Code Mod hooks are under `plugins/model-router/hooks/` and use the experimental `prompt.submit` / `turn.step` event API. RTK independently uses `rtk hook claude` for command output.

No shell hooks are installed directly by the CMR CLI; existing RTK configuration remains untouched.
