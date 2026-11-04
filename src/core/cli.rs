#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Help,
    Version,
    Init { dry_run: bool },
    Doctor,
    Status,
    Stats,
    Uninstall { dry_run: bool },
    Recommend(String),
    Launch(String),
    HookPrompt,
}

pub fn parse(args: &[String]) -> Result<Action, String> {
    let Some(command) = args.first().map(String::as_str) else {
        return Ok(Action::Help);
    };
    let rest = &args[1..];
    match command {
        "help" | "-h" | "--help" if rest.is_empty() => Ok(Action::Help),
        "version" | "--version" | "-V" if rest.is_empty() => Ok(Action::Version),
        "doctor" if rest.is_empty() => Ok(Action::Doctor),
        "status" if rest.is_empty() => Ok(Action::Status),
        "stats" if rest.is_empty() => Ok(Action::Stats),
        "init" | "uninstall" => {
            if rest
                .iter()
                .any(|s| s != "-g" && s != "--global" && s != "--dry-run")
            {
                return Err(format!("Unexpected arguments: {}", rest.join(" ")));
            }
            let dry_run = rest.iter().any(|s| s == "--dry-run");
            if command == "init" {
                Ok(Action::Init { dry_run })
            } else {
                Ok(Action::Uninstall { dry_run })
            }
        }
        "recommend" | "launch" if !rest.is_empty() => {
            let prompt = rest.join(" ");
            if command == "recommend" {
                Ok(Action::Recommend(prompt))
            } else {
                Ok(Action::Launch(prompt))
            }
        }
        "hook" if rest == ["prompt"] => Ok(Action::HookPrompt),
        _ => Err(format!(
            "Unsupported command or arguments: {}",
            args.join(" ")
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_commands() {
        assert_eq!(parse(&[]), Ok(Action::Help));
        assert_eq!(
            parse(&["init".into(), "-g".into(), "--dry-run".into()]),
            Ok(Action::Init { dry_run: true })
        );
        assert_eq!(
            parse(&["hook".into(), "prompt".into()]),
            Ok(Action::HookPrompt)
        );
        assert_eq!(
            parse(&["recommend".into(), "hello".into()]),
            Ok(Action::Recommend("hello".into()))
        );
        assert!(parse(&["init".into(), "--unsafe".into()]).is_err());
    }
}
