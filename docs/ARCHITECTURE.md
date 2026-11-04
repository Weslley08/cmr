# CMR architecture (0.2.0)

```
Claude Code local session (VS Code / terminal)
  │
  ├─ CMR Mod (plugins/model-router)
  │    ├─ prompt.submit: deterministic local classification
  │    ├─ turn.step: next({...e, model}) for main conversation
  │    ├─ command.run: /cmr settings and session counters
  │    └─ session.start: register /cmr
  │
  ├─ RTK (optional, independent)
  │    └─ PreToolUse Bash: compact command output
  │
  └─ Claude permission system, subagents and other hooks remain unchanged

Rust CLI
  ├─ init/uninstall: delegate to Claude plugin manager
  ├─ doctor/status: diagnostics
  ├─ launch: initial --model choice for CLI sessions
  └─ recommend: heuristic classification only
```

## Limits

- Model selection is heuristic, with no additional classifier LLM call.
- One chosen family is sticky across a submitted main-conversation prompt.
- Subagents are left to Claude Code (agentId).
- The visual model picker may not reflect per-request model routing.
- Token and cache counters are per session and are not a total-cost or subscription quota estimate.
- Actual execution with the Claude Code 2.1.318 extension must be tested before a stable release.
- A Mod runs inside Claude Code with user permissions; review the source before installation.

See [official Claude Code Mod documentation](https://code.claude.com/docs/en/plugins/mods).
