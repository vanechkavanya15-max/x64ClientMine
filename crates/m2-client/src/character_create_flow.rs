use std::mem::size_of;

/// Job (Class) definitions according to C++ `introselect.py` and C++ enums.
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
    /// Get default base stats for the specific Job.
    /// Returns (STR, DEX, VIT, INT)
    pub fn get_base_stats(&self) -> (u8, u8, u8, u8) {
        // SourceCodeClient based defaults for jobs
        match self {
            Job::WarriorM | Job::WarriorF => (6, 3, 4, 3), 
            Job::NinjaM | Job::NinjaF => (4, 6, 3, 3),
            Job::SuraM | Job::SuraF => (3, 3, 3, 7),
            Job::ShamanM | Job::ShamanF => (3, 3, 4, 6),
        }
    }
}

/// Character Create Packet
#[repr(C, packed)]
pub struct TPacketCGPlayerCreate {
    pub header: u16,        // 0x0201
    pub length: u16,        // 73 
    pub index: u8,          // slot 0..3
    pub name: [u8; 61],     // Fixed to 61 to match total length 73.
    pub job: u16,           // class
    pub shape: u8,          // appearance 0..1
    pub stat_str: u8,
    pub stat_dex: u8,
    pub stat_vit: u8,
    pub stat_int: u8,
}

impl TPacketCGPlayerCreate {
    pub fn new(index: u8, name_str: &str, job: Job, shape: u8) -> Self {
        let mut name = [0u8; 61];
        let bytes = name_str.as_bytes();
        let len = bytes.len().min(60); // reserve 1 for null terminator just in case
        name[..len].copy_from_slice(&bytes[..len]);
        
        let stats = job.get_base_stats();
        
        Self {
            header: 0x0201,
            length: 73,
            index,
            name,
            job: job as u16,
            shape,
            stat_str: stats.0,
            stat_dex: stats.1,
            stat_vit: stats.2,
            stat_int: stats.3,
        }
    }
    
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

/// Server response packets
#[repr(C, packed)]
pub struct TPacketGCPlayerCreateSuccess {
    pub header: u16, // 0x020C
    pub length: u16,
    pub b_account_character_index: u8,
    pub player: TSimplePlayer,
}

#[repr(C, packed)]
pub struct TPacketGCPlayerCreateFailure {
    pub header: u16, // 0x020D
    pub length: u16,
    pub b_type: u8,
}

#[repr(C, packed)]
pub struct TSimplePlayer {
    pub id: u32,
    pub name: [u8; 25],
    pub job: u8,
    pub level: u8,
    pub play_time: u32,
    pub st: u8,
    pub ht: u8,
    pub dx: u8,
    pub iq: u8,
    pub main_part: u16,
    pub hair_part: u16,
    pub x: u32,
    pub y: u32,
    pub skill_group: u8,
    pub change_name: u8,
}
