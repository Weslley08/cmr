use std::io::Write;
use std::process::{Command, Stdio};

fn command(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_cmr"))
        .args(args)
        .output()
        .expect("CMR binary should run")
}

#[test]
fn help_version_and_recommendation() {
    let version = command(&["version"]);
    assert!(version.status.success());
    assert_eq!(
        String::from_utf8_lossy(&version.stdout).trim(),
        env!("CARGO_PKG_VERSION")
    );
    assert!(String::from_utf8_lossy(&command(&["help"]).stdout).contains("init"));
    let output = command(&["recommend", "Liste arquivos do src"]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("haiku"));
}

#[test]
fn hook_emits_advice_without_model_switch_field() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_cmr"))
        .args(["hook", "prompt"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn cmr hook");
    child.stdin.take().unwrap()
        .write_all(br#"{"hook_event_name":"UserPromptSubmit","prompt":"Refatore a arquitetura distribuida"}"#)
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let parsed: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(parsed["systemMessage"].as_str().unwrap().contains("opus"));
    assert!(parsed.get("model").is_none());
    assert!(parsed["systemMessage"]
        .as_str()
        .unwrap()
        .contains("advisory only"));
}

#[test]
fn rejects_unknown_flags() {
    let output = command(&["init", "--unsafe"]);
    assert_eq!(output.status.code(), Some(2));
}
