use crate::core::{environment, settings};
use std::process::Command;

pub fn execute() -> u8 {
    if environment::available("claude") {
        match Command::new("claude").args(["plugin", "list"]).status() {
            Ok(status) if status.success() => {}
            Ok(_) => return 1,
            Err(error) => {
                eprintln!("Cannot list Claude Code plugins: {error}");
                return 1;
            }
        }
    } else {
        println!("Claude CLI unavailable. Inspect Installed plugins in VS Code using /plugin.");
    }
    if let Ok(path) = environment::settings_path() {
        if let Ok(root) = settings::read(&path) {
            if settings::has_cmr(&root) {
                println!("Legacy CMR advisory hook found (does NOT switch the model).");
                println!("Remove legacy advisory if it duplicates Mod notifications.");
            }
        }
    }
    println!("RTK remains independent. To inspect routing, use /cmr status in Claude Code.");
    0
}
