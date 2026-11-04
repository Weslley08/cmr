# Architecture

```text
Claude Code (CLI / VS Code / supported desktop sessions)
  ├─ CMR model-router Mod (Claude's mod runtime)
  │    ├─ prompt.submit  → deterministic local classifier
  │    ├─ turn.step      → model alias for main conversation
  │    └─ /cmr           → opt-in mode / force / in-session metrics
  ├─ RTK (optional)     → PreToolUse Bash output compaction
  └─ tool permissions, policies, subagents: unchanged

Optional cmr Rust CLI
  ├─ init      → claude plugin marketplace add/install
  ├─ doctor    → PATH diagnostics
  ├─ status    → claude plugin list
  └─ uninstall → claude plugin uninstall
```

## Guarantees and boundaries

- No LLM classifier call or extra classifier token expense.
- Default mode is **off**, requiring `/cmr auto` per new session.
- Model selection is heuristic based on the latest prompt, not full conversation understanding.
- `turn.step` may include multiple model requests per prompt. The same choice is applied to each.
- Non-main agent requests are left unchanged when `e.agentId` is present.
- Mod logs stay in the Claude Code runtime; counters are not persistent across sessions.
- The Mod does not mutate `settings.json` or intercept shell commands.
- Claude Code remains responsible for request compatibility, model availability and permissions.
- Cache effects can offset savings; compare equivalent tasks and quality before claiming savings.

## Failure handling

If the Mod is uninstalled, disabled or `/cmr off` is active, Claude Code chooses the model.
Unsupported model aliases or changes in the Mods API must be tested against the installed Claude Code version.
Do not silence test failures or skip security checks based on model selection.

## Future work

- Routing based on tool results and task failure signals with safe caps
- Optional persistent, privacy-aware metrics
- Cache-aware model stickiness and per-task budgets
- End-to-end Claude Code Mod test suite under the official test runner
- Empirical benchmark against a fixed model baseline
