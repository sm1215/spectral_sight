use std::path::{PathBuf};
use std::{fs, io};

pub fn create_nested_directory(path: &PathBuf) -> io::Result<()> {
    println!("creating directory {:#?}", path);
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn copy_file(source_path: &PathBuf, destination_path: &PathBuf) -> io::Result<()> {
    println!("copying {:?} to {:?}", source_path, destination_path);
    fs::copy(source_path, destination_path)?;
    Ok(())
}

pub fn set_write_perms(path: &PathBuf) {
    let mut perms = fs::metadata(&path)
        .expect("error getting permissions")
        .permissions();
    perms.set_readonly(false);
    fs::set_permissions(&path, perms.clone())
        .expect("error setting permissions");
}

pub fn copy_directory_contents(source: &PathBuf, destination: &PathBuf) -> io::Result<()> {
    println!("\nentering source {:#?}", source);
    println!("destination exists? {:#?}, path {:#?}", destination.exists(), destination);
    if !destination.exists() {
        fs::create_dir_all(&destination)?;
    }
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // need to pull tail from path and append to destination
            let nested_path = match path.iter().last() {
                Some(nested_path) => nested_path,
                None => {
                    println!("no nested_path");
                    break;
                }
            };
            let mut destination = destination.clone();
            destination.push(&nested_path);
            copy_directory_contents(&path, &destination)?;
        } else {
            let mut destination = destination.clone();
            let filename = match path.file_name() {
                Some(filename) => filename,
                None => {
                    println!("no filename");
                    break;
                }
            };
            destination.push(&filename);
            copy_file(&path, &destination)?;
        }
    }
    
    Ok(())
}