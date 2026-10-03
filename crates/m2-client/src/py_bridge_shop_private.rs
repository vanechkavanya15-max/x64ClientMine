use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

pub const HEADER_CG_MYSHOP: u16 = 0x0802;
pub const SHOP_SIGN_MAX_LEN: usize = 32;

#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

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

pub struct PrivateShopState {
    pub is_private_shop: bool,
    pub is_main_player_private_shop: bool,
    pub stock: HashMap<TItemPos, TShopItemTable>,
}

pub static PRIVATE_SHOP_STATE: LazyLock<RwLock<PrivateShopState>> = LazyLock::new(|| {
    RwLock::new(PrivateShopState {
        is_private_shop: false,
        is_main_player_private_shop: false,
        stock: HashMap::new(),
    })
});

#[no_mangle]
pub extern "C" fn m2_py_shop_set_private_shop(is_private_shop: bool, is_main_player: bool) {
    if let Ok(mut state) = PRIVATE_SHOP_STATE.write() {
        state.is_private_shop = is_private_shop;
        state.is_main_player_private_shop = is_main_player;
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_is_private_shop() -> bool {
    if let Ok(state) = PRIVATE_SHOP_STATE.read() {
        state.is_private_shop
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_is_main_player_private_shop() -> bool {
    if let Ok(state) = PRIVATE_SHOP_STATE.read() {
        state.is_main_player_private_shop
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_clear_private_shop_stock() {
    if let Ok(mut state) = PRIVATE_SHOP_STATE.write() {
        state.stock.clear();
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_add_private_shop_item_stock(
    window_type: u8,
    cell: u16,
    display_pos: u8,
    price: u32,
) {
    let pos = TItemPos { window_type, cell };
    let item = TShopItemTable {
        vnum: 0,
        count: 0,
        pos,
        price,
        display_pos,
    };

    if let Ok(mut state) = PRIVATE_SHOP_STATE.write() {
        state.stock.insert(pos, item);
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_del_private_shop_item_stock(window_type: u8, cell: u16) {
    let pos = TItemPos { window_type, cell };
    if let Ok(mut state) = PRIVATE_SHOP_STATE.write() {
        state.stock.remove(&pos);
    }
}

#[no_mangle]
pub extern "C" fn m2_py_shop_get_private_shop_item_price(window_type: u8, cell: u16) -> u32 {
    let pos = TItemPos { window_type, cell };
    if let Ok(state) = PRIVATE_SHOP_STATE.read() {
        if let Some(item) = state.stock.get(&pos) {
            let price = { item.price }; // Using block to avoid unaligned reference error
            return price;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn m2_py_shop_build_private_shop(
    title_ptr: *const u8,
    title_len: usize,
    out_buf: *mut u8,
) -> usize {
    if out_buf.is_null() {
        return 0;
    }

    let mut stock = if let Ok(state) = PRIVATE_SHOP_STATE.read() {
        state.stock.values().cloned().collect::<Vec<TShopItemTable>>()
    } else {
        return 0;
    };

    stock.sort_by(|a, b| {
        let da = { a.display_pos };
        let db = { b.display_pos };
        da.cmp(&db)
    });

    let item_count = std::cmp::min(stock.len(), 39);

    let mut packet = TPacketCGMyShop {
        header: HEADER_CG_MYSHOP,
        length: (std::mem::size_of::<TPacketCGMyShop>() + std::mem::size_of::<TShopItemTable>() * item_count) as u16,
        sz_sign: [0; SHOP_SIGN_MAX_LEN + 1],
        b_count: item_count as u8,
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

        if item_count > 0 {
            std::ptr::copy_nonoverlapping(
                stock.as_ptr() as *const u8,
                out_buf.add(offset),
                std::mem::size_of::<TShopItemTable>() * item_count,
            );
            offset += std::mem::size_of::<TShopItemTable>() * item_count;
        }
    }

    offset
}
