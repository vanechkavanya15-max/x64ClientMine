use crossbeam_channel::{unbounded, Receiver, Sender};
use std::slice;
use std::ptr;
use crate::ffi_guard::safe_ffi_call;

/// Struktura reprezentujaca bezblokowa kolejke pakietow
pub struct NetChannel {
    sender: Sender<Vec<u8>>,
    receiver: Receiver<Vec<u8>>,
}

impl NetChannel {
    pub fn new() -> Self {
        let (s, r) = unbounded();
        NetChannel {
            sender: s,
            receiver: r,
        }
    }
}

/// Tworzy nowy kanal sieciowy
#[no_mangle]
pub extern "C" fn m2_net_channel_create() -> *mut NetChannel {
    safe_ffi_call(ptr::null_mut(), || {
        let channel = Box::new(NetChannel::new());
        Box::into_raw(channel)
    })
}

/// Niszczy kanal sieciowy
#[no_mangle]
pub extern "C" fn m2_net_channel_destroy(channel: *mut NetChannel) {
    safe_ffi_call((), || {
        if !channel.is_null() {
            unsafe {
                let _ = Box::from_raw(channel);
            }
        }
    })
}

/// Wysyla pakiet do kanalu
#[no_mangle]
pub extern "C" fn m2_net_channel_send(
    channel: *mut NetChannel,
    data: *const u8,
    len: usize,
) -> bool {
    safe_ffi_call(false, || {
        if channel.is_null() || data.is_null() || len == 0 {
            return false;
        }

        let ch = unsafe { &*channel };
        let slice = unsafe { slice::from_raw_parts(data, len) };
        let vec_data = slice.to_vec();

        match ch.sender.send(vec_data) {
            Ok(_) => true,
            Err(_) => false,
        }
    })
}

/// Odbiera pakiet z kanalu (nie blokuje)
/// Zwraca true jesli udalo sie odebrac pakiet.
/// out_data zostanie ustawione na wskaznik do bufora (musi byc zwolnione przez m2_net_channel_free_packet).
/// out_len zostanie ustawione na dlugosc bufora.
#[no_mangle]
pub extern "C" fn m2_net_channel_recv(
    channel: *mut NetChannel,
    out_data: *mut *mut u8,
    out_len: *mut usize,
) -> bool {
    safe_ffi_call(false, || {
        if channel.is_null() || out_data.is_null() || out_len.is_null() {
            return false;
        }

        let ch = unsafe { &*channel };

        match ch.receiver.try_recv() {
            Ok(mut packet) => {
                let len = packet.len();
                packet.shrink_to_fit();
                let ptr = packet.as_mut_ptr();
                std::mem::forget(packet);

                unsafe {
                    *out_data = ptr;
                    *out_len = len;
                }
                true
            }
            Err(_) => false, // pusty kanal lub rozlaczony
        }
    })
}

/// Zwalnia pamiec po odebranym pakiecie
#[no_mangle]
pub extern "C" fn m2_net_channel_free_packet(data: *mut u8, len: usize) {
    safe_ffi_call((), || {
        if !data.is_null() && len > 0 {
            unsafe {
                let _ = Vec::from_raw_parts(data, len, len);
            }
        }
    })
}
