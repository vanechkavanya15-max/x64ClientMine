// send_missing_cg_packets.rs - Implementacja brakujacych 23 pakietow sieciowych Client->Game (CG)
// Zgodnosc binarna 1:1 ze strukturami w Packet.h klienta C++.
// Komentarze w jezyku polskim bez polskich znakow diakrytycznych.

use m2_client_bridge::net::send_queue::enqueue_outgoing_packet;
use m2_client_bridge::ffi_guard::safe_ffi_call;
use std::ffi::CStr;
use std::os::raw::c_char;

// ============================================================================
// Stale opkodow CG z Packet.h (uint16_t header)
// ============================================================================
pub const HEADER_CG_CLIENT_VERSION: u16         = 0x000D;
pub const HEADER_CG_STATE_CHECKER: u16          = 0x000F;
pub const HEADER_CG_TEXT: u16                   = 0x0011;
pub const HEADER_CG_LOGIN_SECURE: u16           = 0x0103;
pub const HEADER_CG_EMPIRE: u16                 = 0x010A;
pub const HEADER_CG_SYNC_POSITION: u16          = 0x0303;
pub const HEADER_CG_ADD_FLY_TARGETING: u16      = 0x0405;
pub const HEADER_CG_GUILD_SYMBOL_UPLOAD: u16    = 0x0722;
pub const HEADER_CG_SYMBOL_CRC: u16             = 0x0723;
pub const HEADER_CG_MYSHOP: u16                 = 0x0802;
pub const HEADER_CG_SAFEBOX_ITEM_MOVE: u16      = 0x0822;
pub const HEADER_CG_MALL_CHECKOUT: u16          = 0x0840;
pub const HEADER_CG_SCRIPT_BUTTON: u16          = 0x0902;
pub const HEADER_CG_SCRIPT_SELECT_ITEM: u16     = 0x0903;
pub const HEADER_CG_QUEST_INPUT_STRING: u16     = 0x0904;
pub const HEADER_CG_QUEST_CONFIRM: u16          = 0x0905;
pub const HEADER_CG_QUEST_CANCEL: u16           = 0x0906;
pub const HEADER_CG_FISHING: u16                = 0x0B01;
pub const HEADER_CG_DUNGEON: u16                = 0x0B02;
pub const HEADER_CG_HACK: u16                   = 0x0B03;
pub const HEADER_CG_MARK_LOGIN: u16             = 0x0C01;
pub const HEADER_CG_MARK_CRCLIST: u16           = 0x0C02;
pub const HEADER_CG_MARK_UPLOAD: u16            = 0x0C03;
pub const HEADER_CG_MARK_IDXLIST: u16           = 0x0C04;

pub const QUEST_INPUT_STRING_MAX_NUM: usize     = 64;

// Typ pomocniczy dla pozycji przedmiotu TItemPos (window_type: u8, cell: u16)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TItemPos {
    pub window_type: u8,
    pub cell: u16,
}

// ============================================================================
// Struktury Pakietow CG (#[repr(C, packed)] zgodnie z Packet.h)
// ============================================================================

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGScriptButton {
    pub header: u16,
    pub length: u16,
    pub idx: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGScriptSelectItem {
    pub header: u16,
    pub length: u16,
    pub selection: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGQuestConfirm {
    pub header: u16,
    pub length: u16,
    pub answer: u8,
    pub request_pid: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGQuestCancel {
    pub header: u16,
    pub length: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGQuestInputString {
    pub header: u16,
    pub length: u16,
    pub sz_string: [u8; QUEST_INPUT_STRING_MAX_NUM + 1],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGSafeboxItemMove {
    pub header: u16,
    pub length: u16,
    pub b_source_pos: u8,
    pub item_pos: TItemPos,
    pub b_count: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGMallCheckout {
    pub header: u16,
    pub length: u16,
    pub b_mall_pos: u8,
    pub item_pos: TItemPos,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGEmpire {
    pub header: u16,
    pub length: u16,
    pub b_empire: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGFishing {
    pub header: u16,
    pub length: u16,
    pub dir: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGDungeon {
    pub header: u16,
    pub length: u16,
    pub sub_header: u16,
    pub param1: u32,
    pub param2: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGHack {
    pub header: u16,
    pub length: u16,
    pub sz_buf: [u8; 256],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGClientVersion {
    pub header: u16,
    pub length: u16,
    pub sz_version: [u8; 16],
    pub sz_file_name: [u8; 128],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGSyncPositionElement {
    pub dw_vid: u32,
    pub l_x: i32,
    pub l_y: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGSyncPosition {
    pub header: u16,
    pub length: u16,
    pub element: TPacketCGSyncPositionElement,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGAddFlyTargeting {
    pub header: u16,
    pub length: u16,
    pub dw_target_vid: u32,
    pub x: i32,
    pub y: i32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGMarkLogin {
    pub header: u16,
    pub length: u16,
    pub handle: u32,
    pub random_key: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGMarkCRCList {
    pub header: u16,
    pub length: u16,
    pub img_idx: u8,
    pub crclist: [u32; 8],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGMarkIDXList {
    pub header: u16,
    pub length: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGSymbolCRC {
    pub header: u16,
    pub length: u16,
    pub guild_id: u32,
    pub crc: u32,
    pub size: u32,
}

// ============================================================================
// Implementacje Tworzenia Pakietow i Serializacji
// ============================================================================

impl TPacketCGScriptButton {
    pub fn new(idx: u32) -> Self {
        Self {
            header: HEADER_CG_SCRIPT_BUTTON,
            length: std::mem::size_of::<Self>() as u16,
            idx,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = std::mem::size_of::<Self>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(self as *const Self as *const u8, buf.as_mut_ptr(), size);
        }
        buf
    }
}

impl TPacketCGQuestConfirm {
    pub fn new(answer: u8, request_pid: u32) -> Self {
        Self {
            header: HEADER_CG_QUEST_CONFIRM,
            length: std::mem::size_of::<Self>() as u16,
            answer,
            request_pid,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = std::mem::size_of::<Self>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(self as *const Self as *const u8, buf.as_mut_ptr(), size);
        }
        buf
    }
}

impl TPacketCGQuestInputString {
    pub fn new(input: &str) -> Self {
        let mut sz = [0u8; QUEST_INPUT_STRING_MAX_NUM + 1];
        let bytes = input.as_bytes();
        let copy_len = bytes.len().min(QUEST_INPUT_STRING_MAX_NUM);
        sz[..copy_len].copy_from_slice(&bytes[..copy_len]);

        Self {
            header: HEADER_CG_QUEST_INPUT_STRING,
            length: std::mem::size_of::<Self>() as u16,
            sz_string: sz,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = std::mem::size_of::<Self>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(self as *const Self as *const u8, buf.as_mut_ptr(), size);
        }
        buf
    }
}

impl TPacketCGSafeboxItemMove {
    pub fn new(source_pos: u8, dst_window: u8, dst_cell: u16, count: u8) -> Self {
        Self {
            header: HEADER_CG_SAFEBOX_ITEM_MOVE,
            length: std::mem::size_of::<Self>() as u16,
            b_source_pos: source_pos,
            item_pos: TItemPos { window_type: dst_window, cell: dst_cell },
            b_count: count,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = std::mem::size_of::<Self>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(self as *const Self as *const u8, buf.as_mut_ptr(), size);
        }
        buf
    }
}

impl TPacketCGMallCheckout {
    pub fn new(mall_pos: u8, dst_window: u8, dst_cell: u16) -> Self {
        Self {
            header: HEADER_CG_MALL_CHECKOUT,
            length: std::mem::size_of::<Self>() as u16,
            b_mall_pos: mall_pos,
            item_pos: TItemPos { window_type: dst_window, cell: dst_cell },
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = std::mem::size_of::<Self>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(self as *const Self as *const u8, buf.as_mut_ptr(), size);
        }
        buf
    }
}

impl TPacketCGEmpire {
    pub fn new(empire: u8) -> Self {
        Self {
            header: HEADER_CG_EMPIRE,
            length: std::mem::size_of::<Self>() as u16,
            b_empire: empire,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = std::mem::size_of::<Self>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(self as *const Self as *const u8, buf.as_mut_ptr(), size);
        }
        buf
    }
}

impl TPacketCGFishing {
    pub fn new(dir: u8) -> Self {
        Self {
            header: HEADER_CG_FISHING,
            length: std::mem::size_of::<Self>() as u16,
            dir,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = std::mem::size_of::<Self>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(self as *const Self as *const u8, buf.as_mut_ptr(), size);
        }
        buf
    }
}

impl TPacketCGDungeon {
    pub fn new(sub_header: u16, param1: u32, param2: u32) -> Self {
        Self {
            header: HEADER_CG_DUNGEON,
            length: std::mem::size_of::<Self>() as u16,
            sub_header,
            param1,
            param2,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = std::mem::size_of::<Self>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(self as *const Self as *const u8, buf.as_mut_ptr(), size);
        }
        buf
    }
}

impl TPacketCGSyncPosition {
    pub fn new(vid: u32, x: i32, y: i32) -> Self {
        Self {
            header: HEADER_CG_SYNC_POSITION,
            length: std::mem::size_of::<Self>() as u16,
            element: TPacketCGSyncPositionElement {
                dw_vid: vid,
                l_x: x,
                l_y: y,
            },
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = std::mem::size_of::<Self>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(self as *const Self as *const u8, buf.as_mut_ptr(), size);
        }
        buf
    }
}

impl TPacketCGSymbolCRC {
    pub fn new(guild_id: u32, crc: u32, size: u32) -> Self {
        Self {
            header: HEADER_CG_SYMBOL_CRC,
            length: std::mem::size_of::<Self>() as u16,
            guild_id,
            crc,
            size,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let size = std::mem::size_of::<Self>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(self as *const Self as *const u8, buf.as_mut_ptr(), size);
        }
        buf
    }
}

// ============================================================================
// Eksporty C-ABI (FFI) do Pythona i Glownej Petli Gry
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_net_send_quest_button_packet(idx: u32) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGScriptButton::new(idx);
        enqueue_outgoing_packet(pkt.to_bytes());
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_quest_select_item_packet(selection: u32) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGScriptSelectItem {
            header: HEADER_CG_SCRIPT_SELECT_ITEM,
            length: std::mem::size_of::<TPacketCGScriptSelectItem>() as u16,
            selection,
        };
        let size = std::mem::size_of::<TPacketCGScriptSelectItem>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(&pkt as *const _ as *const u8, buf.as_mut_ptr(), size);
        }
        enqueue_outgoing_packet(buf);
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_quest_confirm_packet_v2(answer: u8, request_pid: u32) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGQuestConfirm::new(answer, request_pid);
        enqueue_outgoing_packet(pkt.to_bytes());
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_quest_cancel_packet() -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGQuestCancel {
            header: HEADER_CG_QUEST_CANCEL,
            length: std::mem::size_of::<TPacketCGQuestCancel>() as u16,
        };
        let size = std::mem::size_of::<TPacketCGQuestCancel>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(&pkt as *const _ as *const u8, buf.as_mut_ptr(), size);
        }
        enqueue_outgoing_packet(buf);
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_quest_input_string_packet_v2(input: *const c_char) -> bool {
    safe_ffi_call(false, || {
        if input.is_null() {
            return false;
        }
        let c_str = unsafe { CStr::from_ptr(input) };
        let s = c_str.to_string_lossy();
        let pkt = TPacketCGQuestInputString::new(&s);
        enqueue_outgoing_packet(pkt.to_bytes());
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_safebox_item_move_packet(source_pos: u8, dst_window: u8, dst_cell: u16, count: u8) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGSafeboxItemMove::new(source_pos, dst_window, dst_cell, count);
        enqueue_outgoing_packet(pkt.to_bytes());
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_mall_checkout_packet(mall_pos: u8, dst_window: u8, dst_cell: u16) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGMallCheckout::new(mall_pos, dst_window, dst_cell);
        enqueue_outgoing_packet(pkt.to_bytes());
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_empire_select_packet(empire: u8) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGEmpire::new(empire);
        enqueue_outgoing_packet(pkt.to_bytes());
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_fishing_cast_packet(dir: u8) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGFishing::new(dir);
        enqueue_outgoing_packet(pkt.to_bytes());
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_dungeon_command_packet(sub_header: u16, p1: u32, p2: u32) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGDungeon::new(sub_header, p1, p2);
        enqueue_outgoing_packet(pkt.to_bytes());
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_sync_position_direct(vid: u32, x: i32, y: i32) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGSyncPosition::new(vid, x, y);
        enqueue_outgoing_packet(pkt.to_bytes());
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_guild_symbol_crc_packet(guild_id: u32, crc: u32, size: u32) -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGSymbolCRC::new(guild_id, crc, size);
        enqueue_outgoing_packet(pkt.to_bytes());
        true
    })
}

#[no_mangle]
pub extern "C" fn m2_net_send_mark_idx_list_packet() -> bool {
    safe_ffi_call(false, || {
        let pkt = TPacketCGMarkIDXList {
            header: HEADER_CG_MARK_IDXLIST,
            length: std::mem::size_of::<TPacketCGMarkIDXList>() as u16,
        };
        let size = std::mem::size_of::<TPacketCGMarkIDXList>();
        let mut buf = vec![0u8; size];
        unsafe {
            std::ptr::copy_nonoverlapping(&pkt as *const _ as *const u8, buf.as_mut_ptr(), size);
        }
        enqueue_outgoing_packet(buf);
        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_button_packet_size_and_header() {
        let pkt = TPacketCGScriptButton::new(42);
        let header = pkt.header;
        let length = pkt.length;
        assert_eq!(header, 0x0902);
        assert_eq!(length, 8); // 2 + 2 + 4
        let bytes = pkt.to_bytes();
        assert_eq!(bytes.len(), 8);
        assert_eq!(u16::from_le_bytes([bytes[0], bytes[1]]), 0x0902);
        assert_eq!(u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]), 42);
    }

    #[test]
    fn test_quest_confirm_packet() {
        let pkt = TPacketCGQuestConfirm::new(1, 100500);
        let header = pkt.header;
        let length = pkt.length;
        assert_eq!(header, 0x0905);
        assert_eq!(length, 9); // 2 + 2 + 1 + 4
        let bytes = pkt.to_bytes();
        assert_eq!(bytes.len(), 9);
        assert_eq!(bytes[4], 1);
        assert_eq!(u32::from_le_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]), 100500);
    }

    #[test]
    fn test_quest_input_string_packet() {
        let pkt = TPacketCGQuestInputString::new("Hello Metin2 Pure Rust");
        let header = pkt.header;
        let length = pkt.length;
        assert_eq!(header, 0x0904);
        assert_eq!(length, 69); // 2 + 2 + 65
        let bytes = pkt.to_bytes();
        assert_eq!(bytes.len(), 69);
        let str_bytes = &bytes[4..4 + 22];
        assert_eq!(std::str::from_utf8(str_bytes).unwrap(), "Hello Metin2 Pure Rust");
    }

    #[test]
    fn test_safebox_item_move_packet() {
        let pkt = TPacketCGSafeboxItemMove::new(5, 1, 12, 10);
        let header = pkt.header;
        assert_eq!(header, 0x0822);
        let bytes = pkt.to_bytes();
        assert_eq!(bytes[4], 5);
        assert_eq!(bytes[5], 1); // window_type
        assert_eq!(u16::from_le_bytes([bytes[6], bytes[7]]), 12); // cell
        assert_eq!(bytes[8], 10); // count
    }

    #[test]
    fn test_sync_position_packet() {
        let pkt = TPacketCGSyncPosition::new(999, 123450, -543210);
        let header = pkt.header;
        let length = pkt.length;
        assert_eq!(header, 0x0303);
        assert_eq!(length, 16); // 2 + 2 + 4 + 4 + 4
        let bytes = pkt.to_bytes();
        assert_eq!(bytes.len(), 16);
        assert_eq!(u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]), 999);
    }
}
