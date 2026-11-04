use std::process::Command;

fn invoke(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_cmr"))
        .args(args)
        .output()
        .expect("CMR binary should execute")
}

#[test]
fn version_is_reported() {
    let output = invoke(&["version"]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        env!("CARGO_PKG_VERSION")
    );
}

#[test]
fn help_is_available_offline() {
    let output = invoke(&["help"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Claude Model Router"));
    assert!(stdout.contains("init"));
}

#[test]
fn invalid_flag_returns_nonzero() {
    let output = invoke(&["init", "--invalid"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Unexpected"));
}

#[test]
fn stats_do_not_require_claude_or_rtk() {
    let output = invoke(&["stats"]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("/cmr stats"));
}
