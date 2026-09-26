use std::sync::{Mutex, OnceLock};

use m2_client::packet_sequence_guard_v2::*;

// Ensure tests that modify global FFI state run sequentially
static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_mutex() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

#[test]
fn test_sequence_guard_initialization() {
    let _lock = get_test_mutex().lock().unwrap();
    let guard = PacketSequenceGuard::new();
    
    // Na poczatku, bez tasowania tablica powinna miec wartosci 0..255
    let mut check_guard = guard;
    for i in 0..256 {
        assert_eq!(check_guard.get_next_send_sequence(), i as u8);
    }
}

#[test]
fn test_sequence_guard_shuffle() {
    let _lock = get_test_mutex().lock().unwrap();
    let mut guard = PacketSequenceGuard::new();
    
    // Szyfrujemy/tasujemy z wlasnym seedem
    let seed = 12345;
    guard.shuffle(seed);
    
    // Tablica nie powinna byc juz rosnaca 0..255
    let mut is_different = false;
    for i in 0..256 {
        let seq = guard.get_next_send_sequence();
        if seq != i as u8 {
            is_different = true;
            break;
        }
    }
    
    assert!(is_different, "Tablica po tasowaniu powinna byc inna od wejsciowej");
}

#[test]
fn test_verify_sequence_valid() {
    let _lock = get_test_mutex().lock().unwrap();
    let mut guard = PacketSequenceGuard::new();
    
    // Brak tasowania: indeks 0 oczekuje 0, indeks 1 oczekuje 1
    assert!(guard.verify_sequence(0));
    assert!(guard.verify_sequence(1));
    assert!(guard.verify_sequence(2));
}

#[test]
fn test_verify_sequence_invalid() {
    let _lock = get_test_mutex().lock().unwrap();
    let mut guard = PacketSequenceGuard::new();
    
    // Brak tasowania, oczekiwane 0
    // Podajemy zla wartosc 100
    assert!(!guard.verify_sequence(100));
    
    // Stan nie powinien sie zaktualizowac
    // Kolejna proba z wlasciwym dla 0 indeksu znow powinna przejsc
    assert!(guard.verify_sequence(0));
}

#[test]
fn test_verify_sequence_wrap_around() {
    let _lock = get_test_mutex().lock().unwrap();
    let mut guard = PacketSequenceGuard::new();
    
    // Odczytujemy wszystkie 256 wartosci zeby spowodowac przewiniecie
    for i in 0..256 {
        assert!(guard.verify_sequence(i as u8));
    }
    
    // Po 256 (indeks zawija do 0), znowu oczekiwane jest 0
    assert!(guard.verify_sequence(0));
}

#[test]
fn test_get_send_sequence_wrap_around() {
    let _lock = get_test_mutex().lock().unwrap();
    let mut guard = PacketSequenceGuard::new();
    
    // Pobieramy 256 wartosci
    for i in 0..256 {
        assert_eq!(guard.get_next_send_sequence(), i as u8);
    }
    
    // 257. wywolanie powinno wrocic do poczatku tablicy (czyli znowu 0 przed shuffle)
    assert_eq!(guard.get_next_send_sequence(), 0);
}

#[test]
fn test_ffi_exports_valid() {
    let _lock = get_test_mutex().lock().unwrap();
    
    // Reset przez FFI shuffle
    let seed = 999;
    m2_net_sequence_shuffle(seed);
    
    // Oczekujemy ze wyslanie pierwszej sekwencji pasuje do pierwszego wyslania C++
    let first_seq = m2_net_get_send_sequence();
    
    // To byla instancja uzywana przez FFI
    // Sprawdzmy weryfikacje FFI (zakladajac ze seed przetasowal i niekoniecznie zaczyna od first_seq dla RECV,
    // ale zarowno REC i SEND wspoldziela te sama tablice z tym samym indexem 0 na starcie, wiec 
    // m2_net_verify_sequence() dla recv_index=0 powinno dac to samo co send_index=0 z m2_net_get_send_sequence).
    
    // Dla testu mozemy po prostu zapytac guard bezposrednio jaka ma tablice, ale sprawdzmy poprzez uzycie tego co dostalismy z get
    assert!(m2_net_verify_sequence(first_seq), "Sequence verified via FFI should match since send and recv share the same shuffled table structure");
    
    // Po uzyciu recv_index wskazuje na 1.
    // Drugie wywolanie verify z inna wartoscia powinno zawiesc.
    assert!(!m2_net_verify_sequence(100)); // zakladajac ze po shuffle 100 nie wyladowalo na indeksie 1. W rzadkim przypadku mogloby.
}

#[test]
fn test_ffi_inject_sequence() {
    let _lock = get_test_mutex().lock().unwrap();
    
    // Zresetowanie stanu przez pelny loop shuffle
    m2_net_sequence_shuffle(42);
    
    let mut pkt = TPacketCGSequence {
        header: 0xFF,
        sequence: 0,
    };
    
    let expected_seq = m2_net_get_send_sequence();
    
    // Przywracamy pozycje z powrotem o 1 cofac za pomoca wewnetrznego guard
    // Ale prosciej po prostu odczytac nowa (wstrzyknieta)
    
    let success = m2_net_inject_sequence(&mut pkt as *mut _);
    assert!(success);
    
    // Nie mamy dostepu do obiektu przed wstrzyknieciem z poziomu testu ffi bez importowania guard 
    // jednakze mozemy zaobserwowac ze wstrzykniecie to po prostu wywolanie get_next_send_sequence()
    // i zapis do pola "sequence".
    // Wiec jezeli pobierzemy get_next_send_sequence(), powinno byc kolejne po tym wstrzyknietym.
    
    // Ale weryfikujemy ze pole zostalo nadpisane nie-zerowa / deterministyczna wartoscia
    assert_ne!(pkt.sequence, 0, "Packet sequence should be modified (assuming it's not 0 by chance after shuffle for second element)");
}

#[test]
fn test_ffi_inject_sequence_null() {
    let _lock = get_test_mutex().lock().unwrap();
    let success = m2_net_inject_sequence(std::ptr::null_mut());
    assert!(!success, "Should gracefully handle null pointers");
}
