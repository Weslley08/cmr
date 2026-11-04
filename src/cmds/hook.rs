//! Reads Claude's native hook JSON from stdin. Never modifies the selected model.
use crate::core::routing::recommend;
use serde_json::{json, Value};
use std::io::{self, Read};

pub fn execute() -> u8 {
    let mut input = String::new();
    if let Err(error) = io::stdin().read_to_string(&mut input) {
        eprintln!("CMR hook stdin: {error}");
        return 1;
    }
    let Ok(payload) = serde_json::from_str::<Value>(&input) else {
        eprintln!("CMR hook expects Claude Code JSON");
        return 1;
    };
    let Some(prompt) = payload.get("prompt").and_then(Value::as_str) else {
        // Fail open when the host gives an unexpected event payload.
        return 0;
    };
    let recommendation = recommend(prompt);
    let message = format!(
        "CMR suggests {} ({}) — advisory only. Current model unchanged; use /model {} to select it.",
        recommendation.model, recommendation.reason, recommendation.model
    );
    println!("{}", json!({ "systemMessage": message }));
    0
}
