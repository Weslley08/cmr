use crate::core::environment::{available, run, PLUGIN};

pub fn execute() -> u8 {
    if !available("claude") {
        println!("In the VS Code extension, uninstall {PLUGIN} using /plugin.");
        return 2;
    }
    if !run("claude", &["plugin", "uninstall", PLUGIN]) {
        return 1;
    }
    println!("CMR removed; RTK and the marketplace were not changed.");
    0
}
