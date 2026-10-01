use std::collections::HashMap;
use std::mem::size_of;

use crate::vfs_security_decrypt::{decompress_lzo, decrypt_tea, SecurityKey};

pub const CHARACTER_NAME_MAX_LEN: usize = 64;
pub const MOB_ENCHANTS_MAX_NUM: usize = 6;
pub const MOB_RESISTS_MAX_NUM: usize = 11;
pub const MOB_SKILL_MAX_NUM: usize = 5;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TMobSkillLevel {
    pub dwVnum: u32,
    pub bLevel: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TMobTable {
    pub dwVnum: u32,
    pub szName: [u8; CHARACTER_NAME_MAX_LEN + 1],
    pub szLocaleName: [u8; CHARACTER_NAME_MAX_LEN + 1],

    pub bType: u8,
    pub bRank: u8,
    pub bBattleType: u8,
    pub bLevel: u8,
    pub bSize: u8,

    pub dwGoldMin: u32,
    pub dwGoldMax: u32,
    pub dwExp: u32,
    pub dwMaxHP: u32,
    pub bRegenCycle: u8,
    pub bRegenPercent: u8,
    pub wDef: u16,

    pub dwAIFlag: u32,
    pub dwRaceFlag: u32,
    pub dwImmuneFlag: u32,

    pub bStr: u8,
    pub bDex: u8,
    pub bCon: u8,
    pub bInt: u8,
    pub dwDamageRange: [u32; 2],

    pub sAttackSpeed: i16,
    pub sMovingSpeed: i16,
    pub bAggresiveHPPct: u8,
    pub wAggressiveSight: u16,
    pub wAttackRange: u16,

    pub cEnchants: [u8; MOB_ENCHANTS_MAX_NUM],
    pub cResists: [u8; MOB_RESISTS_MAX_NUM],

    pub dwResurrectionVnum: u32,
    pub dwDropItemVnum: u32,

    pub bMountCapacity: u8,
    pub bOnClickType: u8,

    pub bEmpire: u8,
    pub szFolder: [u8; 65],
    pub fDamMultiply: f32,
    pub dwSummonVnum: u32,
    pub dwDrainSP: u32,
    pub dwMonsterColor: u32,
    pub dwPolymorphItemVnum: u32,

    pub Skills: [TMobSkillLevel; MOB_SKILL_MAX_NUM],

    pub bBerserkPoint: u8,
    pub bStoneSkinPoint: u8,
    pub bGodSpeedPoint: u8,
    pub bDeathBlowPoint: u8,
    pub bRevivePoint: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LzoHeader {
    pub dwFourCC: u32,
    pub dwEncryptSize: u32,
    pub dwCompressedSize: u32,
    pub dwRealSize: u32,
}

pub const MOB_PROTO_KEY: SecurityKey = [4813894, 18955, 552631, 6822045];
pub const MMPT_FOURCC: u32 = u32::from_le_bytes(*b"MMPT");
pub const MCOZ_FOURCC: u32 = u32::from_le_bytes(*b"MCOZ");

pub fn load_mob_proto_binary(data: &[u8]) -> Result<HashMap<u32, TMobTable>, String> {
    if data.len() < 12 {
        return Err("Data too small to contain MMPT header".to_string());
    }

    let magic = u32::from_le_bytes(data[0..4].try_into().unwrap());
    if magic != MMPT_FOURCC {
        return Err("Invalid Mob proto type (not MMPT)".to_string());
    }

    let dwElements = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
    let dwDataSize = u32::from_le_bytes(data[8..12].try_into().unwrap()) as usize;

    if data.len() < 12 + dwDataSize {
        return Err("Data buffer smaller than expected dwDataSize".to_string());
    }

    let lzo_data = &data[12..12 + dwDataSize];

    if lzo_data.len() < size_of::<LzoHeader>() {
        return Err("Data too small for LZO header".to_string());
    }

    let lzo_header = LzoHeader {
        dwFourCC: u32::from_le_bytes(lzo_data[0..4].try_into().unwrap()),
        dwEncryptSize: u32::from_le_bytes(lzo_data[4..8].try_into().unwrap()),
        dwCompressedSize: u32::from_le_bytes(lzo_data[8..12].try_into().unwrap()),
        dwRealSize: u32::from_le_bytes(lzo_data[12..16].try_into().unwrap()),
    };

    if lzo_header.dwFourCC != MCOZ_FOURCC {
        return Err("Invalid LZO header fourcc".to_string());
    }

    let mut decompressed = Vec::new();

    if lzo_header.dwEncryptSize > 0 {
        let mut encrypted_buffer = vec![0u8; lzo_header.dwEncryptSize as usize];
        let enc_start = size_of::<LzoHeader>();
        let enc_end = enc_start + lzo_header.dwEncryptSize as usize;
        
        if lzo_data.len() < enc_end {
            return Err("Incomplete LZO encrypted data block".to_string());
        }
        
        encrypted_buffer.copy_from_slice(&lzo_data[enc_start..enc_end]);
        
        decrypt_tea(&mut encrypted_buffer, &MOB_PROTO_KEY);
        
        if encrypted_buffer.len() < 4 {
            return Err("Encrypted buffer too small to hold MCOZ magic".to_string());
        }
        
        let dec_magic = u32::from_le_bytes(encrypted_buffer[0..4].try_into().unwrap());
        if dec_magic != MCOZ_FOURCC {
            return Err("Decrypted LZO block has invalid fourcc".to_string());
        }
        
        let compressed_slice = &encrypted_buffer[4..];
        if compressed_slice.len() < lzo_header.dwCompressedSize as usize {
             return Err("Compressed payload smaller than dwCompressedSize".to_string());
        }
        
        decompressed = decompress_lzo(
            &compressed_slice[..lzo_header.dwCompressedSize as usize], 
            lzo_header.dwRealSize as usize
        )?;
    } else {
        let enc_start = size_of::<LzoHeader>();
        let enc_end = enc_start + lzo_header.dwCompressedSize as usize;
        if lzo_data.len() < enc_end {
            return Err("Incomplete LZO compressed data block".to_string());
        }
        let compressed_slice = &lzo_data[enc_start..enc_end];
        decompressed = decompress_lzo(compressed_slice, lzo_header.dwRealSize as usize)?;
    }
    
    if decompressed.len() != lzo_header.dwRealSize as usize {
        return Err("Decompressed size mismatch".to_string());
    }
    
    let table_size = size_of::<TMobTable>();
    if lzo_header.dwRealSize as usize % table_size != 0 {
        return Err("Decompressed data size is not a multiple of TMobTable size".to_string());
    }
    
    let expected_elements = lzo_header.dwRealSize as usize / table_size;
    if dwElements != expected_elements {
        return Err(format!("Element count mismatch: header={} computed={}", dwElements, expected_elements));
    }
    
    let mut table_map = HashMap::with_capacity(dwElements);
    
    for i in 0..dwElements {
        let start = i * table_size;
        let end = start + table_size;
        let chunk = &decompressed[start..end];
        
        let mut mob_table: TMobTable = unsafe { std::mem::zeroed() };
        unsafe {
            std::ptr::copy_nonoverlapping(
                chunk.as_ptr(),
                &mut mob_table as *mut TMobTable as *mut u8,
                table_size,
            );
        }
        
        let vnum = { mob_table.dwVnum };
        table_map.insert(vnum, mob_table);
    }
    
    Ok(table_map)
}

use crate::nonplayer_data::{NonPlayerData, NonPlayerDatabase, RANK_BOSS, RANK_KING};
use std::ffi::CStr;

pub fn load_mob_proto_vfs(vfs_data: &[u8], db: &mut NonPlayerDatabase) -> Result<(), String> {
    let table_map = load_mob_proto_binary(vfs_data)?;
    for (_, mob_table) in table_map {
        let name_bytes = &mob_table.szLocaleName;
        let mut name_len = 0;
        while name_len < name_bytes.len() && name_bytes[name_len] != 0 {
            name_len += 1;
        }
        let mob_name = String::from_utf8_lossy(&name_bytes[..name_len]).into_owned();

        // Check if aggressive flag is present in dwAIFlag (assuming standard flag AIFLAG_AGGRESSIVE = 1 or similar)
        // From Metin2 source AIFLAG_AGGRESSIVE = (1 << 0)
        let is_aggressive = (mob_table.dwAIFlag & (1 << 0)) != 0;

        let npd = NonPlayerData {
            race_vnum: mob_table.dwVnum,
            mob_name,
            rank: mob_table.bRank,
            mob_type: mob_table.bType,
            is_aggressive,
            attack_speed: mob_table.sAttackSpeed as u32,
            move_speed: mob_table.sMovingSpeed as u32,
        };

        db.register_mob(npd);
    }
    Ok(())
}
