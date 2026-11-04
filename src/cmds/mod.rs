//! User-facing CLI commands, organized like RTK's src/cmds.
mod doctor;
mod init;
mod status;
mod uninstall;

use crate::core::cli::Action;

pub fn print_help() {
    println!("CMR — Claude Model Router {}", env!("CARGO_PKG_VERSION"));
    println!("Usage: cmr <command>");
    println!("  init [-g]       Register marketplace and install Mod (user scope)");
    println!("  doctor          Inspect local Claude Code and RTK availability");
    println!("  status          List plugins through Claude Code CLI");
    println!("  stats           Explain how to access in-session CMR metrics");
    println!("  uninstall [-g]  Uninstall CMR Mod (preserves RTK and marketplace)");
    println!("  version         Show the CLI version");
    println!("  help            Show this message");
}

pub fn execute(action: Action) -> u8 {
    match action {
        Action::Help => print_help(),
        Action::Version => println!("{}", env!("CARGO_PKG_VERSION")),
        Action::Init => return init::execute(),
        Action::Doctor => doctor::execute(),
        Action::Status => return status::execute(),
        Action::Stats => crate::analytics::print_stats_help(),
        Action::Uninstall => return uninstall::execute(),
    }
    0
}
