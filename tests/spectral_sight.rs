use spectral_sight::*;
use std::path::Path;
use std::{fs, io, panic};

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

// deletes entire backup directory, intended only for tearing down tests
fn destroy_base_backup_folder(backup_path: &str) -> io::Result<()> {
    fs::remove_dir_all(backup_path)?;
    Ok(())
}

fn setup() {
    let cfg = TestConfig::new();
    let backup_path = [cfg.base_path, cfg.backup_location].join("//");
    let _backup_result = create_base_backup_folder(&backup_path);
}

fn teardown() {
    let cfg = TestConfig::new();
    let backup_path = [cfg.base_path, cfg.backup_location].join("//");
    let _backup_result = destroy_base_backup_folder(&backup_path);
}

fn run_test<T> (test: T) 
where T: FnOnce() -> () + panic::UnwindSafe
{
    setup();
    // wrapping the test in an enclosure will allow the program to error
    // and still run the test teardown logic after
    let result = panic::catch_unwind(|| {
        test()
    });
    teardown();
    assert!(result.is_ok())
}

#[test]
fn creates_backup_folder() {
    let cfg = TestConfig::new();
    let backup_path = [cfg.base_path, cfg.backup_location].join("//");
    let _backup_result = create_base_backup_folder(&backup_path);
    assert_eq!(Path::new(&backup_path).exists(), true);
}

#[test]
fn destroys_backup_folder() {
    let cfg = TestConfig::new();
    let backup_path = [cfg.base_path, cfg.backup_location].join("//");
    let _backup_result = destroy_base_backup_folder(&backup_path);
    assert_eq!(Path::new(&backup_path).exists(), false);
}

