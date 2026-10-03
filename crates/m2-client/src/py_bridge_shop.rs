use std::os::raw::c_char;
use crate::recv_shop_logic_v2::{SHOP_INSTANCE, SHOP_HOST_ITEM_MAX_NUM};
use crate::send_shop_exchange_v2::TItemPos;

pub const HEADER_CG_MYSHOP: u16 = 0x0802;
pub const SHOP_SIGN_MAX_LEN: usize = 32;

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TShopItemTable {
    pub vnum: u32,
    pub count: u8,
    pub pos: TItemPos,
    pub price: u32,
    pub display_pos: u8,
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGMyShop {
    pub header: u16,
    pub length: u16,
    pub sz_sign: [u8; SHOP_SIGN_MAX_LEN + 1],
    pub b_count: u8,
}

#[no_mangle]
pub extern "C" fn m2_py_shop_is_open() -> bool {
    if let Ok(shop) = SHOP_INSTANCE.read() {
        shop.is_open
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_get_item_count() -> usize {
    if let Ok(shop) = SHOP_INSTANCE.read() {
        shop.items.iter().filter(|item| item.vnum != 0).count()
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_get_item_id(slot: usize) -> u32 {
    if slot >= SHOP_HOST_ITEM_MAX_NUM {
        return 0;
    }
    if let Ok(shop) = SHOP_INSTANCE.read() {
        shop.items[slot].vnum
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_get_item_price(slot: usize) -> u32 {
    if slot >= SHOP_HOST_ITEM_MAX_NUM {
        return 0;
    }
    if let Ok(shop) = SHOP_INSTANCE.read() {
        shop.items[slot].price
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_build_private_shop_legacy(
    title_ptr: *const u8,
    title_len: usize,
    item_stock_ptr: *const TShopItemTable,
    item_count: usize,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() || (item_count > 0 && item_stock_ptr.is_null()) {
        return 0;
    }

    let item_count_limited = std::cmp::min(item_count, 39); // bCount max 39 in original client comment

    let mut packet = TPacketCGMyShop {
        header: HEADER_CG_MYSHOP,
        length: (std::mem::size_of::<TPacketCGMyShop>() + std::mem::size_of::<TShopItemTable>() * item_count_limited) as u16,
        sz_sign: [0; SHOP_SIGN_MAX_LEN + 1],
        b_count: item_count_limited as u8,
    };

    if !title_ptr.is_null() && title_len > 0 {
        let title_bytes = unsafe { std::slice::from_raw_parts(title_ptr, title_len) };
        let copy_len = std::cmp::min(title_bytes.len(), SHOP_SIGN_MAX_LEN);
        packet.sz_sign[..copy_len].copy_from_slice(&title_bytes[..copy_len]);
    }

    let mut offset = 0;
    unsafe {
        std::ptr::copy_nonoverlapping(
            &packet as *const _ as *const u8,
            out_buf.add(offset),
            std::mem::size_of::<TPacketCGMyShop>(),
        );
        offset += std::mem::size_of::<TPacketCGMyShop>();

        if item_count_limited > 0 {
            std::ptr::copy_nonoverlapping(
                item_stock_ptr as *const u8,
                out_buf.add(offset),
                std::mem::size_of::<TShopItemTable>() * item_count_limited,
            );
            offset += std::mem::size_of::<TShopItemTable>() * item_count_limited;
        }
    }

    offset
}

#[no_mangle]
pub extern "C" fn m2_py_shop_open() {
    if let Ok(mut shop) = SHOP_INSTANCE.write() {
        shop.is_open = true;
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_close() {
    if let Ok(mut shop) = SHOP_INSTANCE.write() {
        shop.is_open = false;
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_buy(pos: u32) {
    use std::io::Write;
    let mut out_buf = [0u8; 16];
    let len = crate::send_shop_complete_packets::m2_send_shop_buy(pos as u8, 1, out_buf.as_mut_ptr());
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc);
                let _ = stream.write_all(&enc);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_sell(pos: u32, count: u32) {
    use std::io::Write;
    let mut out_buf = [0u8; 16];
    let len = crate::send_shop_complete_packets::m2_send_shop_sell_new(pos as u8, count as u8, out_buf.as_mut_ptr());
    if len > 0 {
        if let Ok(mut sock_guard) = crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET.lock() {
            if let Some((ref mut stream, ref mut cipher)) = *sock_guard {
                let mut enc = out_buf[..len].to_vec();
                cipher.encrypt_in_place(&mut enc);
                let _ = stream.write_all(&enc);
            }
        }
    }
}

