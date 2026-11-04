# CMR development conventions

- Implement a single native Rust executable, no Claude Code plugin/Mod or marketplace.
- Use official Claude Code native hooks; `UserPromptSubmit` cannot change models.
- `cmr launch` can choose the model at CLI session startup only.
- Preserve `rtk hook claude`, all other existing hooks, and user security settings.
- Run Rust format, clippy, tests and native platform builds as blocking checks.
- Do not hide failures or invent savings metrics.
- Keep English and Portuguese READMEs consistent.
