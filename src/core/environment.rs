//! External tool integration. Never edits Claude settings directly.
use std::env;
use std::path::PathBuf;
use std::process::Command;

pub const MARKETPLACE: &str = "Weslley08/cmr";
pub const PLUGIN: &str = "model-router@weslley08-cmr";

pub fn available(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

pub fn run(program: &str, arguments: &[&str]) -> bool {
    match Command::new(program).args(arguments).status() {
        Ok(status) => status.success(),
        Err(error) => {
            eprintln!("Cannot start {program}: {error}");
            false
        }
    }
}

pub fn home_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
}

pub fn print_vscode_instructions() {
    println!("Claude Code CLI not found. For the VS Code extension:");
    println!("  /plugin marketplace add {MARKETPLACE}");
    println!("  /plugin install {PLUGIN}");
    println!("Open the extension's plugin panel to complete installation.");
    println!("Then use: /cmr status; /cmr mode balanced; /cmr auto");
}
