use std::fs;
use std::path::Path;

#[path = "../src/vfs_archive_integrity_checker.rs"]
mod vfs_archive_integrity_checker;
use vfs_archive_integrity_checker::{IntegrityError, VfsArchiveIntegrityChecker};

fn setup_temp_dir(name: &str) -> String {
    let path = format!("target/tmp_{}", name);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

fn create_file(dir: &str, filename: &str) {
    let path = Path::new(dir).join(filename);
    fs::write(path, b"dummy data").unwrap();
}

fn create_dir(dir: &str, dirname: &str) {
    let path = Path::new(dir).join(dirname);
    fs::create_dir_all(path).unwrap();
}

#[test]
fn test_all_archives_present() {
    let temp_dir = setup_temp_dir("all_present");
    create_file(&temp_dir, "root.pck");
    create_file(&temp_dir, "locale.pck");
    create_file(&temp_dir, "ETC.pck");
    create_file(&temp_dir, "uiscript.pck");

    let checker = VfsArchiveIntegrityChecker::new();
    let result = checker.check_archives(Path::new(&temp_dir));
    assert_eq!(result, Ok(()));
}

#[test]
fn test_some_archives_missing() {
    let temp_dir = setup_temp_dir("some_missing");
    create_file(&temp_dir, "root.pck");
    create_file(&temp_dir, "locale.pck");
    // Missing ETC.pck and uiscript.pck

    let checker = VfsArchiveIntegrityChecker::new();
    let result = checker.check_archives(Path::new(&temp_dir));
    assert_eq!(result, Err(IntegrityError::MissingArchive("ETC.pck".to_string())));
}

#[test]
fn test_base_directory_not_found() {
    let checker = VfsArchiveIntegrityChecker::new();
    let result = checker.check_archives(Path::new("target/non_existent_dir"));
    assert_eq!(result, Err(IntegrityError::BasePathNotFound));
}

#[test]
fn test_base_directory_is_file() {
    let temp_dir = setup_temp_dir("base_is_file_dir");
    let file_path = format!("{}/base_file", temp_dir);
    fs::write(&file_path, b"dummy data").unwrap();

    let checker = VfsArchiveIntegrityChecker::new();
    let result = checker.check_archives(Path::new(&file_path));
    assert_eq!(result, Err(IntegrityError::BasePathNotDirectory));
}

#[test]
fn test_archive_is_directory() {
    let temp_dir = setup_temp_dir("archive_is_directory");
    create_file(&temp_dir, "root.pck");
    create_file(&temp_dir, "locale.pck");
    create_file(&temp_dir, "ETC.pck");
    
    // uiscript.pck is a directory
    create_dir(&temp_dir, "uiscript.pck");

    let checker = VfsArchiveIntegrityChecker::new();
    let result = checker.check_archives(Path::new(&temp_dir));
    assert_eq!(result, Err(IntegrityError::ArchiveNotFile("uiscript.pck".to_string())));
}
