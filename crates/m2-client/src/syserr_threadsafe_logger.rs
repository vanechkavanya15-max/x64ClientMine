use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;
use std::thread;

lazy_static! {
    static ref SYSERR_FILE: Mutex<Option<File>> = Mutex::new(None);
}

/// Inicjalizuje logger. Otwiera plik syserr do dopisywania.
pub fn syserr_init(path: &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    
    let mut lock = SYSERR_FILE.lock().unwrap();
    *lock = Some(file);
    Ok(())
}

/// Zamyka plik syserr. Przydatne do testow.
pub fn syserr_close() {
    let mut lock = SYSERR_FILE.lock().unwrap();
    *lock = None;
}

/// Zapisuje wiadomosc do pliku syserr.txt ze znacznikiem czasu i nazwa watku.
pub fn syserr_log(msg: &str) {
    let mut lock = SYSERR_FILE.lock().unwrap();
    if let Some(file) = lock.as_mut() {
        let now = SystemTime::now();
        let timestamp = if let Ok(duration) = now.duration_since(UNIX_EPOCH) {
            let secs = duration.as_secs();
            // Proste formatowanie czasu (YYYY-MM-DD HH:MM:SS) z UNIX_EPOCH.
            // Dla prostoty uzywamy tylko sekund od epoki, bez pelnej biblioteki kalendarzowej,
            // ale dostosujemy sie do ogolnego formatu zeby wygladalo jak trzeba, np. korzystajac
            // z arytmetyki dni, godzin, minut i sekund.
            
            let mut z = secs / 86400 + 719468;
            let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
            let doe = (z - era * 146097) as u64;
            let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
            let y = (yoe + era * 400) as i64;
            let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
            let mp = (5 * doy + 2) / 153;
            let d = doy - (153 * mp + 2) / 5 + 1;
            let m = if mp < 10 { mp + 3 } else { mp - 9 };
            let y = if m <= 2 { y + 1 } else { y };
            
            let seconds_in_day = secs % 86400;
            let hours = seconds_in_day / 3600;
            let minutes = (seconds_in_day % 3600) / 60;
            let seconds = seconds_in_day % 60;
            
            format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, hours, minutes, seconds)
        } else {
            "UNKNOWN TIME".to_string()
        };
        
        let thread_name = thread::current().name().unwrap_or("unknown").to_string();
        
        let formatted_msg = format!("[{}] [{}] {}\n", timestamp, thread_name, msg);
        
        // Zignoruj bledy zapisu - logger nie powinien zglaszac paniki.
        let _ = file.write_all(formatted_msg.as_bytes());
        let _ = file.flush();
    }
}
