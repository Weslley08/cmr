# CMR CLI commands

- `init`: register GitHub marketplace and install the Mod for the current user through the official Claude Code CLI, if available.
- `uninstall`: uninstall only the Mod; RTK and unrelated hooks are not modified.
- `doctor`: inspect availability and legacy hook status.
- `status`: list installed plugins and detect legacy advisory hooks.
- `launch`: choose the first model when launching a new CLI session with `--model`.
- `hook`: retained compatibility command for the obsolete advisory-only native hook.

The model-routing logic that actually changes live requests is inside the Mod, not the Rust CLI.
