// Modul wysylania pakietu uzycia umiejetnosci (CG::USE_SKILL)
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::ffi_guard::safe_ffi_call;
use crate::net::cg::use_skill::TPacketCGUseSkill;
use crate::net::send_queue::enqueue_outgoing_packet;

// Funkcja C FFI do wysylania pakietu uzycia umiejetnosci.
// Buduje fizyczna strukture TPacketCGUseSkill, serializuje ja do bajtow i przekazuje do bufora wysylkowego.
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_use_skill(skill_vnum: u32, target_vid: u32) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGUseSkill::new(skill_vnum, target_vid);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}
