# Configuration

Install the `model-router@cmr` Mod using Claude Code's own plugin manager. Its settings apply to the current local session:

```text
/cmr status
/cmr mode balanced
/cmr auto
/cmr force opus
/cmr force none
/cmr stats
/cmr off
```

The Mod selects a model when a `turn.step` request begins; `prompt.submit` provides a deterministic local classification. It does not overwrite Claude Code's `settings.json` or manipulate the RTK `PreToolUse` hook. It leaves subagents unchanged.

The old native `UserPromptSubmit` hook (0.1.0) is advisory only and no longer installed by `cmr init`. The standalone `cmr launch` still passes `--model` when starting a **new** CLI session.
