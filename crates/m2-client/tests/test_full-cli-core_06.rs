#[path = "../src/syserr_threadsafe_logger.rs"]
mod syserr_threadsafe_logger;

use syserr_threadsafe_logger::{syserr_init, syserr_log, syserr_close};
use std::fs;
use std::thread;
use tempfile::tempdir;
use serial_test::serial;

#[test]
#[serial]
fn test_basic_logging() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("syserr.txt");
    let path_str = file_path.to_str().unwrap();

    // Inicjalizacja
    assert!(syserr_init(path_str).is_ok());

    // Zapis
    syserr_log("TEST_BASIC_LOGGING_MESSAGE");
    syserr_close();

    // Weryfikacja
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("TEST_BASIC_LOGGING_MESSAGE"), "Plik powinien zawierac wiadomosc.");
}

#[test]
#[serial]
fn test_multithreaded_logging() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("syserr_multi.txt");
    let path_str = file_path.to_str().unwrap();

    assert!(syserr_init(path_str).is_ok());

    let mut handles = vec![];
    for i in 0..10 {
        let handle = thread::Builder::new()
            .name(format!("Thread-{}", i))
            .spawn(move || {
                syserr_log(&format!("Wiadomosc z watku {}", i));
            })
            .unwrap();
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    syserr_close();

    let content = fs::read_to_string(&file_path).unwrap();
    for i in 0..10 {
        assert!(content.contains(&format!("Wiadomosc z watku {}", i)), "Brakuje logu z watku {}", i);
        assert!(content.contains(&format!("Thread-{}", i)), "Brakuje nazwy watku {}", i);
    }
}

#[test]
#[serial]
fn test_thread_name_format() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("syserr_format.txt");
    let path_str = file_path.to_str().unwrap();

    assert!(syserr_init(path_str).is_ok());

    let handle = thread::Builder::new()
        .name("CustomLoggerThread".to_string())
        .spawn(|| {
            syserr_log("TEST_FORMAT");
        })
        .unwrap();

    handle.join().unwrap();
    syserr_close();

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("[CustomLoggerThread]"), "Formatowanie powinno zawierac nazwe watku.");
    assert!(content.contains("TEST_FORMAT"), "Formatowanie powinno zawierac wiadomosc.");
    
    // Weryfikacja ze jest znacznik czasu w formacie yyyy-mm-dd
    assert!(content.contains("-"), "Powinny byc myslniki w dacie");
    assert!(content.contains(":"), "Powinny byc dwukropki w czasie");
}

#[test]
#[serial]
fn test_uninitialized_logger() {
    // Resetuj stan logera aby byl nie zainicjalizowany
    syserr_close();
    
    // Zapis do niezainicjalizowanego loggera nie powinien spowodowac paniki
    syserr_log("Ten log nie powinien zglaszac bledu.");
    // Ten test po prostu weryfikuje brak paniki.
}

#[test]
#[serial]
fn test_reinitialization() {
    let dir = tempdir().unwrap();
    let file_path1 = dir.path().join("syserr1.txt");
    let file_path2 = dir.path().join("syserr2.txt");

    // Inicjalizacja pliku 1
    assert!(syserr_init(file_path1.to_str().unwrap()).is_ok());
    syserr_log("MSG1");
    syserr_close();

    // Inicjalizacja pliku 2
    assert!(syserr_init(file_path2.to_str().unwrap()).is_ok());
    syserr_log("MSG2");
    syserr_close();

    let content1 = fs::read_to_string(&file_path1).unwrap();
    assert!(content1.contains("MSG1"));
    assert!(!content1.contains("MSG2"));

    let content2 = fs::read_to_string(&file_path2).unwrap();
    assert!(content2.contains("MSG2"));
    assert!(!content2.contains("MSG1"));
}
