use crate::core::environment::{available, print_vscode_instructions, run};

pub fn execute() -> u8 {
    if !available("claude") {
        print_vscode_instructions();
        return 2;
    }
    if !run("claude", &["plugin", "list"]) {
        return 1;
    }
    0
}
