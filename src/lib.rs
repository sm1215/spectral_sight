use std::path::{Path};


pub struct WowPath {
    path: Path,
}

impl WowPath {
    pub fn new() -> Self {
        let new_path = Path::new("./path_test");
        
        Self {
            path: new_path,
        }
    }
}

fn set_wow_path() {
    println!("hit");
    let wowPath = WowPath::new();
}