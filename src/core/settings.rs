//! Safe, idempotent hook registration: never clobber other hooks (including RTK).
use serde_json::{json, Value};
use std::{fs, path::Path};

const COMMAND: &str = "cmr hook prompt";

pub fn read(path: &Path) -> Result<Value, String> {
    if !path.exists() {
        return Ok(json!({}));
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
        .is_some_and(|items| items.iter().any(|item| {
            item.get("hooks").and_then(Value::as_array)
                .is_some_and(|hooks| hooks.iter().any(|hook| hook.get("command").and_then(Value::as_str) == Some(COMMAND)))
        }))
}

pub fn install(root: &mut Value) -> Result<bool, String> {
    if has_cmr(root) { return Ok(false); }
    let object = root.as_object_mut().ok_or("Settings root is not an object")?;
    let hooks = object.entry("hooks").or_insert_with(|| json!({}))
        .as_object_mut().ok_or("Existing 'hooks' is not an object")?;
    let array = hooks.entry("UserPromptSubmit").or_insert_with(|| json!([]))
        .as_array_mut().ok_or("Existing 'UserPromptSubmit' is not an array")?;
    array.push(json!({ "hooks": [{ "type": "command", "command": COMMAND, "timeout": 10 }] }));
    Ok(true)
}

pub fn uninstall(root: &mut Value) -> Result<bool, String> {
    let Some(hooks) = root.get_mut("hooks").and_then(Value::as_object_mut) else { return Ok(false); };
    let Some(items) = hooks.get_mut("UserPromptSubmit").and_then(Value::as_array_mut) else { return Ok(false); };
    let before = items.len();
    // Remove the CMR command only, keeping other hooks even in the same entry.
    for item in items.iter_mut() {
        if let Some(commands) = item.get_mut("hooks").and_then(Value::as_array_mut) {
            commands.retain(|v| v.get("command").and_then(Value::as_str) != Some(COMMAND));
        }
    }
    items.retain(|item| item.get("hooks").and_then(Value::as_array).is_none_or(|a| !a.is_empty()));
    let removed = before != items.len() || !has_cmr(root);
    Ok(removed && before > 0)
}

pub fn save(path: &Path, root: &Value) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(root).map_err(|e| e.to_string())?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Cannot create settings directory: {e}"))?;
    }
    if path.exists() {
        // Preserve the first original backup; do not overwrite it on upgrades.
        let backup = path.with_file_name("settings.json.cmr.bak");
        if !backup.exists() {
            fs::copy(path, &backup).map_err(|e| format!("Cannot back up settings: {e}"))?;
        }
    }
    fs::write(path, [bytes, vec![b'\n']].concat()).map_err(|e| format!("Cannot save {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_other_hooks() {
        let mut root = json!({ "hooks": { "PreToolUse": [{ "matcher": "Bash", "hooks": [{"type": "command", "command": "rtk hook claude"}]}]}});
        assert!(install(&mut root).unwrap());
        assert!(!install(&mut root).unwrap());
        assert!(has_cmr(&root));
        assert_eq!(root["hooks"]["PreToolUse"][0]["hooks"][0]["command"], "rtk hook claude");
        assert!(uninstall(&mut root).unwrap());
        assert!(!has_cmr(&root));
        assert_eq!(root["hooks"]["PreToolUse"][0]["hooks"][0]["command"], "rtk hook claude");
    }
}
