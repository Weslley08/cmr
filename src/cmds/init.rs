use crate::core::environment::{available, print_vscode_instructions, run, MARKETPLACE, PLUGIN};

pub fn execute() -> u8 {
    if !available("claude") {
        print_vscode_instructions();
        return 2;
    }
    println!("Registering CMR marketplace...");
    // The marketplace might be registered already. Always attempt install.
    let _ = run("claude", &["plugin", "marketplace", "add", MARKETPLACE]);
    if !run("claude", &["plugin", "install", PLUGIN]) {
        eprintln!("Plugin install failed. Inspect Claude Code plugin manager.");
        return 1;
    }
    println!("CMR installed. Start a new session and run /cmr auto.");
    0
}
