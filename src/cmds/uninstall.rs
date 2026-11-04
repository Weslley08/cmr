//! Uninstall CMR plugin without touching RTK, other hooks or marketplaces.
use crate::core::environment;
use std::process::Command;

pub fn execute(dry_run: bool) -> u8 {
    if dry_run {
        println!("[dry-run] claude plugin uninstall model-router@cmr --scope user");
        println!("RTK and the CMR marketplace would remain installed.");
        return 0;
    }
    if !environment::available("claude") {
        println!("Use /plugin in the VS Code Claude Code panel to uninstall model-router@cmr.");
        println!("CMR remains installed until you remove it there.");
        return 2;
    }
    match Command::new("claude")
        .args(["plugin", "uninstall", "model-router@cmr", "--scope", "user"])
        .status()
    {
        Ok(status) if status.success() => {
            println!("CMR Mod removed. RTK and unrelated hooks were not changed.");
            0
        }
        Ok(_) => 1,
        Err(error) => {
            eprintln!("Cannot uninstall CMR Mod: {error}");
            1
        }
    }
}
