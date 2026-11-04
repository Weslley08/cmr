use crate::core::{environment::settings_path, settings};

pub fn execute(dry_run: bool) -> u8 {
    let result = (|| -> Result<(), String> {
        let path = settings_path()?;
        let mut root = settings::read(&path)?;
        if !settings::has_cmr(&root) {
            println!("No CMR hook configured.");
            return Ok(());
        }
        settings::uninstall(&mut root)?;
        if dry_run {
            println!("[dry-run] Would remove only CMR's UserPromptSubmit command.");
            return Ok(());
        }
        settings::save(&path, &root)?;
        println!("CMR hook removed. RTK and unrelated hooks preserved.");
        Ok(())
    })();
    if let Err(e) = result {
        eprintln!("{e}");
        return 1;
    }
    0
}
