#![allow(non_snake_case)]

// Struktury pakietow dodawania i aktualizacji postaci (Game -> Client)
// Zgodne z E:\m2dev-client-src-main\src\UserInterface\Packet.h

use std::panic::catch_unwind;
use std::ptr;

pub const CHARACTER_NAME_MAX_LEN: usize = 64;
pub const CHR_EQUIPPART_NUM: usize = 4;

pub const HEADER_GC_CHARACTER_ADD: u16 = 0x0205;
pub const HEADER_GC_CHARACTER_ADD2: u16 = 0x0206;
pub const HEADER_GC_CHAR_ADDITIONAL_INFO: u16 = 0x0207;
pub const HEADER_GC_CHARACTER_UPDATE: u16 = 0x0209;
pub const HEADER_GC_CHARACTER_UPDATE2: u16 = 0x020A;

#[derive(Debug, PartialEq, Eq)]
pub enum PacketError {
    Eof,
    InvalidHeader,
}

// Struktura pakietu TPacketGCCharacterAdd
// Rozmiar dokladnie 38 bajtow:
// header(2) + length(2) + dwVID(4) + angle(4) + x(4) + y(4) + z(4) + bType(1) + wRaceNum(2)
// + bMovingSpeed(1) + bAttackSpeed(1) + bStateFlag(1) + dwAffectFlag(8) = 38
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCCharacterAdd {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub angle: f32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub bType: u8,
    pub wRaceNum: u16,
    pub bMovingSpeed: u8,
    pub bAttackSpeed: u8,
    pub bStateFlag: u8,
    pub dwAffectFlag: [u32; 2],
}

impl TPacketGCCharacterAdd {
    pub const PACKET_SIZE: usize = 38;

    pub fn from_bytes(buf: &[u8]) -> Result<Self, PacketError> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(PacketError::Eof);
        }

        let header = u16::from_le_bytes(buf[0..2].try_into().unwrap());
        let length = u16::from_le_bytes(buf[2..4].try_into().unwrap());
        let dwVID = u32::from_le_bytes(buf[4..8].try_into().unwrap());
        let angle = f32::from_le_bytes(buf[8..12].try_into().unwrap());
        let x = i32::from_le_bytes(buf[12..16].try_into().unwrap());
        let y = i32::from_le_bytes(buf[16..20].try_into().unwrap());
        let z = i32::from_le_bytes(buf[20..24].try_into().unwrap());
        let bType = buf[24];
        let wRaceNum = u16::from_le_bytes(buf[25..27].try_into().unwrap());
        let bMovingSpeed = buf[27];
        let bAttackSpeed = buf[28];
        let bStateFlag = buf[29];
        let dwAffectFlag = [
            u32::from_le_bytes(buf[30..34].try_into().unwrap()),
            u32::from_le_bytes(buf[34..38].try_into().unwrap()),
        ];

        Ok(TPacketGCCharacterAdd {
            header,
            length,
            dwVID,
            angle,
            x,
            y,
            z,
            bType,
            wRaceNum,
            bMovingSpeed,
            bAttackSpeed,
            bStateFlag,
            dwAffectFlag,
        })
    }

    // Akcesory dla wygody
    pub fn dw_vid(&self) -> u32 {
        unsafe { ptr::addr_of!(self.dwVID).read_unaligned() }
    }

    pub fn w_race_num(&self) -> u16 {
        unsafe { ptr::addr_of!(self.wRaceNum).read_unaligned() }
    }

    pub fn b_type(&self) -> u8 {
        unsafe { ptr::addr_of!(self.bType).read_unaligned() }
    }

    pub fn b_moving_speed(&self) -> u8 {
        unsafe { ptr::addr_of!(self.bMovingSpeed).read_unaligned() }
    }

    pub fn b_attack_speed(&self) -> u8 {
        unsafe { ptr::addr_of!(self.bAttackSpeed).read_unaligned() }
    }

    pub fn b_state_flag(&self) -> u8 {
        unsafe { ptr::addr_of!(self.bStateFlag).read_unaligned() }
    }

    pub fn dw_affect_flag(&self) -> [u32; 2] {
        unsafe { ptr::addr_of!(self.dwAffectFlag).read_unaligned() }
    }
}

// Struktura pakietu TPacketGCCharacterAdd2
// Rozmiar 123 bajty:
// header(2) + length(2) + dwVID(4) + name(65) + angle(4) + x(4) + y(4) + z(4) + bType(1)
// + wRaceNum(2) + awPart(8) + bMovingSpeed(1) + bAttackSpeed(1) + bStateFlag(1)
// + dwAffectFlag(8) + bEmpire(1) + dwGuild(4) + sAlignment(2) + bPKMode(1) + dwMountVnum(4) = 123
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCCharacterAdd2 {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub angle: f32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub bType: u8,
    pub wRaceNum: u16,
    pub awPart: [u16; CHR_EQUIPPART_NUM],
    pub bMovingSpeed: u8,
    pub bAttackSpeed: u8,
    pub bStateFlag: u8,
    pub dwAffectFlag: [u32; 2],
    pub bEmpire: u8,
    pub dwGuild: u32,
    pub sAlignment: i16,
    pub bPKMode: u8,
    pub dwMountVnum: u32,
}

impl TPacketGCCharacterAdd2 {
    pub const PACKET_SIZE: usize = 123;

    pub fn from_bytes(buf: &[u8]) -> Result<Self, PacketError> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(PacketError::Eof);
        }

        let mut name = [0u8; CHARACTER_NAME_MAX_LEN + 1];
        name.copy_from_slice(&buf[8..73]);

        let mut awPart = [0u16; CHR_EQUIPPART_NUM];
        awPart[0] = u16::from_le_bytes(buf[88..90].try_into().unwrap());
        awPart[1] = u16::from_le_bytes(buf[90..92].try_into().unwrap());
        awPart[2] = u16::from_le_bytes(buf[92..94].try_into().unwrap());
        awPart[3] = u16::from_le_bytes(buf[94..96].try_into().unwrap());

        Ok(TPacketGCCharacterAdd2 {
            header: u16::from_le_bytes(buf[0..2].try_into().unwrap()),
            length: u16::from_le_bytes(buf[2..4].try_into().unwrap()),
            dwVID: u32::from_le_bytes(buf[4..8].try_into().unwrap()),
            name,
            angle: f32::from_le_bytes(buf[73..77].try_into().unwrap()),
            x: i32::from_le_bytes(buf[77..81].try_into().unwrap()),
            y: i32::from_le_bytes(buf[81..85].try_into().unwrap()),
            z: i32::from_le_bytes(buf[85..89].try_into().unwrap()),
            bType: buf[89],
            wRaceNum: u16::from_le_bytes(buf[90..92].try_into().unwrap()),
            awPart,
            bMovingSpeed: buf[96],
            bAttackSpeed: buf[97],
            bStateFlag: buf[98],
            dwAffectFlag: [
                u32::from_le_bytes(buf[99..103].try_into().unwrap()),
                u32::from_le_bytes(buf[103..107].try_into().unwrap()),
            ],
            bEmpire: buf[107],
            dwGuild: u32::from_le_bytes(buf[108..112].try_into().unwrap()),
            sAlignment: i16::from_le_bytes(buf[112..114].try_into().unwrap()),
            bPKMode: buf[114],
            dwMountVnum: u32::from_le_bytes(buf[115..119].try_into().unwrap()),
        })
    }
}

// Struktura pakietu TPacketGCCharacterAdditionalInfo
// Rozmiar 97 bajtow:
// header(2) + length(2) + dwVID(4) + name(65) + awPart(8) + bEmpire(1)
// + dwGuildID(4) + dwLevel(4) + sAlignment(2) + bPKMode(1) + dwMountVnum(4) = 97
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCCharacterAdditionalInfo {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub awPart: [u16; CHR_EQUIPPART_NUM],
    pub bEmpire: u8,
    pub dwGuildID: u32,
    pub dwLevel: u32,
    pub sAlignment: i16,
    pub bPKMode: u8,
    pub dwMountVnum: u32,
}

impl TPacketGCCharacterAdditionalInfo {
    pub const PACKET_SIZE: usize = 97;

    pub fn from_bytes(buf: &[u8]) -> Result<Self, PacketError> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(PacketError::Eof);
        }

        let mut name = [0u8; CHARACTER_NAME_MAX_LEN + 1];
        name.copy_from_slice(&buf[8..73]);

        let mut awPart = [0u16; CHR_EQUIPPART_NUM];
        awPart[0] = u16::from_le_bytes(buf[73..75].try_into().unwrap());
        awPart[1] = u16::from_le_bytes(buf[75..77].try_into().unwrap());
        awPart[2] = u16::from_le_bytes(buf[77..79].try_into().unwrap());
        awPart[3] = u16::from_le_bytes(buf[79..81].try_into().unwrap());

        Ok(TPacketGCCharacterAdditionalInfo {
            header: u16::from_le_bytes(buf[0..2].try_into().unwrap()),
            length: u16::from_le_bytes(buf[2..4].try_into().unwrap()),
            dwVID: u32::from_le_bytes(buf[4..8].try_into().unwrap()),
            name,
            awPart,
            bEmpire: buf[81],
            dwGuildID: u32::from_le_bytes(buf[82..86].try_into().unwrap()),
            dwLevel: u32::from_le_bytes(buf[86..90].try_into().unwrap()),
            sAlignment: i16::from_le_bytes(buf[90..92].try_into().unwrap()),
            bPKMode: buf[92],
            dwMountVnum: u32::from_le_bytes(buf[93..97].try_into().unwrap()),
        })
    }
}

// Struktura pakietu TPacketGCCharacterUpdate
// Rozmiar 38 bajtow:
// header(2) + length(2) + dwVID(4) + awPart(8) + bMovingSpeed(1) + bAttackSpeed(1)
// + bStateFlag(1) + dwAffectFlag(8) + dwGuildID(4) + sAlignment(2) + bPKMode(1) + dwMountVnum(4) = 38
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_snake_case)]
pub struct TPacketGCCharacterUpdate {
    pub header: u16,
    pub length: u16,
    pub dwVID: u32,
    pub awPart: [u16; CHR_EQUIPPART_NUM],
    pub bMovingSpeed: u8,
    pub bAttackSpeed: u8,
    pub bStateFlag: u8,
    pub dwAffectFlag: [u32; 2],
    pub dwGuildID: u32,
    pub sAlignment: i16,
    pub bPKMode: u8,
    pub dwMountVnum: u32,
}

impl TPacketGCCharacterUpdate {
    pub const PACKET_SIZE: usize = 38;

    pub fn from_bytes(buf: &[u8]) -> Result<Self, PacketError> {
        if buf.len() < Self::PACKET_SIZE {
            return Err(PacketError::Eof);
        }

        let mut awPart = [0u16; CHR_EQUIPPART_NUM];
        awPart[0] = u16::from_le_bytes(buf[8..10].try_into().unwrap());
        awPart[1] = u16::from_le_bytes(buf[10..12].try_into().unwrap());
        awPart[2] = u16::from_le_bytes(buf[12..14].try_into().unwrap());
        awPart[3] = u16::from_le_bytes(buf[14..16].try_into().unwrap());

        Ok(TPacketGCCharacterUpdate {
            header: u16::from_le_bytes(buf[0..2].try_into().unwrap()),
            length: u16::from_le_bytes(buf[2..4].try_into().unwrap()),
            dwVID: u32::from_le_bytes(buf[4..8].try_into().unwrap()),
            awPart,
            bMovingSpeed: buf[16],
            bAttackSpeed: buf[17],
            bStateFlag: buf[18],
            dwAffectFlag: [
                u32::from_le_bytes(buf[19..23].try_into().unwrap()),
                u32::from_le_bytes(buf[23..27].try_into().unwrap()),
            ],
            dwGuildID: u32::from_le_bytes(buf[27..31].try_into().unwrap()),
            sAlignment: i16::from_le_bytes(buf[31..33].try_into().unwrap()),
            bPKMode: buf[33],
            dwMountVnum: u32::from_le_bytes(buf[34..38].try_into().unwrap()),
        })
    }
}

pub type TPacketGCCharacterUpdate2 = TPacketGCCharacterUpdate;

#[no_mangle]
pub unsafe extern "C" fn m2_net_parse_char_add(
    data: *const u8,
    len: u32,
    out_vid: *mut u32,
    out_x: *mut i32,
    out_y: *mut i32,
    out_race: *mut u16,
) -> bool {
    let result = catch_unwind(std::panic::AssertUnwindSafe(|| {
        if data.is_null() || out_vid.is_null() || out_x.is_null() || out_y.is_null() || out_race.is_null() {
            return false;
        }

        let slice = std::slice::from_raw_parts(data, len as usize);
        match TPacketGCCharacterAdd::from_bytes(slice) {
            Ok(packet) => {
                ptr::write(out_vid, ptr::addr_of!(packet.dwVID).read_unaligned());
                ptr::write(out_x, ptr::addr_of!(packet.x).read_unaligned());
                ptr::write(out_y, ptr::addr_of!(packet.y).read_unaligned());
                ptr::write(out_race, ptr::addr_of!(packet.wRaceNum).read_unaligned());
                true
            }
            Err(_) => false,
        }
    }));

    result.unwrap_or(false)
}
