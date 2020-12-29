use std::path::{Path};
use std::{fs, io};

pub fn create_base_backup_folder(backup_path: &str) -> io::Result<()> {
    fs::create_dir(backup_path)?;
    Ok(())
}

pub fn pick_folders(target_path: &str, target_folders: &[&str]) -> io::Result<()> {
    println!("pick_folders running...");
    let wow_path = Path::new(target_path);

    let mut entries = fs::read_dir(wow_path)?
        .map(|res| {
            // println!("res {:#?}", res);
            res.map(|e| e.path())
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    println!("entries {:#?}", entries);

    Ok(())
}