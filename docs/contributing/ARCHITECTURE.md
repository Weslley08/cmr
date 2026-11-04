# CMR internals

CMR has two independent parts:

1. **Rust CLI** under `src/core`, `src/cmds`, and `src/analytics`, similar to RTK's repository organization. It provides installation through Claude's plugin manager, diagnostics, recommendations and CLI session launch.
2. **Claude Code Mod** under `plugins/model-router`, which actually selects per-request model aliases via the documented `turn.step` event. A GitHub marketplace manifest at the root allows user-scope installation.

`UserPromptSubmit` (old Rust hook) is legacy, **advisory only**, and is no longer installed by `cmr init -g`.

No proxy, captured credentials, privileged process, external classifier or change to the RTK hook is required. Ensure the Mod loads and the expected model really processes each request before making optimization claims.

## Testing

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
node --test plugins/model-router/tests/*.test.mjs
node scripts/check-mod.mjs
```

On a machine with Claude Code CLI, additionally run `claude plugin validate ./plugins/model-router` and `claude plugin test ./plugins/model-router`. The GitHub CI only mocks hook contracts; it does not send requests to Anthropic.
