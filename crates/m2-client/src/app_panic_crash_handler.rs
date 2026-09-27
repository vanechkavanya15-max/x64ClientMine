use std::fs::OpenOptions;
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use std::panic::PanicHookInfo;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Struktura reprezentujaca zrzut informacji o bledzie.
#[derive(Debug, Clone)]
pub struct CrashReport {
    pub error_message: String,
    pub stack_trace: Vec<String>,
    pub timestamp: u64,
    pub system_info: String,
}

/// Przechowuje aktywny uchwyt do strumienia TCP (jesli istnieje) aby
/// umozliwic jego bezpieczne i natychmiastowe zamkniecie podczas paniki.
static ACTIVE_TCP_STREAM: OnceLock<Mutex<Option<TcpStream>>> = OnceLock::new();

/// Rejestruje aktywny strumien TCP do ewentualnego awaryjnego odlaczenia.
pub fn register_active_tcp_stream(stream: TcpStream) {
    let lock = ACTIVE_TCP_STREAM.get_or_init(|| Mutex::new(None));
    if let Ok(mut lock) = lock.lock() {
        *lock = Some(stream);
    }
}

/// Wymusza zamkniecie zarejestrowanego wczesniej strumienia TCP.
pub fn disconnect_active_tcp_stream() {
    if let Some(lock) = ACTIVE_TCP_STREAM.get() {
        if let Ok(mut opt_stream) = lock.try_lock() {
            if let Some(stream) = opt_stream.take() {
                // Bezpieczne wymuszenie rozlaczenia aktywnej sesji TCP
                let _ = stream.shutdown(Shutdown::Both);
            }
        }
    }
}

/// Formatuje informacje o bledzie (PanicHookInfo) do struktury CrashReport.
pub fn format_panic_report(info: &PanicHookInfo) -> CrashReport {
    let payload = info.payload();
    let error_msg = if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic payload".to_string()
    };

    let location_str = if let Some(location) = info.location() {
        format!("{}:{}", location.file(), location.line())
    } else {
        "unknown_file:0".to_string()
    };

    let full_error_message = format!("{} at {}", error_msg, location_str);

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let stack_trace = std::backtrace::Backtrace::capture().to_string().lines().map(String::from).collect();

    let system_info = format!("OS: {}, Arch: {}", std::env::consts::OS, std::env::consts::ARCH);

    CrashReport {
        error_message: full_error_message,
        stack_trace,
        timestamp,
        system_info,
    }
}

/// Zapisuje raport awarii do pliku tekstowego w formacie syserr.
pub fn flush_report_to_syserr(report: &CrashReport, file_path: &str) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    // Zgodnie ze specyfikacja zrzut do pliku musi miec format: 
    // '[Data Czas] :: Rust Panic: ... at file:line'
    let output = format!("[{}] :: Rust Panic: {}\n", report.timestamp, report.error_message);
    
    file.write_all(output.as_bytes())?;
    Ok(())
}

/// Rejestruje glowny hook dla paniki, ktory formatuje log, rozlacza siec i konczy dzialanie.
pub fn install_panic_hook(log_path: &str) {
    let log_path_owned = log_path.to_string();
    std::panic::set_hook(Box::new(move |info| {
        // Formatuje raport z paniki
        let report = format_panic_report(info);
        
        // Zapisuje raport do syserr.txt
        let _ = flush_report_to_syserr(&report, &log_path_owned);
        
        // Bezpieczne rozlaczenie aktywnej sesji TCP przed zamknieciem procesu
        disconnect_active_tcp_stream();
    }));
}
