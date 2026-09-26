use m2_client::recv_target_info_v2::{
    clear_target, get_target_state, m2_net_get_target_hp, parse_target_packet, TPacketGCTarget,
};
use std::ptr;
use std::sync::{Mutex, OnceLock};

// Wspoldzielony Mutex do synchronizacji testow mutujacych globalny stan
static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

#[test]
fn test_parse_valid_target_packet() {
    let _guard = get_test_lock();
    clear_target();

    let packet = TPacketGCTarget {
        header: 123,
        length: 9,
        dwVID: 4567,
        bHPPercent: 85,
    };

    parse_target_packet(&packet);

    let state = get_target_state().lock().unwrap();
    assert_eq!(state.target_vid, 4567);
    assert_eq!(state.hp_percent, 85);
}

#[test]
fn test_ffi_get_target_success() {
    let _guard = get_test_lock();
    clear_target();

    let packet = TPacketGCTarget {
        header: 1,
        length: 9,
        dwVID: 9999,
        bHPPercent: 42,
    };
    parse_target_packet(&packet);

    let mut out_vid: u32 = 0;
    let mut out_hp_pct: u8 = 0;

    let result = unsafe { m2_net_get_target_hp(&mut out_vid, &mut out_hp_pct) };

    assert!(result);
    assert_eq!(out_vid, 9999);
    assert_eq!(out_hp_pct, 42);
}

#[test]
fn test_ffi_get_target_no_target() {
    let _guard = get_test_lock();
    clear_target();

    let mut out_vid: u32 = 0;
    let mut out_hp_pct: u8 = 0;

    let result = unsafe { m2_net_get_target_hp(&mut out_vid, &mut out_hp_pct) };

    assert!(!result);
    assert_eq!(out_vid, 0); // Pozostaje bez zmian
    assert_eq!(out_hp_pct, 0); // Pozostaje bez zmian
}

#[test]
fn test_ffi_get_target_null_pointers() {
    let _guard = get_test_lock();
    clear_target();

    // Najpierw ustawiamy poprawny cel, zeby pominac test "brak celu"
    let packet = TPacketGCTarget {
        header: 1,
        length: 9,
        dwVID: 100,
        bHPPercent: 50,
    };
    parse_target_packet(&packet);

    let mut valid_vid = 0;
    let mut valid_hp = 0;

    // Test 1: Pierwszy wskaznik null
    let res1 = unsafe { m2_net_get_target_hp(ptr::null_mut(), &mut valid_hp) };
    assert!(!res1);

    // Test 2: Drugi wskaznik null
    let res2 = unsafe { m2_net_get_target_hp(&mut valid_vid, ptr::null_mut()) };
    assert!(!res2);

    // Test 3: Oba wskazniki null
    let res3 = unsafe { m2_net_get_target_hp(ptr::null_mut(), ptr::null_mut()) };
    assert!(!res3);
}

#[test]
fn test_packet_endianness_safety() {
    let _guard = get_test_lock();
    clear_target();

    // Wymuszamy big endian / little endian w zaleznosci od architektury, by sprawdzic from_le_bytes
    // Pakiet przychodzi z sieci z u32, my mozemy symulowac rozne przypadki.
    let dwvid_be = u32::from_be_bytes([0x12, 0x34, 0x56, 0x78]); // Przekazujemy taka wartosc
    let packet = TPacketGCTarget {
        header: 0,
        length: 0,
        dwVID: dwvid_be,
        bHPPercent: 10,
    };

    parse_target_packet(&packet);
    let state = get_target_state().lock().unwrap();

    // Sprawdzamy czy poprawnie przeksztalcilo z powrotem.
    assert_eq!(state.target_vid, u32::from_le_bytes(dwvid_be.to_le_bytes()));
    assert_eq!(state.hp_percent, 10);
}

#[test]
fn test_clear_target() {
    let _guard = get_test_lock();
    clear_target();

    let packet = TPacketGCTarget {
        header: 0,
        length: 0,
        dwVID: 1234,
        bHPPercent: 100,
    };
    parse_target_packet(&packet);

    let state_before = get_target_state().lock().unwrap();
    assert_eq!(state_before.target_vid, 1234);
    drop(state_before);

    clear_target();

    let state_after = get_target_state().lock().unwrap();
    assert_eq!(state_after.target_vid, 0);
    assert_eq!(state_after.hp_percent, 0);
}
