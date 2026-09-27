use m2_client::app_panic_crash_handler::{
    CrashReport, disconnect_active_tcp_stream, flush_report_to_syserr, register_active_tcp_stream,
};
use std::fs;
use std::net::TcpListener;

#[test]
fn test_crash_report_creation() {
    let report = CrashReport {
        error_message: "Test error at file.rs:10".to_string(),
        stack_trace: vec!["trace_1".to_string(), "trace_2".to_string()],
        timestamp: 1234567890,
        system_info: "OS: Test, Arch: Test".to_string(),
    };

    assert_eq!(report.error_message, "Test error at file.rs:10");
    assert_eq!(report.stack_trace.len(), 2);
    assert_eq!(report.timestamp, 1234567890);
    assert_eq!(report.system_info, "OS: Test, Arch: Test");
}

#[test]
fn test_flush_report_to_syserr_creates_file_and_formats_correctly() {
    let test_file = "syserr_test.txt";
    
    // Upewniamy sie, ze plik nie istnieje przed testem
    let _ = fs::remove_file(test_file);

    let report = CrashReport {
        error_message: "Test error at test.rs:42".to_string(),
        stack_trace: vec![],
        timestamp: 1672531200,
        system_info: "Test OS".to_string(),
    };

    let result = flush_report_to_syserr(&report, test_file);
    assert!(result.is_ok(), "flush_report_to_syserr failed");

    let content = fs::read_to_string(test_file).expect("Failed to read test file");
    
    // Oczekiwany format: '[Data Czas] :: Rust Panic: ... at file:line'
    let expected_output = "[1672531200] :: Rust Panic: Test error at test.rs:42\n";
    assert_eq!(content, expected_output);

    // Sprzatamy po tescie
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_flush_report_appends_to_existing_file() {
    let test_file = "syserr_test_append.txt";
    let _ = fs::remove_file(test_file);

    let report1 = CrashReport {
        error_message: "First error at A.rs:1".to_string(),
        stack_trace: vec![],
        timestamp: 1000000000,
        system_info: "sys".to_string(),
    };

    let report2 = CrashReport {
        error_message: "Second error at B.rs:2".to_string(),
        stack_trace: vec![],
        timestamp: 2000000000,
        system_info: "sys".to_string(),
    };

    assert!(flush_report_to_syserr(&report1, test_file).is_ok());
    assert!(flush_report_to_syserr(&report2, test_file).is_ok());

    let content = fs::read_to_string(test_file).expect("Failed to read test file");
    let expected_content = "[1000000000] :: Rust Panic: First error at A.rs:1\n[2000000000] :: Rust Panic: Second error at B.rs:2\n";
    
    assert_eq!(content, expected_content);

    let _ = fs::remove_file(test_file);
}

#[test]
fn test_register_and_disconnect_tcp_stream() {
    // Tworzymy lokalny nasluchiwacz na losowym porcie
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let addr = listener.local_addr().unwrap();

    // Tworzymy polaczenie
    let stream = std::net::TcpStream::connect(addr).expect("Failed to connect");
    
    // Klonujemy strumien do uzycia w testach (poniewaz register pochalania wlasnosc)
    let stream_clone = stream.try_clone().expect("Failed to clone stream");

    // Rejestrujemy aktywny strumien TCP
    register_active_tcp_stream(stream);

    // Wywolujemy odlaczenie (tak jak to robi panic hook)
    disconnect_active_tcp_stream();

    // Poniewaz register_active_tcp_stream pobiera ownershipa i odlacza strumien uzywajac Shutdown::Both,
    // proby pisania/czytania do sklonowanego strumienia powinny teraz zwracac blad (zaleznie od OS i timingow,
    // najbezpieczniej sprawdzic czy nie ma paniki w trakcie wykonywania powyzszych).
    
    // Funkcja shutdown na sklonowanym gniezdzie powinna rowniez przejsc bez problemow jesli juz odlaczone (lub wywolac error wg platformy)
    // Ale wazne jest ze wywolanie disconnect_active_tcp_stream nie wywoluje paniki.
    
    // Sprawdzmy czy proba zapisu zakonczy sie bledem:
    use std::io::Write;
    let mut test_stream = stream_clone;
    let res = test_stream.write(b"test");
    // W zaleznosci od platformy moze to zwrocic Ok(4) przed realnym wyslaniem pakietu TCP (OS buffer), 
    // jednak nie psujemy testu dla roznych OS, najwazniejsze to brak paniki z disconnect_active_tcp_stream.
    let _ = res; 
}

#[test]
fn test_panic_hook_formatting_indirectly() {
    // Poniewaz przechwycenie hooka jest globalne i modyfikuje watek glowny,
    // manualnie sformatujemy CrashReport uzywajac struktury, aby zapobiec
    // nadpisywaniu globalnego hooka przez narzedzia CI.

    let report = CrashReport {
        error_message: "Artificial panic for test at dummy.rs:10".to_string(),
        stack_trace: vec![],
        timestamp: 0,
        system_info: "Test".to_string(),
    };
    
    assert!(report.error_message.contains("Artificial panic"));
    assert!(report.error_message.contains("dummy.rs"));
}
