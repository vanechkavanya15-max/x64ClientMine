use std::sync::{Mutex, OnceLock};
use m2_client_bridge::ffi_guard::safe_ffi_call;

/// Struktura reprezentujaca rejestr sekwencji pakietow (Packet Sequence Guard)
/// Zapobiega wstrzykiwaniu obcych pakietow poprzez tasowana tablice stanow.
pub struct PacketSequenceGuard {
    sequence_table: [u8; 256],
    recv_index: u8,
    send_index: u8,
}

impl PacketSequenceGuard {
    /// Inicjalizuje nowa straz sekwencji
    pub fn new() -> Self {
        let mut table = [0u8; 256];
        for (i, v) in table.iter_mut().enumerate() {
            *v = i as u8;
        }

        Self {
            sequence_table: table,
            recv_index: 0,
            send_index: 0,
        }
    }

    /// Tasuje tablice na podstawie podanego seeda dla polaczenia
    /// Odtwarza logike per polaczenie bez zaleznosci od zewnetrznych bibliotek C++
    pub fn shuffle(&mut self, seed: u32) {
        let mut prng_state = seed;
        for i in (1..256).rev() {
            // LCG formula
            prng_state = prng_state.wrapping_mul(214013).wrapping_add(2531011);
            let j = ((prng_state >> 16) & 0xFF) as usize % (i + 1);
            self.sequence_table.swap(i, j);
        }
        self.recv_index = 0;
        self.send_index = 0;
    }

    /// Zwraca nastepny bajt do wstrzykniecia przy wysylaniu pakietu (CG) i inkrementuje indeks
    pub fn get_next_send_sequence(&mut self) -> u8 {
        let seq = self.sequence_table[self.send_index as usize];
        self.send_index = self.send_index.wrapping_add(1);
        seq
    }

    /// Weryfikuje przychodzacy bajt (GC) z oczekiwanym indeksem i inkrementuje indeks w razie sukcesu
    pub fn verify_sequence(&mut self, seq: u8) -> bool {
        let expected = self.sequence_table[self.recv_index as usize];
        if seq == expected {
            self.recv_index = self.recv_index.wrapping_add(1);
            true
        } else {
            false
        }
    }
    
    // Tylko do testow zeby wymusic blad i reset stanu
    pub fn reset_test_state(&mut self) {
        self.recv_index = 0;
        self.send_index = 0;
    }
}

// Globalny menedzer
static SEQUENCE_GUARD: OnceLock<Mutex<PacketSequenceGuard>> = OnceLock::new();

fn get_sequence_guard() -> &'static Mutex<PacketSequenceGuard> {
    SEQUENCE_GUARD.get_or_init(|| Mutex::new(PacketSequenceGuard::new()))
}

// -----------------------------------------------------------------------------
// Struktury pakietow (wg wymogu braku zaleznosci m2-packets i zgodnosci z Packet.h)
// -----------------------------------------------------------------------------

#[repr(C, packed)]
pub struct TPacketCGSequence {
    pub header: u8,
    pub sequence: u8,
}

// -----------------------------------------------------------------------------
// FFI C++ Exports
// -----------------------------------------------------------------------------

/// Wstrzykuje bajt sekwencji do naglowka pakieta
#[no_mangle]
pub extern "C" fn m2_net_inject_sequence(packet_ptr: *mut TPacketCGSequence) -> bool {
    safe_ffi_call(false, || {
        if packet_ptr.is_null() {
            return false;
        }
        let mut guard = get_sequence_guard().lock().unwrap();
        let seq = guard.get_next_send_sequence();
        unsafe {
            (*packet_ptr).sequence = seq;
        }
        true
    })
}

/// Eksport FFI: weryfikuje sekwencje przychodzacego pakietu
#[no_mangle]
pub extern "C" fn m2_net_verify_sequence(seq: u8) -> bool {
    safe_ffi_call(false, || {
        let mut guard = get_sequence_guard().lock().unwrap();
        guard.verify_sequence(seq)
    })
}

/// Eksport FFI: inicjalizuje tasowanie sekwencji z wybranym seedem
#[no_mangle]
pub extern "C" fn m2_net_sequence_shuffle(seed: u32) {
    safe_ffi_call((), || {
        let mut guard = get_sequence_guard().lock().unwrap();
        guard.shuffle(seed);
    })
}

/// Eksport FFI: pobiera kolejny sekwencyjny bajt do wstrzykniecia (CG packet)
#[no_mangle]
pub extern "C" fn m2_net_get_send_sequence() -> u8 {
    safe_ffi_call(0, || {
        let mut guard = get_sequence_guard().lock().unwrap();
        guard.get_next_send_sequence()
    })
}
