//! Entry point for the cross-platform CMR CLI.
//! Model routing itself runs inside the Claude Code Mod, not this process.
mod analytics;
mod cmds;
mod core;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match core::cli::parse(&args) {
        Ok(action) => ExitCode::from(cmds::execute(action)),
        Err(reason) => {
            eprintln!("{reason}");
            cmds::print_help();
            ExitCode::from(2)
        }
    }
}
