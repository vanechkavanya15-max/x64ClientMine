use std::sync::RwLock;

pub const QUICKSLOT_MAX_NUM: usize = 36;
pub const SLOT_TYPE_NONE: u8 = 0;
pub const SLOT_TYPE_INVENTORY: u8 = 1;
pub const SLOT_TYPE_SKILL: u8 = 2;
pub const SLOT_TYPE_EMOTION: u8 = 3;
// Following the C++ enum ESlotType from GameType.h
// SLOT_TYPE_SHOP = 4, etc.
// But we mostly care about None(0), Item(1), Skill(2), Emotion(3).

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct TQuickSlot {
    pub slot_type: u8,
    pub slot_position: u32,
}

#[repr(C, packed)]
pub struct TPacketGCQuickSlotAdd {
    pub header: u16,
    pub length: u16,
    pub pos: u8,
    pub slot: TQuickSlot,
}

#[repr(C, packed)]
pub struct TPacketGCQuickSlotDel {
    pub header: u16,
    pub length: u16,
    pub pos: u8,
}

#[repr(C, packed)]
pub struct TPacketGCQuickSlotSwap {
    pub header: u16,
    pub length: u16,
    pub pos: u8,
    pub change_pos: u8,
}

use std::sync::OnceLock;

// Global state using RwLock to safely store the array.
static QUICKSLOT_REGISTRY: OnceLock<RwLock<[TQuickSlot; QUICKSLOT_MAX_NUM]>> = OnceLock::new();

fn get_quickslot_registry() -> &'static RwLock<[TQuickSlot; QUICKSLOT_MAX_NUM]> {
    QUICKSLOT_REGISTRY.get_or_init(|| RwLock::new([TQuickSlot::default(); QUICKSLOT_MAX_NUM]))
}

/// Parses the TPacketGCQuickSlotAdd and updates the registry
pub fn process_quickslot_add(packet_ptr: *const u8) -> bool {
    let packet: TPacketGCQuickSlotAdd = unsafe {
        std::ptr::read_unaligned(packet_ptr as *const TPacketGCQuickSlotAdd)
    };

    if packet.pos as usize >= QUICKSLOT_MAX_NUM {
        return false;
    }

    if let Ok(mut registry) = get_quickslot_registry().write() {
        registry[packet.pos as usize] = packet.slot;
        return true;
    }

    false
}

/// Parses the TPacketGCQuickSlotDel and resets the slot in the registry
pub fn process_quickslot_del(packet_ptr: *const u8) -> bool {
    let packet: TPacketGCQuickSlotDel = unsafe {
        std::ptr::read_unaligned(packet_ptr as *const TPacketGCQuickSlotDel)
    };

    if packet.pos as usize >= QUICKSLOT_MAX_NUM {
        return false;
    }

    if let Ok(mut registry) = get_quickslot_registry().write() {
        registry[packet.pos as usize] = TQuickSlot {
            slot_type: SLOT_TYPE_NONE,
            slot_position: 0,
        };
        return true;
    }

    false
}

/// Parses the TPacketGCQuickSlotSwap and swaps the two slots
pub fn process_quickslot_swap(packet_ptr: *const u8) -> bool {
    let packet: TPacketGCQuickSlotSwap = unsafe {
        std::ptr::read_unaligned(packet_ptr as *const TPacketGCQuickSlotSwap)
    };

    if packet.pos as usize >= QUICKSLOT_MAX_NUM || packet.change_pos as usize >= QUICKSLOT_MAX_NUM {
        return false;
    }

    if let Ok(mut registry) = get_quickslot_registry().write() {
        registry.swap(packet.pos as usize, packet.change_pos as usize);
        return true;
    }

    false
}

/// Retrieves the specified quickslot via FFI
/// Returns (type: u8, position: u32).
/// If out of bounds or read fails, returns (0, 0).
#[repr(C)]
pub struct QuickSlotResult {
    pub slot_type: u8,
    pub slot_position: u32,
}

#[no_mangle]
pub extern "C" fn m2_quickslot_get_v2(pos: u8) -> QuickSlotResult {
    if pos as usize >= QUICKSLOT_MAX_NUM {
        return QuickSlotResult { slot_type: SLOT_TYPE_NONE, slot_position: 0 };
    }

    if let Ok(registry) = get_quickslot_registry().read() {
        let slot = registry[pos as usize];
        return QuickSlotResult { slot_type: slot.slot_type, slot_position: slot.slot_position as u32 };
    }

    QuickSlotResult { slot_type: SLOT_TYPE_NONE, slot_position: 0 }
}

/// Utility function to clear the registry (mainly for testing)
pub fn clear_quickslot_registry() {
    if let Ok(mut registry) = get_quickslot_registry().write() {
        for slot in registry.iter_mut() {
            *slot = TQuickSlot {
                slot_type: SLOT_TYPE_NONE,
                slot_position: 0,
            };
        }
    }
}
