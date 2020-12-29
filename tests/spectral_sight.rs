use spectral_sight::*;
use std::fs;
use std::path::Path;

pub struct TestConfig {
    base_path: String,
    backup_location: String,
}

impl TestConfig {
    pub fn new() -> Self {
        Self {
            base_path: String::from("./tests/path_test"),
            backup_location: String::from("interface_backups"),
        }
    }
}

#[test]
fn creates_backup_folder() {
    let cfg = TestConfig::new();
    let backup_path = [cfg.base_path, cfg.backup_location].join("//");
    let backup_result = create_base_backup_folder(&backup_path);

    assert_eq!(backup_result, ());
    assert_eq!(Path::new(&backup_path).exists(), true);
}

fn destroys_backup_folder() {
    
}

// #[test]
// fn test_pick_folders() {
//     let testing_path = "./tests/path_test";
//     let testing_folders = [
//         "include",
//         "important",
//     ];
//     let result = pick_folders(&testing_path, &testing_folders);
//     println!("result {:#?}", result);
//     // assert_eq!("sample", output);
// }
