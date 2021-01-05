// use std::path::{Path};
use std::{fs, io};

pub fn create_base_backup_folder(backup_path: &str) -> io::Result<()> {
    fs::create_dir(backup_path)?;
    Ok(())
}

pub fn copy_folder(source_path: &str, destination_path: &str) -> io::Result<()> {
    fs::copy(source_path, destination_path)?;
    Ok(())
}

pub fn set_write_perms(path: &str) {
    let mut perms = fs::metadata(&path)
        .expect("error getting permissions")
        .permissions();
    perms.set_readonly(false);
    fs::set_permissions(&path, perms.clone())
        .expect("error setting permissions");
}