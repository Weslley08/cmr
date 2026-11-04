# Quick start

## In VS Code (no standalone CLI)

```text
/plugin marketplace add Weslley08/cmr
/plugin install model-router@cmr
/reload-plugins
/cmr status
/cmr mode balanced
/cmr auto
```

Use the *user* install scope for all local projects. Run `/cmr off` to stop routing for the current session.

## With a standalone Rust CLI (optional)

```sh
cmr init -g --dry-run
cmr init -g
cmr status
cmr doctor
```

The CLI's `init` delegates to the official Claude Code plugin manager. If the `claude` executable is missing, follow the VS Code steps above; no native hook can substitute for the Mod.
