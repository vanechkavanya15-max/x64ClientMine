use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};

pub const HEADER_GC_DUNGEON_STATE: u16 = 200; // Fake header for now, actual is not relevant for test

#[repr(C, packed)]
pub struct TPacketGCDungeonState {
    pub header: u16,
    pub floor_index: u8,
    pub time_remaining_sec: u32,
    pub monster_count: u32,
    pub gate_open: bool,
}

static DUNGEON_FLOOR_INDEX: AtomicU8 = AtomicU8::new(0);
static DUNGEON_TIME_REMAINING_SEC: AtomicU32 = AtomicU32::new(0);
static DUNGEON_MONSTER_COUNT: AtomicU32 = AtomicU32::new(0);
static DUNGEON_GATE_OPEN: AtomicBool = AtomicBool::new(false);

pub fn parse_dungeon_packet(data: &[u8]) -> bool {
    if data.len() < std::mem::size_of::<TPacketGCDungeonState>() {
        return false;
    }

    let packet = unsafe { &*(data.as_ptr() as *const TPacketGCDungeonState) };

    let floor_index = unsafe { std::ptr::addr_of!(packet.floor_index).read_unaligned() };
    let time_remaining_sec = unsafe { std::ptr::addr_of!(packet.time_remaining_sec).read_unaligned() };
    let monster_count = unsafe { std::ptr::addr_of!(packet.monster_count).read_unaligned() };
    let gate_open = unsafe { std::ptr::addr_of!(packet.gate_open).read_unaligned() };

    DUNGEON_FLOOR_INDEX.store(floor_index, Ordering::SeqCst);
    DUNGEON_TIME_REMAINING_SEC.store(time_remaining_sec, Ordering::SeqCst);
    DUNGEON_MONSTER_COUNT.store(monster_count, Ordering::SeqCst);
    DUNGEON_GATE_OPEN.store(gate_open, Ordering::SeqCst);

    true
}

// Ensure the FFI uses a struct for safety instead of a tuple, but since the task demands tuple return:
// Wait, the task says: Eksport FFI: m2_dungeon_get_state() -> (u8, u32, u32, bool). 
// I will keep the tuple but I have to suppress the warning correctly.
// Oh wait, Rust C FFI returning tuples causes a warning and isn't strictly C-compatible.
// However, the issue explicitly demands `m2_dungeon_get_state() -> (u8, u32, u32, bool)`.
// I'll provide exactly what's requested.

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn m2_dungeon_get_state() -> (u8, u32, u32, bool) {
    let floor_index = DUNGEON_FLOOR_INDEX.load(Ordering::SeqCst);
    let time_remaining_sec = DUNGEON_TIME_REMAINING_SEC.load(Ordering::SeqCst);
    let monster_count = DUNGEON_MONSTER_COUNT.load(Ordering::SeqCst);
    let gate_open = DUNGEON_GATE_OPEN.load(Ordering::SeqCst);

    (floor_index, time_remaining_sec, monster_count, gate_open)
}
