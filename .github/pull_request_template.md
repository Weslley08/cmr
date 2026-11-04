## Changes

## Verification
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets -- -D warnings`
- [ ] `cargo test --all-targets`
- [ ] Windows, Linux and macOS native builds

## Safety
- [ ] No plugin, Mod, marketplace or undocumented API
- [ ] Existing RTK and Claude Code hooks preserved
- [ ] Advisory recommendations are not misrepresented as actual model switches
- [ ] No user prompt content or secrets logged externally
