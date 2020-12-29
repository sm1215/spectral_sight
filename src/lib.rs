use std::path::{Path};
use std::{fs, io};

pub fn read_contents(target_path: String) -> io::Result<()> {
    println!("read_contents running...");
    let default_path = String::from("./");
    let wow_path = Path::new(&target_path);

    let mut entries = fs::read_dir(wow_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    println!("entries {:#?}", entries);

    Ok(())
}