use crate::ffi_guard::safe_ffi_call;

// Funkcja wysylajaca pakiet ataku do serwera
// Zabezpieczona przed panika w C++
#[no_mangle]
pub extern "C" fn m2_net_send_attack(target_vid: u32, attack_type: u8) -> bool {
    safe_ffi_call(false, || {
        // Tymczasowa implementacja
        // Tutaj powinna znalezc sie logika wysylania pakietu
        let _ = target_vid;
        let _ = attack_type;
        true
    })
}
