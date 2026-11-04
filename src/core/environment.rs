use std::{env, path::PathBuf, process::Command};

pub fn available(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .output()
        .is_ok_and(|result| result.status.success())
}

pub fn home_dir() -> Option<PathBuf> {
    env::var_os("USERPROFILE").or_else(|| env::var_os("HOME")).map(PathBuf::from)
}

pub fn settings_path() -> Result<PathBuf, String> {
    home_dir().map(|home| home.join(".claude").join("settings.json"))
        .ok_or_else(|| "USERPROFILE or HOME is required to find Claude Code settings".into())
}

pub fn start_claude(model: &str, prompt: &str) -> Result<i32, String> {
    let status = Command::new("claude")
        .arg("--model").arg(model).arg(prompt)
        .status()
        .map_err(|e| format!("Cannot start Claude Code CLI: {e}. The VS Code extension does not install the 'claude' command automatically."))?;
    Ok(status.code().unwrap_or(1))
}
