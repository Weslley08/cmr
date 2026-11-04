use crate::core::{environment, settings};

pub fn execute() -> u8 {
    let result = (|| -> Result<(), String> {
        let path = environment::settings_path()?;
        let root = settings::read(&path)?;
        println!(
            "CMR native hook: {}",
            if settings::has_cmr(&root) {
                "installed"
            } else {
                "not installed"
            }
        );
        println!("Settings: {}", path.display());
        println!("RTK hook is independent; CMR runs on UserPromptSubmit, RTK on PreToolUse.");
        Ok(())
    })();
    if let Err(error) = result {
        eprintln!("{error}");
        return 1;
    }
    0
}
