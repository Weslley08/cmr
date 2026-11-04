# Security policy

CMR 0.2.0 is experimental. Report vulnerabilities privately using [GitHub security advisories](https://github.com/Weslley08/cmr/security/advisories/new).

## Trust

The Claude Code Mod executes in Claude Code with user privileges. Review `plugins/model-router/hooks/register.mjs` and `policy.mjs` before installing.

CMR currently:
- Makes **no** external classifier API requests.
- Does not read tokens or credential files.
- Does not run shell commands from the Mod.
- Does not override tool approvals, security policy, or subagent models.
- Does not install, remove, or modify RTK.
- Maintains counters only in memory for the active session.

Model routing cannot guarantee better quality or savings. Use `/cmr off` or disable the Mod in `/plugin` to regain the default Claude Code selection.

Supply-chain signing and end-to-end tests are still outstanding before a stable public release.
