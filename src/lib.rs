use std::path::{PathBuf};
use std::{fs, io};

pub fn create_base_backup_folder(backup_path: &str) -> io::Result<()> {
    fs::create_dir(backup_path)?;
    Ok(())
}

pub fn copy_file(source_path: &PathBuf, destination_path: &PathBuf) -> io::Result<()> {
    println!("copying {:?} to {:?}", source_path, destination_path);
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

pub fn copy_directory_contents(source: &PathBuf, destination: &PathBuf) -> io::Result<()> {

    // destination
    // ./tests/source_test/interface_backups/include

    if !destination.exists() {
        fs::create_dir_all(&destination)?;
    }
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        // let entry_meta = fs::metadata(&path);

        println!("reading entry at {:#?}", path);
        
        // let mut nested_dirs = vec![];
        if path.is_dir() {
            // copy_directory_contents
            // nested_dirs.push(&path)
        } else {
            let mut destination = destination.clone();
            let filename = match path.file_name() {
                Some(filename) => filename,
                None => {
                    println!("no filename");
                    break;
                }
            };
            println!("filename {:#?}", filename);
            destination.push(&filename);



            copy_file(&path, &destination)?;
        }


    }
    
    Ok(())
}