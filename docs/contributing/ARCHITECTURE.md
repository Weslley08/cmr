# Native CMR architecture

```text
Windows / macOS / Linux
  cmr (single Rust CLI)
    ├─ src/core/routing.rs       local recommendations
    ├─ src/core/settings.rs      safe hook config merge/uninstall
    ├─ src/cmds/launch.rs        claude --model <recommendation> "<prompt>"
    ├─ src/cmds/hook.rs          UserPromptSubmit stdin → systemMessage
    └─ src/cmds/doctor.rs        PATH + hook diagnostics

Claude Code CLI and VS Code extension
  ├─ CMR UserPromptSubmit native hook (first)
  └─ optional RTK PreToolUse Bash hook (only on tool execution)
```

There is no plugin runtime, marketplace, proxy server or Anthropic credential integration.

**Critical distinction:** the hook shows advice; it cannot actually switch the model. Only `cmr launch` can choose a session's initial model via `claude --model`, and that requires standalone CLI access. This deliberate limitation avoids undocumented hacks and misleading claims.

The settings editor preserves unrelated hooks, including RTK. Unit tests verify registration/removal without mutating real user files.
