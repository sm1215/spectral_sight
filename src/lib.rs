// use std::path::{Path};
use std::{fs, io};

pub fn create_base_backup_folder(backup_path: &str) -> io::Result<()> {
    fs::create_dir(backup_path)?;
    // set_perms(backup_path);
    Ok(())
}

pub fn copy_folder(source_path: &str, destination_path: &str) -> io::Result<()> {
    println!("copy source {:?}", source_path);
    println!("copy destination {:?}", destination_path);

    set_perms(source_path);
    set_perms(destination_path);
    fs::copy(source_path, destination_path)?;
    Ok(())
}

pub fn set_perms(path: &str) {
    let mut perms = fs::metadata(&path)
        .expect("error getting permissions in test setup")
        .permissions();
    perms.set_readonly(false);
    fs::set_permissions(&path, perms.clone())
        .expect("error setting permissions in test setup");
    println!("perms {:#?}", &perms);
}