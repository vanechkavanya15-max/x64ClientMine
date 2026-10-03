use std::mem;
use std::sync::{RwLock, OnceLock};

pub const ITEM_SOCKET_SLOT_MAX_NUM: usize = 3;
pub const ITEM_ATTRIBUTE_SLOT_MAX_NUM: usize = 7;
pub const SHOP_HOST_ITEM_MAX_NUM: usize = 40;
pub const SHOP_TAB_NAME_MAX: usize = 32;
pub const REFINE_MATERIAL_MAX_NUM: usize = 5;
pub const EXCHANGE_ITEM_MAX_NUM: usize = 12;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TPlayerItemAttribute {
    pub b_type: u8,
    pub s_value: i16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCItemSet {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
    pub vnum: u32,
    pub count: u8,
    pub flags: u32,
    pub anti_flags: u32,
    pub highlight: u8,
    pub al_sockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub a_attr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCItemDel {
    pub header: u16,
    pub length: u16,
    pub pos: TItemPos,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCItemUpdate {
    pub header: u16,
    pub length: u16,
    pub cell: TItemPos,
    pub count: u8,
    pub al_sockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub a_attr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCItemGroundAdd {
    pub header: u16,
    pub length: u16,
    pub l_x: i32,
    pub l_y: i32,
    pub l_z: i32,
    pub dw_vid: u32,
    pub dw_vnum: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCItemGroundDel {
    pub header: u16,
    pub length: u16,
    pub vid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct packet_shop_item {
    pub vnum: u32,
    pub price: u32,
    pub count: u8,
    pub display_pos: u8,
    pub al_sockets: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub a_attr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

impl Default for packet_shop_item {
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

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCShop {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCShopStart {
    pub items: [packet_shop_item; SHOP_HOST_ITEM_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TSubPacketShopTab {
    pub name: [u8; SHOP_TAB_NAME_MAX],
    pub coin_type: u8,
    pub items: [packet_shop_item; SHOP_HOST_ITEM_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCShopStartEx {
    pub owner_vid: u32,
    pub shop_tab_count: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCShopUpdateItem {
    pub pos: u8,
    pub item: packet_shop_item,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCShopUpdatePrice {
    pub i_elk_amount: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TMaterial {
    pub vnum: u32,
    pub count: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TRefineTable {
    pub src_vnum: u32,
    pub result_vnum: u32,
    pub material_count: u8,
    pub cost: i32,
    pub prob: i32,
    pub materials: [TMaterial; REFINE_MATERIAL_MAX_NUM],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCRefineInformation {
    pub header: u16,
    pub length: u16,
    pub type_: u8,
    pub pos: u8,
    pub refine_table: TRefineTable,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketGCRefineInformationNew {
    pub header: u16,
    pub length: u16,
    pub type_: u8,
    pub pos: u8,
    pub refine_table: TRefineTable,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCExchange {
    pub header: u16,
    pub length: u16,
    pub subheader: u8,
    pub is_me: u8,
    pub arg1: u32,
    pub arg2: TItemPos,
    pub arg3: u32,
    pub al_values: [i32; ITEM_SOCKET_SLOT_MAX_NUM],
    pub a_attr: [TPlayerItemAttribute; ITEM_ATTRIBUTE_SLOT_MAX_NUM],
}

use std::collections::HashMap;

// Item Ground State
pub struct ItemGroundData {
    pub vnum: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct ItemGroundState {
    pub items: HashMap<u32, ItemGroundData>,
}

pub fn get_item_ground_state() -> &'static RwLock<ItemGroundState> {
    static STATE: OnceLock<RwLock<ItemGroundState>> = OnceLock::new();
    STATE.get_or_init(|| RwLock::new(ItemGroundState { items: HashMap::new() }))
}

// Shop State
#[derive(Clone)]
pub struct ShopTab {
    pub name: [u8; SHOP_TAB_NAME_MAX],
    pub coin_type: u8,
    pub items: [packet_shop_item; SHOP_HOST_ITEM_MAX_NUM],
}

pub struct ShopState {
    pub owner_vid: u32,
    pub tabs: Vec<ShopTab>,
    pub items: [packet_shop_item; SHOP_HOST_ITEM_MAX_NUM],
}

pub fn get_shop_state() -> &'static RwLock<ShopState> {
    static STATE: OnceLock<RwLock<ShopState>> = OnceLock::new();
    STATE.get_or_init(|| RwLock::new(ShopState {
        owner_vid: 0,
        tabs: Vec::new(),
        items: [packet_shop_item::default(); SHOP_HOST_ITEM_MAX_NUM],
    }))
}

// Refine State
pub struct RefineState {
    pub info: Option<TRefineTable>,
    pub type_: u8,
    pub pos: u8,
}

pub fn get_refine_state() -> &'static RwLock<RefineState> {
    static STATE: OnceLock<RwLock<RefineState>> = OnceLock::new();
    STATE.get_or_init(|| RwLock::new(RefineState { info: None, type_: 0, pos: 0 }))
}

// Exchange State Data
#[derive(Clone)]
pub struct ExchangeStateData {
    pub active: bool,
}

pub fn get_exchange_state() -> &'static RwLock<ExchangeStateData> {
    static STATE: OnceLock<RwLock<ExchangeStateData>> = OnceLock::new();
    STATE.get_or_init(|| RwLock::new(ExchangeStateData { active: false }))
}

// FFI Functions

#[no_mangle]
pub extern "C" fn m2_net_recv_item_set(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len < mem::size_of::<TPacketGCItemSet>() {
        return false;
    }
    let _packet = unsafe { &*(buf as *const TPacketGCItemSet) };
    true
}

#[no_mangle]
pub extern "C" fn m2_net_recv_item_del(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len < mem::size_of::<TPacketGCItemDel>() {
        return false;
    }
    true
}

#[no_mangle]
pub extern "C" fn m2_net_recv_item_update(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len < mem::size_of::<TPacketGCItemUpdate>() {
        return false;
    }
    true
}

#[no_mangle]
pub extern "C" fn m2_net_recv_item_ground_add(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len < mem::size_of::<TPacketGCItemGroundAdd>() {
        return false;
    }
    let packet = unsafe { &*(buf as *const TPacketGCItemGroundAdd) };
    if let Ok(mut state) = get_item_ground_state().write() {
        let vid = { packet.dw_vid };
        let vnum = { packet.dw_vnum };
        let x = { packet.l_x };
        let y = { packet.l_y };
        let z = { packet.l_z };
        state.items.insert(vid, ItemGroundData { vnum, x, y, z });
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_recv_item_ground_del(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len < mem::size_of::<TPacketGCItemGroundDel>() {
        return false;
    }
    let packet = unsafe { &*(buf as *const TPacketGCItemGroundDel) };
    if let Ok(mut state) = get_item_ground_state().write() {
        let vid = { packet.vid };
        state.items.remove(&vid);
        true
    } else {
        false
    }
}

pub extern "C" fn m2_net_recv_shop(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len < mem::size_of::<TPacketGCShop>() {
        return false;
    }
    let packet = unsafe { &*(buf as *const TPacketGCShop) };
    let subheader = { packet.subheader };
    
    let payload = unsafe { buf.add(mem::size_of::<TPacketGCShop>()) };
    let payload_len = len - mem::size_of::<TPacketGCShop>();

    if let Ok(mut state) = get_shop_state().write() {
        match subheader {
            0 => { // START
                if payload_len < mem::size_of::<TPacketGCShopStart>() + 4 {
                    return false;
                }
                let owner_vid = unsafe { std::ptr::read_unaligned(payload as *const u32) };
                let start_packet = unsafe { &*(payload.add(4) as *const TPacketGCShopStart) };
                state.owner_vid = owner_vid;
                state.items.copy_from_slice(&start_packet.items);
                state.tabs.clear();
            }
            10 => { // START_EX
                if payload_len < mem::size_of::<TPacketGCShopStartEx>() {
                    return false;
                }
                let start_ex = unsafe { &*(payload as *const TPacketGCShopStartEx) };
                let tab_count = { start_ex.shop_tab_count };
                state.owner_vid = { start_ex.owner_vid };
                state.tabs.clear();
                
                let mut read_pos = mem::size_of::<TPacketGCShopStartEx>();
                for _ in 0..tab_count {
                    if payload_len < read_pos + mem::size_of::<TSubPacketShopTab>() {
                        break;
                    }
                    let tab = unsafe { &*(payload.add(read_pos) as *const TSubPacketShopTab) };
                    state.tabs.push(ShopTab { name: tab.name, coin_type: tab.coin_type, items: tab.items });
                    read_pos += mem::size_of::<TSubPacketShopTab>();
                }
            }
            2 => { // UPDATE_ITEM
                if payload_len < mem::size_of::<TPacketGCShopUpdateItem>() {
                    return false;
                }
                let update = unsafe { &*(payload as *const TPacketGCShopUpdateItem) };
                let pos = { update.pos } as usize;
                if pos < SHOP_HOST_ITEM_MAX_NUM {
                    state.items[pos] = update.item;
                }
            }
            _ => {}
        }
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn m2_net_recv_exchange_v2(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len < mem::size_of::<TPacketGCExchange>() {
        return false;
    }
    let _packet = unsafe { &*(buf as *const TPacketGCExchange) };
    true
}

#[no_mangle]
pub extern "C" fn m2_net_recv_refine_info(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len < mem::size_of::<TPacketGCRefineInformation>() {
        return false;
    }
    let packet = unsafe { &*(buf as *const TPacketGCRefineInformation) };
    let refine_table = { packet.refine_table };
    let pos = { packet.pos };
    let type_ = { packet.type_ };
    if let Ok(mut state) = get_refine_state().write() {
        state.info = Some(refine_table);
        state.type_ = type_;
        state.pos = pos;
    }
    let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
    if game_wnd != 0 {
        crate::py_event_dispatcher::call_python_object_method4_i32(
            game_wnd,
            "OpenRefineDialog",
            pos as i32,
            refine_table.result_vnum as i32,
            refine_table.cost,
            refine_table.prob,
        );
        for i in 0..(refine_table.material_count as usize).min(REFINE_MATERIAL_MAX_NUM) {
            crate::py_event_dispatcher::call_python_object_method2_i32(
                game_wnd,
                "AppendMaterialToRefineDialog",
                refine_table.materials[i].vnum as i32,
                refine_table.materials[i].count,
            );
        }
    }
    true
}

#[no_mangle]
pub extern "C" fn m2_net_recv_refine_info_new(buf: *const u8, len: usize) -> bool {
    if buf.is_null() || len < mem::size_of::<TPacketGCRefineInformationNew>() {
        return false;
    }
    let packet = unsafe { &*(buf as *const TPacketGCRefineInformationNew) };
    let refine_table = { packet.refine_table };
    let pos = { packet.pos };
    let type_ = { packet.type_ };
    if let Ok(mut state) = get_refine_state().write() {
        state.info = Some(refine_table);
        state.type_ = type_;
        state.pos = pos;
    }
    let game_wnd = crate::py_bridge_net::get_phase_window(crate::py_bridge_net::PHASE_WINDOW_GAME);
    if game_wnd != 0 {
        crate::py_event_dispatcher::call_python_object_method5_i32(
            game_wnd,
            "OpenRefineDialog",
            pos as i32,
            refine_table.result_vnum as i32,
            refine_table.cost,
            refine_table.prob,
            type_ as i32,
        );
        for i in 0..(refine_table.material_count as usize).min(REFINE_MATERIAL_MAX_NUM) {
            crate::py_event_dispatcher::call_python_object_method2_i32(
                game_wnd,
                "AppendMaterialToRefineDialog",
                refine_table.materials[i].vnum as i32,
                refine_table.materials[i].count,
            );
        }
    }
    true
}
