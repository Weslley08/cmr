//! Standalone Rust executable. No Claude Code plugin, Mod or marketplace.
mod analytics;
mod cmds;
mod core;

use std::{env, process::ExitCode};

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
