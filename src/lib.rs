use std::path::{Path};
use std::{fs, io};

pub fn create_base_backup_folder(backup_path: &str) -> io::Result<()> {
    fs::create_dir(backup_path)?;
    Ok(())
}
