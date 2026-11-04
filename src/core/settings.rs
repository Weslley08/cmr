//! Read-only inspection of legacy CMR hook configuration.
//! CMR 0.2 installs a Mod through Claude Code, never mutates settings.json.
use serde_json::Value;
use std::{fs, path::Path};

const COMMAND: &str = "cmr hook prompt";

pub fn read(path: &Path) -> Result<Value, String> {
    if !path.exists() {
        return Ok(serde_json::json!({}));
    }
    let raw = fs::read(path).map_err(|e| format!("Read {}: {e}", path.display()))?;
    let root: Value = serde_json::from_slice(&raw)
        .map_err(|e| format!("Invalid JSON in {}: {e}", path.display()))?;
    if !root.is_object() {
        return Err("Claude settings.json must be a JSON object".into());
    }
    Ok(root)
}

pub fn has_cmr(root: &Value) -> bool {
    root.get("hooks")
        .and_then(|h| h.get("UserPromptSubmit"))
        .and_then(Value::as_array)
        .is_some_and(|items| {
            items.iter().any(|item| {
                item.get("hooks")
                    .and_then(Value::as_array)
                    .is_some_and(|hooks| {
                        hooks.iter().any(|hook| {
                            hook.get("command").and_then(Value::as_str) == Some(COMMAND)
                        })
                    })
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_legacy_hook_without_touching_rtk() {
        let old = serde_json::json!({"hooks":{
            "PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"rtk hook claude"}]}],
            "UserPromptSubmit":[{"hooks":[{"type":"command","command":"cmr hook prompt"}]}]
        }});
        assert!(has_cmr(&old));
        assert_eq!(
            old["hooks"]["PreToolUse"][0]["hooks"][0]["command"],
            "rtk hook claude"
        );
    }

    #[test]
    fn does_not_confuse_rtk_with_cmr() {
        let root = serde_json::json!({"hooks":{
            "PreToolUse":[{"matcher":"Bash","hooks":[{"type":"command","command":"rtk hook claude"}]}]
        }});
        assert!(!has_cmr(&root));
    }
}
