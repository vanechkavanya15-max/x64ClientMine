use std::mem::size_of;
use std::sync::{RwLock, LazyLock};

pub const SHOP_SUBHEADER_GC_START: u8 = 0;
pub const SHOP_SUBHEADER_GC_END: u8 = 1;
pub const SHOP_SUBHEADER_GC_UPDATE_ITEM: u8 = 2;
pub const SHOP_SUBHEADER_GC_UPDATE_PRICE: u8 = 3;
pub const SHOP_SUBHEADER_GC_NOT_ENOUGH_MONEY: u8 = 5;
pub const SHOP_SUBHEADER_GC_SOLDOUT: u8 = 6;
pub const SHOP_SUBHEADER_GC_INVENTORY_FULL: u8 = 7;
pub const SHOP_SUBHEADER_GC_INVALID_POS: u8 = 8;
pub const SHOP_SUBHEADER_GC_START_EX: u8 = 10;
pub const SHOP_SUBHEADER_GC_NOT_ENOUGH_MONEY_EX: u8 = 11;

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;
pub const SHOP_HOST_ITEM_MAX_NUM: usize = 40;

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TPlayerItemAttribute {
    pub b_type: u8,
    pub s_value: i16,
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TShopItemData {
    pub vnum: u32,
    pub price: u32,
    pub count: u8,
    pub display_pos: u8,
    pub al_sockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub a_attr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCShop {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCShopStart {
    pub items: [TShopItemData; SHOP_HOST_ITEM_MAX_NUM],
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCShopUpdateItem {
    pub pos: u8,
    pub item: TShopItemData,
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCShopUpdatePrice {
    pub i_elk_amount: i32,
}

impl Default for TPlayerItemAttribute {
    fn default() -> Self {
        Self { b_type: 0, s_value: 0 }
    }
}

impl Default for TShopItemData {
    fn default() -> Self {
        Self {
            vnum: 0,
            price: 0,
            count: 0,
            display_pos: 0,
            al_sockets: [0; ITEM_SOCKET_SLOT_MAX_NUM],
            a_attr: [TPlayerItemAttribute::default(); ITEM_ATTRIBUTE_SLOT_MAX_NUM],
        }
    }
}

pub struct PythonShop {
    pub items: [TShopItemData; SHOP_HOST_ITEM_MAX_NUM],
    pub is_open: bool,
    pub vid: u32,
    pub error_msg: Option<String>,
    pub selling_price: i32,
}

impl PythonShop {
    pub fn new() -> Self {
        Self {
            items: [TShopItemData::default(); SHOP_HOST_ITEM_MAX_NUM],
            is_open: false,
            vid: 0,
            error_msg: None,
            selling_price: 0,
        }
    }

    pub fn clear(&mut self) {
        self.items = [TShopItemData::default(); SHOP_HOST_ITEM_MAX_NUM];
        self.is_open = false;
        self.vid = 0;
        self.error_msg = None;
        self.selling_price = 0;
    }
    
    pub fn can_afford(&self, pos: usize, gold: u64) -> bool {
        if pos >= SHOP_HOST_ITEM_MAX_NUM {
            return false;
        }
        gold >= (self.items[pos].price as u64)
    }
}

pub static SHOP_INSTANCE: LazyLock<RwLock<PythonShop>> = LazyLock::new(|| RwLock::new(PythonShop {
    items: [TShopItemData {
        vnum: 0,
        price: 0,
        count: 0,
        display_pos: 0,
        al_sockets: [0; ITEM_SOCKET_SLOT_MAX_NUM],
        a_attr: [TPlayerItemAttribute { b_type: 0, s_value: 0 }; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
    }; SHOP_HOST_ITEM_MAX_NUM],
    is_open: false,
    vid: 0,
    error_msg: None,
    selling_price: 0,
}));

#[no_mangle]
pub extern "C" fn m2_net_recv_shop(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len == 0 {
        return false;
    }
    let slice = unsafe { std::slice::from_raw_parts(buf, len) };
    
    if slice.len() < size_of::<TPacketGCShop>() {
        return false;
    }

    let packet_shop = unsafe { std::ptr::read_unaligned(slice.as_ptr() as *const TPacketGCShop) };

    if (packet_shop.length as usize) > slice.len() {
        return false; 
    }
    
    let payload_len = packet_shop.length as usize - size_of::<TPacketGCShop>();
    let payload = &slice[size_of::<TPacketGCShop>()..size_of::<TPacketGCShop>()+payload_len];

    let mut shop = match SHOP_INSTANCE.write() {
        Ok(guard) => guard,
        Err(_) => return false,
    };

    match packet_shop.subheader {
        SHOP_SUBHEADER_GC_START => {
           if payload.len() < 4 + size_of::<TPacketGCShopStart>() {
               return false;
           }
           let vid = unsafe { std::ptr::read_unaligned(payload.as_ptr() as *const u32) };
           let p_shop_start = unsafe { std::ptr::read_unaligned((payload.as_ptr().add(4)) as *const TPacketGCShopStart) };
           
           shop.clear();
           shop.vid = vid;
           shop.is_open = true;
           
           for i in 0..SHOP_HOST_ITEM_MAX_NUM {
               shop.items[i] = p_shop_start.items[i];
           }
           true
        }
        SHOP_SUBHEADER_GC_END => {
            shop.is_open = false;
            true
        }
        SHOP_SUBHEADER_GC_UPDATE_ITEM => {
            if payload.len() < size_of::<TPacketGCShopUpdateItem>() {
                return false;
            }
            let p_shop_update_item = unsafe { std::ptr::read_unaligned(payload.as_ptr() as *const TPacketGCShopUpdateItem) };
            
            if p_shop_update_item.pos as usize >= SHOP_HOST_ITEM_MAX_NUM {
                return false;
            }
            shop.items[p_shop_update_item.pos as usize] = p_shop_update_item.item;
            true
        }
        SHOP_SUBHEADER_GC_UPDATE_PRICE => {
            if payload.len() < size_of::<TPacketGCShopUpdatePrice>() {
                return false;
            }
            let p = unsafe { std::ptr::read_unaligned(payload.as_ptr() as *const TPacketGCShopUpdatePrice) };
            shop.selling_price = p.i_elk_amount;
            true
        }
        SHOP_SUBHEADER_GC_NOT_ENOUGH_MONEY => {
            shop.error_msg = Some("NOT_ENOUGH_MONEY".to_string());
            true
        }
        SHOP_SUBHEADER_GC_NOT_ENOUGH_MONEY_EX => {
            shop.error_msg = Some("NOT_ENOUGH_MONEY_EX".to_string());
            true
        }
        SHOP_SUBHEADER_GC_SOLDOUT => {
            shop.error_msg = Some("SOLDOUT".to_string());
            true
        }
        SHOP_SUBHEADER_GC_INVENTORY_FULL => {
            shop.error_msg = Some("INVENTORY_FULL".to_string());
            true
        }
        SHOP_SUBHEADER_GC_INVALID_POS => {
            shop.error_msg = Some("INVALID_POS".to_string());
            true
        }
        SHOP_SUBHEADER_GC_START_EX => {
            true
        }
        _ => {
            true // unknown but handled
        }
    }
}
