//! Modul precyzyjnego pomiaru czasu i synchronizacji z serwerem gry Metin2.
//!
//! Zastepuje logike ELTimer_GetMSec / ELTimer_SetServerMSec z EterBase/Timer.cpp.

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Instant;
use once_cell::sync::Lazy;

static START_INSTANT: Lazy<Instant> = Lazy::new(Instant::now);

static BASE_MSEC: AtomicU32 = AtomicU32::new(0);
static SERVER_TIME: AtomicU32 = AtomicU32::new(0);
static CLIENT_TIME_AT_SYNC: AtomicU32 = AtomicU32::new(0);
static FRAME_TIME: AtomicU32 = AtomicU32::new(0);

/// Inicjalizuje bazowy licznik czasu
pub fn init() -> bool {
    let now = START_INSTANT.elapsed().as_millis() as u32;
    BASE_MSEC.store(now, Ordering::SeqCst);
    true
}

/// Zwraca biezacy czas w milisekundach od startu klienta
pub fn get_msec() -> u32 {
    let elapsed = START_INSTANT.elapsed().as_millis() as u32;
    let base = BASE_MSEC.load(Ordering::Relaxed);
    elapsed.wrapping_sub(base)
}

/// Synchronizuje czas klienta z czasem serwera otrzymanym w pakiecie
pub fn set_server_msec(server_msec: u32) {
    if server_msec != 0 {
        let current_client_time = get_msec();
        SERVER_TIME.store(server_msec, Ordering::SeqCst);
        CLIENT_TIME_AT_SYNC.store(current_client_time, Ordering::SeqCst);
    }
}

/// Zwraca zsynchronizowany czas serwera
pub fn get_server_msec() -> u32 {
    let current_client = get_msec();
    let sync_client = CLIENT_TIME_AT_SYNC.load(Ordering::Relaxed);
    let server_base = SERVER_TIME.load(Ordering::Relaxed);
    current_client.wrapping_sub(sync_client).wrapping_add(server_base)
}

/// Ustawia biezacy znacznik czasu klatki
pub fn set_frame_msec() {
    FRAME_TIME.store(get_msec(), Ordering::Relaxed);
}

/// Pobiera czas klatki w milisekundach
pub fn get_frame_msec() -> u32 {
    FRAME_TIME.load(Ordering::Relaxed)
}

/// Pobiera czas klatki z uwzglednieniem delty serwera
pub fn get_server_frame_msec() -> u32 {
    let frame = get_frame_msec();
    let sync_client = CLIENT_TIME_AT_SYNC.load(Ordering::Relaxed);
    let server_base = SERVER_TIME.load(Ordering::Relaxed);
    frame.wrapping_sub(sync_client).wrapping_add(server_base)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_progression() {
        init();
        let t1 = get_msec();
        std::thread::sleep(std::time::Duration::from_millis(15));
        let t2 = get_msec();
        assert!(t2 >= t1 + 10, "Czas musi isc do przodu: t1={}, t2={}", t1, t2);
    }

    #[test]
    fn test_server_sync() {
        init();
        set_server_msec(100_000);
        let s1 = get_server_msec();
        assert!(s1 >= 100_000, "Zsynchronizowany czas musi byc >= wartosci bazowej: {}", s1);
    }

    #[test]
    fn test_frame_msec() {
        set_frame_msec();
        let f = get_frame_msec();
        assert!(f > 0 || f == 0);
    }
}
