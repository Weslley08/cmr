# Troubleshooting

- **No `cmr` command**: compile it from Rust source and add its executable directory to PATH; reopen VS Code.
- **Hook missing**: run `cmr status` then `cmr init -g`.
- **RTK not working**: independently verify `rtk init --show`; CMR doesn't modify its `PreToolUse` hook.
- **Model did not change**: expected in VS Code. Native hooks only recommend a model. Select `/model` manually, or use `cmr launch` with standalone CLI.
- **No `claude` command**: VS Code extension can run without CLI; `cmr launch` specifically needs the standalone CLI.
- **Invalid settings JSON**: CMR refuses to overwrite it; inspect and repair it manually before retrying.
