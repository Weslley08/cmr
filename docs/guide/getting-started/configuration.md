# Configuration

The CMR binary registers one global `UserPromptSubmit` hook in Claude Code's `settings.json`. It does **not** modify `PreToolUse`, the current model, `model` settings, authorization, or RTK. Model classification is currently a fixed local heuristic. Changes to the active model must be performed with `/model` in Claude Code or through `cmr launch` for a new CLI session.
