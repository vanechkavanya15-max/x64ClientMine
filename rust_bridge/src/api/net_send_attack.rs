// Modul wysylania pakietu ataku do serwera (CG::ATTACK)
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::ffi_guard::safe_ffi_call;
use crate::net::cg::attack::TPacketCGAttack;
use crate::net::send_queue::enqueue_outgoing_packet;

// Funkcja wysylajaca pakiet ataku do serwera.
// Buduje fizyczna strukture TPacketCGAttack, serializuje ja do bufora bajtow i przekazuje do bufora wysylkowego.
#[no_mangle]
pub extern "C" fn m2_net_send_attack(target_vid: u32, attack_type: u8) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGAttack::new_simple(attack_type, target_vid);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}
