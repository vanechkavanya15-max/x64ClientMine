// Modul wysylania pakietu podniesienia przedmiotu (CG::ITEM_PICKUP)
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::ffi_guard::safe_ffi_call;
use crate::net::cg::item_pickup::TPacketCGItemPickUp;
use crate::net::send_queue::enqueue_outgoing_packet;

// Funkcja C FFI do wysylania pakietu podniesienia przedmiotu o zadanym VID.
// Buduje fizyczna strukture TPacketCGItemPickUp, serializuje ja do bufora bajtow i przekazuje do bufora wysylkowego.
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_pickup(vid: u32) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGItemPickUp::new(vid);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}
