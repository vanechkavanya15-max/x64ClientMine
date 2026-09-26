// Modul wysylania pakietu uzycia przedmiotu (CG::ITEM_USE)
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::ffi_guard::safe_ffi_call;
use crate::net::cg::item_use::TPacketCGItemUse;
use crate::net::send_queue::enqueue_outgoing_packet;

// Funkcja C FFI do wysylania pakietu uzycia przedmiotu.
// Argumenty: window - identyfikator okna, slot - indeks komorki.
// Buduje fizyczna strukture TPacketCGItemUse, serializuje ja do bajtow i przekazuje do bufora wysylkowego.
#[no_mangle]
pub extern "C" fn m2_net_send_use_item(window: u8, slot: u16) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGItemUse::new(window, slot);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}
