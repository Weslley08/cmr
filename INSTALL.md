# CMR installation

The CMR project is a **native Rust CLI**, not a Claude Code plugin. You do not need a marketplace.

## Build from source (today)

Install Rust stable and Git:

```sh
git clone https://github.com/Weslley08/cmr.git
cd cmr
cargo test --all-targets
cargo build --release
```

Move `target/release/cmr` (or Windows `target/release/cmr.exe`) into an existing PATH directory, or add its containing directory to your user PATH. Restart terminals and VS Code after changing PATH.

## Set up native Claude Code hooks

```sh
cmr init -g --dry-run
cmr init -g
cmr status
cmr doctor
```

The installer edits only `~/.claude/settings.json`, retaining all existing hooks. It backs up the original settings as `settings.json.cmr.bak` before its first modification, and will not overwrite the first backup on subsequent runs. The added entry is:

```json
{
  "hooks": {
    "UserPromptSubmit": [
      {
        "hooks": [
          { "type": "command", "command": "cmr hook prompt", "timeout": 10 }
        ]
      }
    ]
  }
}
```

This is **merged**, not used to replace existing settings. RTK's separate `PreToolUse` Bash hook remains untouched.

## How each integration behaves

- **Claude Code CLI**: `cmr launch "prompt"` chooses a model for a **new** session and runs `claude --model <alias> "prompt"`, if the CLI is installed.
- **VS Code Claude Code extension**: native `UserPromptSubmit` hook runs, displaying an advisory model recommendation. There is no documented native hook that forces a per-prompt switch. Use the extension's model selector or `/model` for actual changes.
- **RTK**: still runs independently before compatible Bash tool calls.

## Planned package channels

WinGet `Weslley08.CMR`, Homebrew `weslley08/tap/cmr` and signed release assets are **planned**, not yet available.

## Uninstall

```sh
cmr uninstall -g --dry-run
cmr uninstall -g
```

This removes only the CMR hook. It does not remove RTK or Claude Code settings.
