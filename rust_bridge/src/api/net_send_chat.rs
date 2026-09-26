// Modul wysylania pakietu czatu (CG::CHAT)
// Komentarze wylacznie po polsku bez polskich znakow diakrytycznych.

use crate::ffi_guard::safe_ffi_call;
use crate::net::cg::chat::TPacketCGChat;
use crate::net::send_queue::enqueue_outgoing_packet;
use std::ffi::CStr;
use std::os::raw::c_char;

// Funkcja C FFI do wysylania wiadomosci czatu do serwera (CG::CHAT).
// Buduje fizyczna strukture TPacketCGChat, dolacza tresc wiadomosci tekstowej,
// serializuje calosc do wektora bajtow i przekazuje do bufora wysylkowego.
#[no_mangle]
pub unsafe extern "C" fn m2_net_send_chat(chat_type: u8, msg: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if msg.is_null() {
            return false;
        }

        let c_str = unsafe { CStr::from_ptr(msg) };
        let msg_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let bytes = TPacketCGChat::build_with_message(chat_type, msg_str);
        enqueue_outgoing_packet(bytes)
    })
}
