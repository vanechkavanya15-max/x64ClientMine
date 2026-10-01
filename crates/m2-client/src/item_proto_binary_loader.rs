use std::collections::HashMap;
use crate::vfs_security_decrypt::{decompress_lzo, decrypt_tea, SecurityKey};

pub const ITEM_NAME_MAX_LEN: usize = 64;
pub const ITEM_LIMIT_MAX_NUM: usize = 2;
pub const ITEM_APPLY_MAX_NUM: usize = 3;
pub const ITEM_VALUES_MAX_NUM: usize = 6;
pub const ITEM_SOCKET_MAX_NUM: usize = 3;

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct TItemLimit {
    pub bType: u8,
    pub lValue: i32,
}

#[repr(C, packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct TItemApply {
    pub bType: u8,
    pub lValue: i32,
}

#[repr(C, packed(1))]
#[derive(Debug, Clone)]
pub struct TItemTable {
    pub dwVnum: u32,
    pub dwVnumRange: u32,
    pub szName: [u8; ITEM_NAME_MAX_LEN + 1],
    pub szLocaleName: [u8; ITEM_NAME_MAX_LEN + 1],
    pub bType: u8,
    pub bSubType: u8,
    pub bWeight: u8,
    pub bSize: u8,
    pub dwAntiFlags: u32,
    pub dwFlags: u32,
    pub dwWearFlags: u32,
    pub dwImmuneFlag: u32,
    pub dwIBuyItemPrice: u32,
    pub dwISellItemPrice: u32,
    pub aLimits: [TItemLimit; ITEM_LIMIT_MAX_NUM],
    pub aApplies: [TItemApply; ITEM_APPLY_MAX_NUM],
    pub alValues: [i32; ITEM_VALUES_MAX_NUM],
    pub alSockets: [i32; ITEM_SOCKET_MAX_NUM],
    pub dwRefinedVnum: u32,
    pub wRefineSet: u16,
    pub bAlterToMagicItemPct: u8,
    pub bSpecular: u8,
    pub bGainSocketPct: u8,
}

#[repr(C, packed(4))]
struct CLZObjectHeader {
    dwFourCC: u32,
    dwEncryptSize: u32,
    dwCompressedSize: u32,
    dwRealSize: u32,
}

const MIPX_FOURCC: u32 = 0x5850494D; // 'MIPX'
const MCOZ_FOURCC: u32 = 0x5A4F434D; // 'MCOZ'
const ITEM_PROTO_KEY: SecurityKey = [173217, 72619434, 408587239, 27973291];

pub fn load_item_proto_binary(data: &[u8]) -> Result<HashMap<u32, TItemTable>, String> {
    if data.len() < 20 {
        return Err("Data too small to contain MIPX header".to_string());
    }

    let mut pos = 0;
    
    let four_cc = u32::from_le_bytes(data[pos..pos+4].try_into().unwrap());
    pos += 4;
    
    if four_cc != MIPX_FOURCC {
        return Err(format!("Invalid FourCC: expected MIPX, found {:X}", four_cc));
    }
    
    let version = u32::from_le_bytes(data[pos..pos+4].try_into().unwrap());
    pos += 4;
    
    if version != 1 {
        return Err(format!("Invalid item_proto version: {}", version));
    }
    
    let stride = u32::from_le_bytes(data[pos..pos+4].try_into().unwrap());
    pos += 4;
    
    if stride as usize != std::mem::size_of::<TItemTable>() {
        return Err(format!("Invalid item_proto stride: {} != {}", stride, std::mem::size_of::<TItemTable>()));
    }
    
    let elements = u32::from_le_bytes(data[pos..pos+4].try_into().unwrap());
    pos += 4;
    
    let data_size = u32::from_le_bytes(data[pos..pos+4].try_into().unwrap());
    pos += 4;
    
    if pos + data_size as usize > data.len() {
        return Err("Data size exceeds available bytes".to_string());
    }
    
    let lzo_data = &data[pos..pos + data_size as usize];
    
    // Parse CLZObject Header
    if lzo_data.len() < 16 {
        return Err("LZO data too small for header".to_string());
    }
    
    let lzo_four_cc = u32::from_le_bytes(lzo_data[0..4].try_into().unwrap());
    if lzo_four_cc != MCOZ_FOURCC {
        return Err("Invalid LZO FourCC".to_string());
    }
    
    let encrypt_size = u32::from_le_bytes(lzo_data[4..8].try_into().unwrap()) as usize;
    let compressed_size = u32::from_le_bytes(lzo_data[8..12].try_into().unwrap()) as usize;
    let real_size = u32::from_le_bytes(lzo_data[12..16].try_into().unwrap()) as usize;
    
    let mut decompressed_buffer: Vec<u8> = Vec::new();
    
    if encrypt_size > 0 {
        if 16 + encrypt_size > lzo_data.len() {
             return Err("Invalid encrypt size".to_string());
        }
        
        let mut decrypted_buffer = lzo_data[16..16 + encrypt_size].to_vec();
        decrypt_tea(&mut decrypted_buffer, &ITEM_PROTO_KEY);
        
        if decrypted_buffer.len() < 4 {
             return Err("Decrypted buffer too small".to_string());
        }
        
        let dec_four_cc = u32::from_le_bytes(decrypted_buffer[0..4].try_into().unwrap());
        if dec_four_cc != MCOZ_FOURCC {
            return Err("LZObject key incorrect".to_string());
        }
        
        if 4 + compressed_size > decrypted_buffer.len() {
            return Err("Invalid compressed size inside encrypted buffer".to_string());
        }
        
        decompressed_buffer = decompress_lzo(&decrypted_buffer[4..4 + compressed_size], real_size)?;
    } else {
        if 16 + compressed_size > lzo_data.len() {
            return Err("Invalid compressed size".to_string());
        }
        decompressed_buffer = decompress_lzo(&lzo_data[16..16 + compressed_size], real_size)?;
    }
    
    if decompressed_buffer.len() != real_size {
        return Err(format!("Decompressed size differs: {} != {}", decompressed_buffer.len(), real_size));
    }
    
    let expected_total_size = elements as usize * stride as usize;
    if decompressed_buffer.len() < expected_total_size {
        return Err("Decompressed buffer too small for expected elements".to_string());
    }
    
    let mut map = HashMap::new();
    let mut curr_pos = 0;
    
    for _ in 0..elements {
        let mut table: TItemTable = unsafe { std::mem::zeroed() };
        let slice = &decompressed_buffer[curr_pos..curr_pos + stride as usize];
        unsafe {
            std::ptr::copy_nonoverlapping(
                slice.as_ptr(),
                &mut table as *mut TItemTable as *mut u8,
                stride as usize,
            );
        }
        
        let vnum = {
            let mut val_bytes = [0u8; 4];
            unsafe {
                std::ptr::copy_nonoverlapping(
                    std::ptr::addr_of!(table.dwVnum) as *const u8,
                    val_bytes.as_mut_ptr(),
                    4,
                );
            }
            u32::from_le_bytes(val_bytes)
        };
        
        map.insert(vnum, table);
        curr_pos += stride as usize;
    }
    
    Ok(map)
}
