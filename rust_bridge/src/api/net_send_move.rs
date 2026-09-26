// Modul wysylania pakietu ruchu klienta do serwera (CG::MOVE)
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::ffi_guard::safe_ffi_call;
use crate::net::cg::move_packet::TPacketCGMove;
use crate::net::send_queue::enqueue_outgoing_packet;
use std::os::raw::{c_float, c_uchar};

// Funkcja C FFI do wysylania pakietu ruchu przez klienta do serwera.
// Buduje strukture TPacketCGMove, serializuje ja do bufora bajtow i przekazuje do bufora wysylkowego.
#[no_mangle]
pub extern "C" fn m2_net_send_move(x: c_float, y: c_float, rot: c_float, move_type: c_uchar) -> bool {
    safe_ffi_call(false, || {
        let normalized_rot = if rot < 0.0 {
            360.0 + rot
        } else if rot > 360.0 {
            rot % 360.0
        } else {
            rot
        };
        let b_rot = ((normalized_rot / 5.0) as u8) % 72;
        let l_x = x as i32;
        let l_y = y as i32;
        let dw_time = crate::timer::get_server_msec();

        let packet = TPacketCGMove::new(move_type, 0, b_rot, l_x, l_y, dw_time);
        let bytes = packet.to_bytes();
        enqueue_outgoing_packet(bytes)
    })
}
