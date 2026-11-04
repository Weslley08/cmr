# Architecture

Structure inspired by [RTK](https://github.com/rtk-ai/rtk) without reusing RTK's compression implementation:

```text
src/main.rs              Thin native Rust CLI entry point
src/core/                Argument parsing and OS/process helpers
src/cmds/                Commands: init, doctor, status, uninstall
src/analytics/           Usage/metric help
plugins/model-router/    Claude Code Mod, local classification, simulated hook tests
.claude-plugin/          Claude Code plugin marketplace catalog
hooks/                   Integration notes, separate from RTK PreToolUse
tests/                   Rust binary end-to-end tests
docs/guide/              User docs
examples/                Policy examples
scripts/                 Project and release validation
.github/workflows/       CI, native builds and release automation
```

**CLI:** delegates plugin installation to official Claude Code commands; it never edits user settings directly.

**Mod:** opt-in classification at `prompt.submit` and attempted model override at `turn.step`. Rules are local and deterministic. Subagent requests are left to Claude Code.

**RTK:** separate optional `PreToolUse` command-output filter, not a dependency or an alternative CMR implementation.

The initial Mod integration has not been validated in a real VS Code/Claude Code session; do not confuse mocked tests with host compatibility.
