use crate::core::{environment, settings};

pub fn execute() {
    println!("CMR {}", env!("CARGO_PKG_VERSION"));
    println!("Claude Code CLI in PATH: {}", environment::available("claude"));
    println!("RTK (optional) in PATH: {}", environment::available("rtk"));
    if let Ok(path) = environment::settings_path() {
        match settings::read(&path) {
            Ok(root) => println!("CMR hook installed: {}", settings::has_cmr(&root)),
            Err(error) => eprintln!("Settings warning: {error}"),
        }
    }
    println!("Native hook coverage: Claude Code CLI and local VS Code extension.");
    println!("Current limitation: native hooks cannot select the model per prompt.");
}
