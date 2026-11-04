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
    println!("  init -g [--dry-run]        Install CMR Mod globally via Claude Code");
    println!("  uninstall -g [--dry-run]   Remove only CMR Mod; preserve RTK");
    println!("  recommend <prompt>        Classify without changing the current model");
    println!("  launch <prompt>           Start CLI session with chosen --model");
    println!("  doctor                    Diagnose CLI, optional RTK and settings");
    println!("  status                    List installed Claude Code plugins");
    println!("  stats                     Explain current metrics/limitations");
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
        // Legacy native advisory hook; no longer installed by default.
        Action::HookPrompt => return hook::execute(),
    }
    0
}
