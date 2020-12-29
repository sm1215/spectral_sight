use std::path::{Path};


pub struct WowPath {
    path: Path,
}

impl WowPath {
    pub fn new() -> Self {
        let path = Path::new("./path_test");
        
        Self {
            path,
        }
    }
}

fn set_wow_path() {
    println!("hit");
    let wowPath = WowPath::new();
}