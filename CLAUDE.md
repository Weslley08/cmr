# CMR development conventions

- Maintain **both** the native Rust CLI and the official Claude Code Mod at `plugins/model-router`.
- The Mod handles per-request routing through `prompt.submit` and `turn.step`; native settings hooks alone cannot select a model.
- The Rust CLI installs through the official Claude Code plugin manager; when the Claude CLI is absent, provide VS Code installation instructions without claiming success.
- Preserve `rtk hook claude`, all user hooks, permission decisions and security settings.
- Avoid dynamic imports, network classifiers, credential interception, proxies or unapproved model APIs.
- Do not override subagents' model choices or silently hide request errors.
- Run Rust format, clippy, tests, Mod syntax tests, policy tests, manifests and native platform builds as blocking checks.
- Distinguish mocked Mod tests from live Claude Code request validation.
- Do not claim measured savings or broad IDE compatibility without tests.
- Keep English and Portuguese READMEs consistent; bump Cargo and plugin versions together.
