# Core

Pure CLI argument parsing (`cli.rs`) and cross-platform environment adapters (`environment.rs`). No prompt interception or model selection occurs in this Rust crate; routing belongs to the Claude Code Mod in `plugins/model-router/`.

Follow RTK's `src/core` convention without introducing its compression functionality into CMR.
