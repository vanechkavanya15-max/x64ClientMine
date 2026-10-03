use std::ptr;
use std::mem::size_of;

#[cfg(not(test))]
use crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET;
#[cfg(test)]
use crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET;

use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::io::Write;
use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;

pub const HEADER_CG_SHOP: u16 = 0x0801;

pub const SHOP_SUB_END: u8 = 0;
pub const SHOP_SUB_BUY: u8 = 1;
pub const SHOP_SUB_SELL: u8 = 2;
pub const SHOP_SUB_SELL2: u8 = 3;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TPacketCGShop {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
}

fn send_packet(bytes: &[u8]) {
    // Write directly to socket if available.
    if let Ok(mut guard) = ACTIVE_GAME_SOCKET.lock() {
        if let Some((stream, cipher)) = guard.as_mut() {
            let mut encrypted_bytes = bytes.to_vec();
            cipher.encrypt_in_place(&mut encrypted_bytes);
            let _ = stream.write_all(&encrypted_bytes);
            return;
        }
    }
    // Fallback to queue if no direct socket or failed.
    enqueue_outgoing_packet(bytes.to_vec());
}

#[no_mangle]
pub extern "C" fn m2_send_shop_end(out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        let packet = TPacketCGShop {
            header: HEADER_CG_SHOP,
            length: size_of::<TPacketCGShop>() as u16,
            subheader: SHOP_SUB_END,
        };

        let mut bytes = Vec::with_capacity(size_of::<TPacketCGShop>());
        let packet_bytes: [u8; size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
        bytes.extend_from_slice(&packet_bytes);
        
        send_packet(&bytes);

        if !out_buf.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf, bytes.len());
            }
        }
        bytes.len()
    })
}

#[no_mangle]
pub extern "C" fn m2_send_shop_buy(pos: u8, count: u8, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        let packet = TPacketCGShop {
            header: HEADER_CG_SHOP,
            length: (size_of::<TPacketCGShop>() + 2) as u16,
            subheader: SHOP_SUB_BUY,
        };

        let mut bytes = Vec::with_capacity(size_of::<TPacketCGShop>() + 2);
        let packet_bytes: [u8; size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
        bytes.extend_from_slice(&packet_bytes);
        bytes.push(count);
        bytes.push(pos);
        
        send_packet(&bytes);

        if !out_buf.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf, bytes.len());
            }
        }
        bytes.len()
    })
}

#[no_mangle]
pub extern "C" fn m2_send_shop_sell(pos: u8, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        let packet = TPacketCGShop {
            header: HEADER_CG_SHOP,
            length: (size_of::<TPacketCGShop>() + 1) as u16,
            subheader: SHOP_SUB_SELL,
        };

        let mut bytes = Vec::with_capacity(size_of::<TPacketCGShop>() + 1);
        let packet_bytes: [u8; size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
        bytes.extend_from_slice(&packet_bytes);
        bytes.push(pos);
        
        send_packet(&bytes);

        if !out_buf.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf, bytes.len());
            }
        }
        bytes.len()
    })
}

#[no_mangle]
pub extern "C" fn m2_send_shop_sell_new(pos: u8, count: u8, out_buf: *mut u8) -> usize {
    safe_ffi_call(0, || {
        let packet = TPacketCGShop {
            header: HEADER_CG_SHOP,
            length: (size_of::<TPacketCGShop>() + 2) as u16,
            subheader: SHOP_SUB_SELL2,
        };

        let mut bytes = Vec::with_capacity(size_of::<TPacketCGShop>() + 2);
        let packet_bytes: [u8; size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
        bytes.extend_from_slice(&packet_bytes);
        bytes.push(pos);
        bytes.push(count);
        
        send_packet(&bytes);

        if !out_buf.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf, bytes.len());
            }
        }
        bytes.len()
    })
}
