# CMR security policy

CMR is experimental. Report security vulnerabilities through a private [GitHub security advisory](https://github.com/Weslley08/cmr/security/advisories/new).

The Rust executable processes Claude Code hook JSON **locally**, does not send prompts to external services, does not need an API key, and must not read or modify authentication tokens. `cmr init` updates only the native `UserPromptSubmit` hook in the user's settings and backs up the file. It must not overwrite RTK or unrelated hooks.

Review the source before installing; native hooks execute with your user permissions. No automatic per-turn model switch is claimed.
