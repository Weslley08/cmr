use crate::core::{environment::settings_path, settings};

pub fn execute(dry_run: bool) -> u8 {
    let result = (|| -> Result<(), String> {
        let path = settings_path()?;
        let mut root = settings::read(&path)?;
        if !settings::install(&mut root)? {
            println!(
                "CMR UserPromptSubmit hook already registered at {}",
                path.display()
            );
            return Ok(());
        }
        if dry_run {
            println!(
                "[dry-run] Would add 'cmr hook prompt' to UserPromptSubmit in {}",
                path.display()
            );
            return Ok(());
        }
        settings::save(&path, &root)?;
        println!(
            "CMR hook installed: {}. Restart Claude Code / VS Code.",
            path.display()
        );
        println!("CMR only recommends the model in VS Code; it cannot switch per prompt via native hooks.");
        Ok(())
    })();
    if let Err(e) = result {
        eprintln!("{e}");
        return 1;
    }
    0
}
