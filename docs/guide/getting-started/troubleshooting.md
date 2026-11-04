# Troubleshooting

- **`claude` not recognized in PowerShell:** The VS Code extension can be used without the standalone CLI. Use its plugin manager.
- **`cmr` not recognized:** There is no published CLI package yet; build from source if you want the CLI.
- **Mod not loading:** Verify the Claude Code runtime version and inspect sanitized logs; mocked unit tests are not a guarantee.
- **No routing:** Start with `/cmr status`, then `/cmr auto` and submit a new prompt. Check actual model selection.
- **Unexpected model:** Heuristics can be wrong. Use `/cmr force sonnet` or `/cmr off`.
- **RTK missing:** RTK is separate; verify with `rtk init --show`. CMR does not rewrite RTK hooks.
- **`rtk gain` not changing:** RTK measures output compaction; it does not count CMR routing.
