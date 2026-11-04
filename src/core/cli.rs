//! Pure command-line parser: no I/O side effects.
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Help,
    Version,
    Init,
    Doctor,
    Status,
    Stats,
    Uninstall,
}

pub fn parse(args: &[String]) -> Result<Action, String> {
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
        assert_eq!(parse(&["init".into(), "--global".into()]), Ok(Action::Init));
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
