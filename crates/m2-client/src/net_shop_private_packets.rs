use m2_client_bridge::ffi_guard::safe_ffi_call;
use crate::ui_bridge_ffi::ACTIVE_GAME_SOCKET;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::io::Write;

pub const HEADER_CG_SHOP: u16 = 0x0801;
pub const HEADER_CG_MYSHOP: u16 = 0x0802;

pub const SHOP_SUB_END: u8 = 0;
pub const SHOP_SUB_BUY: u8 = 1;
pub const SHOP_SUB_SELL: u8 = 2;
pub const SHOP_SUB_SELL2: u8 = 3;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TShopItemTable {
    pub vnum: u32,
    pub count: u8,
    pub pos: TItemPos,
    pub price: u32,
    pub display_pos: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketCGMyShop {
    pub header: u16,
    pub length: u16,
    pub szSign: [u8; 33],
    pub bCount: u8,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TPacketCGShop {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
}

fn send_packet(bytes: &[u8]) -> bool {
    let mut state = ACTIVE_GAME_SOCKET.lock().unwrap();
    if let Some(socket_tuple) = state.as_mut() {
        if socket_tuple.0.write_all(bytes).is_err() {
            // failed to send
            return false;
        }
        return true;
    }
    true // If no socket, C++ assumes success for offline acting unless Send returns false, we return true to not crash.
}

#[no_mangle]
pub extern "C" fn m2_net_send_build_private_shop_packet(
    c_szName: *const c_char,
    c_rSellingItemStock: *const TShopItemTable,
    stock_count: usize,
) -> bool {
    safe_ffi_call(false, || {
        if c_szName.is_null() || (c_rSellingItemStock.is_null() && stock_count > 0) {
            return false;
        }

        let name = unsafe { CStr::from_ptr(c_szName).to_bytes() };
        let mut szSign = [0u8; 33];
        let len = std::cmp::min(name.len(), 32);
        szSign[..len].copy_from_slice(&name[..len]);

        let items = if stock_count > 0 {
            unsafe { std::slice::from_raw_parts(c_rSellingItemStock, stock_count) }
        } else {
            &[]
        };

        let packet = TPacketCGMyShop {
            header: HEADER_CG_MYSHOP,
            length: (std::mem::size_of::<TPacketCGMyShop>() + std::mem::size_of::<TShopItemTable>() * stock_count) as u16,
            szSign,
            bCount: stock_count as u8,
        };

        let mut bytes = Vec::new();
        let packet_bytes: [u8; std::mem::size_of::<TPacketCGMyShop>()] = unsafe { std::mem::transmute(packet) };
        bytes.extend_from_slice(&packet_bytes);
        
        for item in items {
            let item_bytes: [u8; std::mem::size_of::<TShopItemTable>()] = unsafe { std::mem::transmute_copy(item) };
            bytes.extend_from_slice(&item_bytes);
        }

        send_packet(&bytes)
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_shop_buy_packet(bPos: u8) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGShop {
            header: HEADER_CG_SHOP,
            length: (std::mem::size_of::<TPacketCGShop>() + 2) as u16,
            subheader: SHOP_SUB_BUY,
        };

        let mut bytes = Vec::new();
        let packet_bytes: [u8; std::mem::size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
        bytes.extend_from_slice(&packet_bytes);
        
        let bCount = 1u8;
        bytes.push(bCount);
        bytes.push(bPos);

        send_packet(&bytes)
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_shop_sell_packet(bySlot: u8) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGShop {
            header: HEADER_CG_SHOP,
            length: (std::mem::size_of::<TPacketCGShop>() + 1) as u16,
            subheader: SHOP_SUB_SELL,
        };

        let mut bytes = Vec::new();
        let packet_bytes: [u8; std::mem::size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
        bytes.extend_from_slice(&packet_bytes);
        bytes.push(bySlot);

        send_packet(&bytes)
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_shop_sell_packet_new(bySlot: u8, byCount: u8) -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGShop {
            header: HEADER_CG_SHOP,
            length: (std::mem::size_of::<TPacketCGShop>() + 2) as u16,
            subheader: SHOP_SUB_SELL2,
        };

        let mut bytes = Vec::new();
        let packet_bytes: [u8; std::mem::size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
        bytes.extend_from_slice(&packet_bytes);
        bytes.push(bySlot);
        bytes.push(byCount);

        send_packet(&bytes)
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_shop_end_packet() -> bool {
    safe_ffi_call(false, || {
        let packet = TPacketCGShop {
            header: HEADER_CG_SHOP,
            length: std::mem::size_of::<TPacketCGShop>() as u16,
            subheader: SHOP_SUB_END,
        };

        let packet_bytes: [u8; std::mem::size_of::<TPacketCGShop>()] = unsafe { std::mem::transmute(packet) };
        send_packet(&packet_bytes)
    })
}
