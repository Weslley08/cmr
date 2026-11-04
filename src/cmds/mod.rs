mod doctor;
mod hook;
mod init;
mod launch;
mod status;
mod uninstall;

use crate::core::cli::Action;

pub fn print_help() {
    println!("CMR — Claude Model Router {}", env!("CARGO_PKG_VERSION"));
    println!("Usage: cmr <command>");
    println!("  init -g [--dry-run]        Install native global UserPromptSubmit hook");
    println!("  uninstall -g [--dry-run]   Remove only CMR hook; preserve RTK");
    println!("  recommend <prompt>        Classify without changing the model");
    println!("  launch <prompt>           Start Claude Code CLI with the selected model");
    println!("  doctor                    Diagnose PATH, hooks and RTK");
    println!("  status                    Check CMR hook installation");
    println!("  stats                     Describe metrics and current limitations");
    println!("  version                   Print version");
    println!("  help                      Show help");
}

pub fn execute(action: Action) -> u8 {
    match action {
        Action::Help => print_help(),
        Action::Version => println!("{}", env!("CARGO_PKG_VERSION")),
        Action::Init { dry_run } => return init::execute(dry_run),
        Action::Doctor => doctor::execute(),
        Action::Status => return status::execute(),
        Action::Stats => crate::analytics::print_stats_help(),
        Action::Uninstall { dry_run } => return uninstall::execute(dry_run),
        Action::Recommend(prompt) => {
            let selection = crate::core::routing::recommend(&prompt);
            println!(
                "{} — {} (recommendation only; current session unchanged)",
                selection.model, selection.reason
            );
        }
        Action::Launch(prompt) => return launch::execute(&prompt),
        Action::HookPrompt => return hook::execute(),
    }
    0
}
