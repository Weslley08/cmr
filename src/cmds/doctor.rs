use crate::core::environment::{available, home_dir};

pub fn execute() {
    println!("CMR CLI: {}", env!("CARGO_PKG_VERSION"));
    println!(
        "Claude Code CLI: {}",
        if available("claude") {
            "found"
        } else {
            "not found (VS Code extension works independently)"
        }
    );
    println!(
        "RTK (optional): {}",
        if available("rtk") {
            "found"
        } else {
            "not found"
        }
    );
    if let Some(home) = home_dir() {
        let settings = home.join(".claude").join("settings.json");
        println!(
            "Claude settings: {} ({})",
            settings.display(),
            if settings.exists() {
                "exists"
            } else {
                "not found"
            }
        );
    }
    println!("Availability check only, not proof of live model routing.");
}
