//! Effective model choice only at CLI session startup.
use crate::core::{environment, routing::recommend};

pub fn execute(prompt: &str) -> u8 {
    let recommendation = recommend(prompt);
    println!("CMR launching Claude Code with --model {} ({})", recommendation.model, recommendation.reason);
    match environment::start_claude(recommendation.model, prompt) {
        Ok(code) if (0..=255).contains(&code) => code as u8,
        Ok(_) => 1,
        Err(error) => { eprintln!("{error}"); 1 }
    }
}
