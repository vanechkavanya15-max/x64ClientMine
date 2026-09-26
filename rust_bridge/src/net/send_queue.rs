// Modul kolejki bufora pakietow wychodzacych (CG).
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use std::sync::Mutex;
use once_cell::sync::Lazy;

static OUTGOING_PACKETS: Lazy<Mutex<Vec<Vec<u8>>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// Dodaje zserializowany pakiet do bufora pakietow wychodzacych.
pub fn enqueue_outgoing_packet(data: Vec<u8>) -> bool {
    if data.len() >= 4 {
        let header = u16::from_le_bytes([data[0], data[1]]);
        let length = u16::from_le_bytes([data[2], data[3]]);
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("rust_bridge_live.log") {
            let _ = writeln!(f, "[RUST NET SEND] Enqueued Packet: opcode=0x{:04X}, len={} B, total_bytes={}",
                header, length, data.len());
        }
    }
    if let Ok(mut queue) = OUTGOING_PACKETS.lock() {
        queue.push(data);
        true
    } else {
        false
    }
}

/// Zwraca liczbe pakietow oczekujacych w kolejce.
pub fn outgoing_packet_count() -> usize {
    OUTGOING_PACKETS.lock().map(|q| q.len()).unwrap_or(0)
}

/// Pobiera ostatni zserializowany pakiet z kolejki.
pub fn get_last_outgoing_packet() -> Option<Vec<u8>> {
    OUTGOING_PACKETS.lock().ok().and_then(|q| q.last().cloned())
}

/// Pobiera i usuwa wszystkie pakiety z kolejki.
pub fn drain_outgoing_packets() -> Vec<Vec<u8>> {
    if let Ok(mut queue) = OUTGOING_PACKETS.lock() {
        queue.drain(..).collect()
    } else {
        Vec::new()
    }
}

/// Czysci kolejke pakietow wychodzacych.
pub fn clear_outgoing_packets() {
    if let Ok(mut queue) = OUTGOING_PACKETS.lock() {
        queue.clear();
    }
}

#[no_mangle]
pub extern "C" fn m2_net_get_outgoing_count() -> usize {
    outgoing_packet_count()
}

#[no_mangle]
pub extern "C" fn m2_net_clear_outgoing() {
    clear_outgoing_packets();
}
