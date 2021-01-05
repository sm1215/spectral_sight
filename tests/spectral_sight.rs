use spectral_sight::*;
use std::path::{Path, PathBuf};
use std::{fs, io, panic};

pub struct TestConfig {
    base_path: String,
    backup_location: String,
    disable_cleanup: bool,
}

impl TestConfig {
    pub fn new() -> Self {
        Self {
            base_path: String::from("./tests/source_test"),
            backup_location: String::from("interface_backups"),
            disable_cleanup: true
        }
    }
    pub fn get_backup_path() -> String {
        let cfg = Self::new();
        [cfg.base_path, cfg.backup_location].join("/")
    }
}

// deletes entire backup directory, intended only for tearing down tests
fn destroy_base_backup_folder(backup_path: &str) -> io::Result<()> {
    fs::remove_dir_all(backup_path)?;
    Ok(())
}

fn setup() {
    let backup_path = &TestConfig::get_backup_path();
    let _create_backup_result = create_base_backup_folder(backup_path);

    // make sure backup_directory can be written to
    set_write_perms(backup_path);

    let backup_path_exists = Path::new(&backup_path).exists();
    assert_eq!(backup_path_exists, true);
}

fn teardown() {
    let cfg = TestConfig::new();
    if cfg.disable_cleanup {
        println!("test cleanup disabled, directories will persist...");
        return
    }
    let backup_path = &TestConfig::get_backup_path();
    let _destroy_backup_result = destroy_base_backup_folder(&TestConfig::get_backup_path());
    
    let backup_path_exists = Path::new(&backup_path).exists();
    assert_eq!(backup_path_exists, false);
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
fn _creates_backup_folder() {
    let backup_path = &TestConfig::get_backup_path();
    let _backup_result = create_base_backup_folder(backup_path);
    assert_eq!(Path::new(&backup_path).exists(), true);
}

#[test]
fn _destroys_backup_folder() {
    let backup_path = &TestConfig::get_backup_path();
    let _backup_result = destroy_base_backup_folder(&backup_path);
    assert_eq!(Path::new(backup_path).exists(), false);
}

#[test]
fn copies_a_file() {
    let cfg = TestConfig::new();
    let base_path = cfg.base_path;
    let backup_location = cfg.backup_location;
    let target_dir = String::from("include");
    let target_file = String::from("payload.txt");

    let source: PathBuf = [base_path.clone(), target_dir.clone(), target_file.clone()].iter().collect();
    let destination: PathBuf = [base_path.clone(), backup_location.clone(), target_file.clone()].iter().collect();

    run_test(|| {
        let _copy_result = copy_file(&source, &destination);
        let backed_up_file = destination.clone();
        assert_eq!(Path::new(&backed_up_file).exists(), true);
    });
}

#[test]
fn copies_nested_files() {
    let cfg = TestConfig::new();
    let base_path = cfg.base_path;
    let backup_location = cfg.backup_location;
    let target_dir = String::from("include");

    let source: PathBuf = [base_path.clone(), target_dir.clone()].iter().collect();
    let destination: PathBuf = [base_path.clone(), backup_location.clone(), target_dir.clone()].iter().collect();
    
    let mut backed_up_file_1 = destination.clone();
    backed_up_file_1.push("payload.txt");

    println!("backed_up_file_1 {:#?}", backed_up_file_1);
    
    let mut backed_up_file_2 = destination.clone();
    backed_up_file_2.push("payload_2.txt");

    // let backed_up_file_3 = [destination.clone(), String::from("nested_folder"), String::from("nested_file.txt")].join("/");

    run_test(|| {
        let _copy_result = copy_directory_contents(&source, &destination);
        assert_eq!(backed_up_file_1.exists(), true);
        assert_eq!(backed_up_file_2.exists(), true);
        // assert_eq!(backed_up_file_3.exists(), true);
    });
}