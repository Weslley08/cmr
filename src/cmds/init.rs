//! Install the CMR Mod through the official Claude Code plugin manager.
//! Never edits a hook as a substitute for model routing.
use crate::core::environment;
use std::process::Command;

const MARKETPLACE: &str = "Weslley08/cmr";
const PLUGIN: &str = "model-router@cmr";

fn run(args: &[&str]) -> Result<bool, String> {
    Command::new("claude")
        .args(args)
        .status()
        .map(|status| status.success())
        .map_err(|error| format!("Cannot execute Claude Code CLI: {error}"))
}

pub fn execute(dry_run: bool) -> u8 {
    if dry_run {
        println!("[dry-run] claude plugin marketplace add {MARKETPLACE}");
        println!("[dry-run] claude plugin install {PLUGIN} --scope user");
        println!("No RTK or Claude settings changed.");
        return 0;
    }
    if !environment::available("claude") {
        println!("Claude Code CLI not found. Install using the Claude Code VS Code panel:");
        println!("  /plugin marketplace add {MARKETPLACE}");
        println!("  /plugin install {PLUGIN}");
        println!("Choose user scope. This is a manual step; CMR has NOT been installed.");
        return 2;
    }
    println!("Adding CMR marketplace (existing registration is safe)...");
    let _ = run(&["plugin", "marketplace", "add", MARKETPLACE]);
    match run(&["plugin", "install", PLUGIN, "--scope", "user"]) {
        Ok(true) => {
            println!("CMR Mod installed for user scope; reload Claude Code plugins.");
            println!("RTK settings and existing hooks remain unchanged.");
            0
        }
        Ok(false) => {
            eprintln!("CMR plugin installation failed; inspect Claude Code /plugin.");
            1
        }
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}
