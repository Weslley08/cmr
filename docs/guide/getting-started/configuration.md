# Configuration

CMR currently stores settings only within the running Mod session.

| Profile | Behavior |
|---|---|
| economy | Prioritize Haiku for simple read-only queries; Sonnet for work; Opus for critical cases |
| balanced | Haiku for scoped reads, Sonnet for routine implementation, Opus for complex or critical tasks |
| quality | Sonnet for general queries and Opus for implementation/complex work |

Use `/cmr mode economy`, `/cmr mode balanced`, or `/cmr mode quality`. Run `/cmr auto` to opt in and `/cmr off` to stop. Manual overrides: `/cmr force haiku`, `/cmr force sonnet`, `/cmr force opus`, `/cmr force none`.

CMR cannot bypass Claude Code model entitlements, existing tool permissions or organization policy.
