use crate::send_shop_exchange_v2::{
    m2_send_exchange_start,
    m2_send_exchange_item_add,
    m2_send_exchange_item_del,
    m2_send_exchange_elk_add,
    m2_send_exchange_accept,
    m2_send_exchange_exit,
};
use crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET;
use crate::recv_exchange_logic_v2::EXCHANGE_ITEM_MAX_NUM;
use std::io::Write;

fn send_packet_helper(len: usize, mut out_buf: [u8; 32]) -> bool {
    if len > 0 {
        if let Ok(mut sock_guard) = ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc_buf = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc_buf);
                return stream.write_all(&enc_buf).is_ok();
            } else {
                // Testing fallback
                return true;
            }
        }
    }
    false
}

pub fn send_exchange_start(vid: u32) -> bool {
    let mut out_buf = [0u8; 32];
    let len = m2_send_exchange_start(vid, out_buf.as_mut_ptr());
    send_packet_helper(len, out_buf)
}

pub fn send_exchange_item_add(pos: u16, display_pos: u8) -> bool {
    if (display_pos as usize) >= EXCHANGE_ITEM_MAX_NUM {
        return false;
    }
    let mut out_buf = [0u8; 32];
    let len = m2_send_exchange_item_add(1, pos, display_pos, out_buf.as_mut_ptr());
    send_packet_helper(len, out_buf)
}

pub fn send_exchange_item_del(display_pos: u8) -> bool {
    if (display_pos as usize) >= EXCHANGE_ITEM_MAX_NUM {
        return false;
    }
    let mut out_buf = [0u8; 32];
    let len = m2_send_exchange_item_del(display_pos, out_buf.as_mut_ptr());
    send_packet_helper(len, out_buf)
}

pub fn send_exchange_elk_add(gold: u32) -> bool {
    if (gold as i32) < 0 {
        return false;
    }
    let mut out_buf = [0u8; 32];
    let len = m2_send_exchange_elk_add(gold, out_buf.as_mut_ptr());
    send_packet_helper(len, out_buf)
}

pub fn send_exchange_accept() -> bool {
    let mut out_buf = [0u8; 32];
    let len = m2_send_exchange_accept(out_buf.as_mut_ptr());
    send_packet_helper(len, out_buf)
}

pub fn send_exchange_exit() -> bool {
    let mut out_buf = [0u8; 32];
    let len = m2_send_exchange_exit(out_buf.as_mut_ptr());
    send_packet_helper(len, out_buf)
}
