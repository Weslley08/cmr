# Security policy

## Supported versions

CMR v0.1.0 is experimental and not yet a supported production release.

## Reporting vulnerabilities

Please open a private security advisory at:
https://github.com/Weslley08/cmr/security/advisories/new

Do not post credentials, private prompts, or exploit details in public GitHub issues.

## Trust model

- The Mod runs with the permissions of your Claude Code session. Read its source before enabling it.
- CMR does not elevate permission modes, disable RTK, or modify tool-approval policy.
- No API keys, model proxy, or external classifier is configured.
- The CLI delegates installation to the official Claude Code plugin manager. It does not write settings directly.
- The GitHub Actions release requires passing build jobs.
- Avoid running scripts from untrusted forks or unsigned community release assets without verification.
