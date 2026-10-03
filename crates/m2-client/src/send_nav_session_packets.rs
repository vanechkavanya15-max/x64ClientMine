use std::mem::size_of;
use std::slice;

// Opcodes specified by requirements
pub const HEADER_CG_WARP: u16 = 0x0307;
pub const HEADER_CG_LOGOUT: u16 = 0x0009;
pub const HEADER_CG_EXIT_GAME: u16 = 0x000A;
pub const HEADER_CG_CHARACTER_POSITION: u16 = 0x0308;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGWarp {
    pub header: u16,
    pub length: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGLogout {
    pub header: u16,
    pub length: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGExitGame {
    pub header: u16,
    pub length: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TPacketCGPosition {
    pub header: u16,
    pub length: u16,
    pub position: u8,
}

/// Builds the CG_WARP packet
pub fn build_warp_packet() -> Vec<u8> {
    let packet = TPacketCGWarp {
        header: HEADER_CG_WARP,
        length: size_of::<TPacketCGWarp>() as u16,
    };
    struct_to_bytes(&packet)
}

/// Builds the CG_LOGOUT packet
pub fn build_logout_packet() -> Vec<u8> {
    let packet = TPacketCGLogout {
        header: HEADER_CG_LOGOUT,
        length: size_of::<TPacketCGLogout>() as u16,
    };
    struct_to_bytes(&packet)
}

/// Builds the CG_EXIT_GAME packet
pub fn build_exit_game_packet() -> Vec<u8> {
    let packet = TPacketCGExitGame {
        header: HEADER_CG_EXIT_GAME,
        length: size_of::<TPacketCGExitGame>() as u16,
    };
    struct_to_bytes(&packet)
}

/// Builds the CG_CHARACTER_POSITION packet
pub fn build_character_position_packet(position: u8) -> Vec<u8> {
    let packet = TPacketCGPosition {
        header: HEADER_CG_CHARACTER_POSITION,
        length: size_of::<TPacketCGPosition>() as u16,
        position,
    };
    struct_to_bytes(&packet)
}

/// Helper to serialize `#[repr(C, packed)]` structs to a byte vector.
fn struct_to_bytes<T>(s: &T) -> Vec<u8> {
    let size = size_of::<T>();
    let ptr = s as *const T as *const u8;
    let bytes = unsafe { slice::from_raw_parts(ptr, size) };
    bytes.to_vec()
}
