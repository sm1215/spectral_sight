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
            base_path: String::from("./tests/source_test"),
            backup_location: String::from("interface_backups"),
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

fn set_perms(path: &str) {
    let path = String::from("./tests");
    let mut perms = fs::metadata(&path)
        .expect("error getting permissions in test setup")
        .permissions();
    perms.set_readonly(false);
    fs::set_permissions(&path, perms)
        .expect("error setting permissions in test setup");
}

fn setup() {
    println!("\n running test setup...");
    let backup_path = &TestConfig::get_backup_path();
    let _create_backup_result = create_base_backup_folder(backup_path);
    println!("backup directory created at {:?}", backup_path);

    // directory is initially created with read-only permissions
    let mut perms = fs::metadata(backup_path)
        .expect("error getting permissions in test setup")
        .permissions();
    perms.set_readonly(false);
    fs::set_permissions(backup_path, perms.clone())
        .expect("error setting permissions in test setup");
    
        println!("backup directory perms {:#?}", perms);

    let backup_path_exists = Path::new(&backup_path).exists();
    assert_eq!(backup_path_exists, true);
}

fn teardown() {
    println!("running test teardown...");
    let backup_path = &TestConfig::get_backup_path();
    let _destry_backup_result = destroy_base_backup_folder(&TestConfig::get_backup_path());
    
    let backup_path_exists = Path::new(&backup_path).exists();
    println!("backup directory removed {:?}", backup_path_exists);
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
    println!("test result {:#?}", result);
    // teardown();
    assert!(result.is_ok())
}

// #[test]
// fn creates_backup_folder() {
//     let backup_path = &TestConfig::get_backup_path();
//     println!("backup_path {:?}", backup_path);
//     let _backup_result = create_base_backup_folder(backup_path);
//     assert_eq!(Path::new(&backup_path).exists(), true);
// }

// #[test]
// fn destroys_backup_folder() {
//     let backup_path = &TestConfig::get_backup_path();
//     let _backup_result = destroy_base_backup_folder(&backup_path);
//     assert_eq!(Path::new(backup_path).exists(), false);
// }

#[test]
fn copies_a_folder() {
    let cfg = TestConfig::new();
    let base_path = cfg.base_path;
    let backup_location = cfg.backup_location;
    let target_dir = String::from("include");
    let target_file = String::from("payload.txt");

    let source = [base_path.clone(), target_dir.clone(), target_file.clone()].join("/");
    let destination = [base_path.clone(), backup_location.clone(), target_file.clone()].join("/");

    run_test(|| {
        let _copy_result = copy_folder(&source, &destination);
        println!("copy result {:#?}", _copy_result);

        let backed_up_file = [destination.clone(), target_dir.clone()].join("/");
        println!("backup path {:?}", backed_up_file);

        assert_eq!(Path::new(&backed_up_file).exists(), true);
    })
}
