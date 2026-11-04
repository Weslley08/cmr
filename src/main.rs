//! CMR CLI. The authoritative model routing happens in the Claude Code mod.
//! The CLI only manages the official marketplace and gives diagnostics.
use std::env;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

const MARKETPLACE: &str = "Weslley08/cmr";
const PLUGIN: &str = "model-router@weslley08-cmr";

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Help,
    Version,
    Init,
    Doctor,
    Status,
    Stats,
    Uninstall,
}

fn parse(args: &[String]) -> Result<Action, String> {
    let Some(command) = args.first().map(String::as_str) else {
        return Ok(Action::Help);
    };
    let extras = &args[1..];
    let accepts_global = matches!(command, "init" | "uninstall")
        && (extras.is_empty() || extras == ["-g"] || extras == ["--global"]);
    if !extras.is_empty() && !accepts_global {
        return Err(format!("Unexpected arguments: {}", extras.join(" ")));
    }
    match command {
        "help" | "-h" | "--help" => Ok(Action::Help),
        "version" | "-V" | "--version" => Ok(Action::Version),
        "init" => Ok(Action::Init),
        "doctor" => Ok(Action::Doctor),
        "status" => Ok(Action::Status),
        "stats" => Ok(Action::Stats),
        "uninstall" => Ok(Action::Uninstall),
        other => Err(format!("Unknown command: {other}")),
    }
}

fn available(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

fn run(program: &str, arguments: &[&str]) -> bool {
    match Command::new(program).args(arguments).status() {
        Ok(status) => status.success(),
        Err(error) => {
            eprintln!("Cannot start {program}: {error}");
            false
        }
    }
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
}

fn print_vscode_instructions() {
    println!("Claude Code CLI not found. For the VS Code extension:");
    println!("  /plugin marketplace add {MARKETPLACE}");
    println!("  /plugin install {PLUGIN}");
    println!("Open the extension's plugin panel to complete installation.");
    println!("Then use: /cmr status; /cmr mode balanced; /cmr auto");
}

fn print_help() {
    println!("CMR — Claude Model Router {}", env!("CARGO_PKG_VERSION"));
    println!("Usage: cmr <command>");
    println!("  init [-g]       Register GitHub marketplace and install the mod (user scope)");
    println!("  doctor          Inspect local Claude Code and optional RTK availability");
    println!("  status          Show plugins through Claude Code CLI");
    println!("  stats           Explain how to access session-scoped CMR metrics");
    println!("  uninstall [-g]  Uninstall CMR mod (leaves RTK and marketplace untouched)");
    println!("  version         Show the CLI version");
    println!("  help            Show this message");
}

fn execute(action: Action) -> u8 {
    match action {
        Action::Help => print_help(),
        Action::Version => println!("{}", env!("CARGO_PKG_VERSION")),
        Action::Init => {
            if !available("claude") {
                print_vscode_instructions();
                return 2;
            }
            println!("Registering official CMR marketplace...");
            // Marketplace might already be registered: installation is still attempted.
            let _ = run("claude", &["plugin", "marketplace", "add", MARKETPLACE]);
            if !run("claude", &["plugin", "install", PLUGIN]) {
                eprintln!("Plugin install failed. Inspect the Claude Code plugin manager.");
                return 1;
            }
            println!("CMR installed. Start a new Claude session and run /cmr auto.");
        }
        Action::Doctor => {
            println!("CMR CLI: {}", env!("CARGO_PKG_VERSION"));
            println!("Claude Code CLI: {}", if available("claude") { "found" } else { "not found (VS Code extension can work independently)" });
            println!("RTK (optional): {}", if available("rtk") { "found" } else { "not found" });
            if let Some(home) = home_dir() {
                let settings = home.join(".claude").join("settings.json");
                println!("Claude settings: {} ({})", settings.display(), if settings.exists() { "exists" } else { "not found" });
            }
            println!("Note: this checks availability, not actual routing or host compatibility.");
        }
        Action::Status => {
            if !available("claude") {
                print_vscode_instructions();
                return 2;
            }
            if !run("claude", &["plugin", "list"]) {
                return 1;
            }
        }
        Action::Stats => {
            println!("CMR statistics are session-scoped: type /cmr stats inside Claude Code.");
            println!("For RTK command savings, run: rtk gain");
            println!("Do not equate RTK or CMR counters to subscription quota savings.");
        }
        Action::Uninstall => {
            if !available("claude") {
                println!("In the VS Code extension, uninstall {PLUGIN} using /plugin.");
                return 2;
            }
            if !run("claude", &["plugin", "uninstall", PLUGIN]) {
                return 1;
            }
            println!("CMR removed; RTK and the marketplace were not changed.");
        }
    }
    0
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match parse(&args) {
        Ok(action) => ExitCode::from(execute(action)),
        Err(reason) => {
            eprintln!("{reason}");
            print_help();
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_empty_args() {
        assert_eq!(parse(&[]), Ok(Action::Help));
    }

    #[test]
    fn accepts_global_flag_for_init() {
        assert_eq!(parse(&["init".into(), "-g".into()]), Ok(Action::Init));
        assert_eq!(
            parse(&["init".into(), "--global".into()]),
            Ok(Action::Init)
        );
    }

    #[test]
    fn rejects_unknown_flags() {
        assert!(parse(&["init".into(), "--force".into()]).is_err());
        assert!(parse(&["status".into(), "--global".into()]).is_err());
    }

    #[test]
    fn works_without_cli_for_help_and_diagnostics() {
        assert_eq!(parse(&["doctor".into()]), Ok(Action::Doctor));
        assert_eq!(parse(&["stats".into()]), Ok(Action::Stats));
    }
}
