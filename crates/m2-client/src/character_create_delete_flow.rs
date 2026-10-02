use std::mem::size_of;

pub const CHARACTER_NAME_MAX_LEN: usize = 64;
pub const PRIVATE_CODE_LENGTH: usize = 8;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Job {
    WarriorM = 0,
    NinjaF = 1,
    SuraM = 2,
    ShamanF = 3,
    WarriorF = 4,
    NinjaM = 5,
    SuraF = 6,
    ShamanM = 7,
}

impl Job {
    pub fn get_base_stats(&self) -> (u8, u8, u8, u8) {
        // (CON, INT, STR, DEX) matching C++
        match self {
            Job::WarriorM | Job::WarriorF => (4, 3, 6, 3), 
            Job::NinjaM | Job::NinjaF => (3, 3, 4, 6),
            Job::SuraM | Job::SuraF => (3, 7, 3, 3),
            Job::ShamanM | Job::ShamanF => (4, 6, 3, 3),
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGCreateCharacter {
    pub header: u16,
    pub length: u16,
    pub index: u8,
    pub name: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub job: u16,
    pub shape: u8,
    pub con: u8,
    pub int: u8,
    pub str: u8,
    pub dex: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketCGDestroyCharacter {
    pub header: u16,
    pub length: u16,
    pub index: u8,
    pub szPrivateCode: [u8; PRIVATE_CODE_LENGTH],
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TSimplePlayerInformation {
    pub dwID: u32,
    pub szName: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub byJob: u8,
    pub byLevel: u8,
    pub dwPlayMinutes: u32,
    pub byST: u8,
    pub byHT: u8,
    pub byDX: u8,
    pub byIQ: u8,
    pub wMainPart: u16,
    pub bChangeName: u8,
    pub wHairPart: u16,
    pub bDummy: [u8; 4],
    pub x: i32,
    pub y: i32,
    pub lAddr: u32,
    pub wPort: u16,
    pub bySkillGroup: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCPlayerCreateSuccess {
    pub header: u16,
    pub length: u16,
    pub bAccountCharacterSlot: u8,
    pub kSimplePlayerInfomation: TSimplePlayerInformation,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCCreateFailure {
    pub header: u16,
    pub length: u16,
    pub bType: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCDestroyCharacterSuccess {
    pub header: u16,
    pub length: u16,
    pub account_index: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TPacketGCBlank {
    pub header: u16,
    pub length: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateCharacterError {
    InvalidNameLength,
    InvalidNameCharacters,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteCharacterError {
    InvalidPinLength,
}

pub fn create_character(index: u8, name_str: &str, job: Job, shape: u8) -> Result<TPacketCGCreateCharacter, CreateCharacterError> {
    if name_str.len() < 3 || name_str.len() > 12 {
        return Err(CreateCharacterError::InvalidNameLength);
    }
    if !name_str.chars().all(|c| c.is_alphanumeric()) {
        return Err(CreateCharacterError::InvalidNameCharacters);
    }

    let mut name = [0u8; CHARACTER_NAME_MAX_LEN + 1];
    let bytes = name_str.as_bytes();
    let len = bytes.len();
    name[..len].copy_from_slice(bytes);

    let (con, int, str, dex) = job.get_base_stats();

    Ok(TPacketCGCreateCharacter {
        header: 0x0201,
        length: size_of::<TPacketCGCreateCharacter>() as u16,
        index,
        name,
        job: job as u16,
        shape,
        con,
        int,
        str,
        dex,
    })
}

pub fn delete_character(index: u8, pin_code: &str) -> Result<TPacketCGDestroyCharacter, DeleteCharacterError> {
    if pin_code.len() != 7 {
        return Err(DeleteCharacterError::InvalidPinLength);
    }

    let mut szPrivateCode = [0u8; PRIVATE_CODE_LENGTH];
    let bytes = pin_code.as_bytes();
    let len = bytes.len().min(PRIVATE_CODE_LENGTH - 1);
    szPrivateCode[..len].copy_from_slice(&bytes[..len]);

    Ok(TPacketCGDestroyCharacter {
        header: 0x0202,
        length: size_of::<TPacketCGDestroyCharacter>() as u16,
        index,
        szPrivateCode,
    })
}
