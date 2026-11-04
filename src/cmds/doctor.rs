use crate::core::{environment, settings};

pub fn execute() {
    println!("CMR CLI {}", env!("CARGO_PKG_VERSION"));
    println!(
        "Claude Code CLI in PATH: {}",
        environment::available("claude")
    );
    println!("RTK (optional) in PATH: {}", environment::available("rtk"));
    if let Ok(path) = environment::settings_path() {
        match settings::read(&path) {
            Ok(root) => {
                println!(
                    "Legacy advisory hook installed: {}",
                    settings::has_cmr(&root)
                );
            }
            Err(error) => eprintln!("Settings warning: {error}"),
        }
    }
    println!("CMR automatic routing runs inside the Claude Code Mod via turn.step.");
    println!("If only VS Code is installed, use /plugin to inspect Mod installation.");
    println!("This diagnostic does not confirm that real API requests were routed.");
}
