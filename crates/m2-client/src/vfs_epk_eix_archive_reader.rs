use std::collections::HashMap;
use crate::vfs_security_decrypt::{decompress_lzo, decrypt_tea, SecurityKey};

pub const EPKD_FOURCC: u32 = 0x444B5045; // 'EPKD'
pub const MCOZ_FOURCC: u32 = 0x5A4F434D; // 'MCOZ'

pub const COMPRESSED_TYPE_NONE: u8 = 0;
pub const COMPRESSED_TYPE_COMPRESS: u8 = 1;
pub const COMPRESSED_TYPE_SECURITY: u8 = 2;
pub const COMPRESSED_TYPE_PANAMA: u8 = 3;
pub const COMPRESSED_TYPE_HYBRIDCRYPT: u8 = 4;

#[repr(C, packed(4))]
#[derive(Debug, Clone, Copy)]
pub struct TEterPackIndex {
    pub id: i32,
    pub filename: [u8; 161],
    pub filename_crc: u32,
    pub real_data_size: i32,
    pub data_size: i32,
    pub data_crc: u32,
    pub data_position: i32,
    pub compressed_type: u8,
}

impl TEterPackIndex {
    pub fn get_filename(&self) -> String {
        let pos = self.filename.iter().position(|&c| c == 0).unwrap_or(self.filename.len());
        String::from_utf8_lossy(&self.filename[..pos]).to_string()
    }
}

pub fn load_eix_index(eix_data: &[u8], eix_key: &SecurityKey) -> Result<HashMap<String, TEterPackIndex>, String> {
    if eix_data.len() < 12 {
        return Err("Data too small to contain EPKD header".to_string());
    }

    let mut pos = 0;
    
    let four_cc = u32::from_le_bytes(eix_data[pos..pos+4].try_into().unwrap());
    pos += 4;
    
    if four_cc != EPKD_FOURCC && four_cc != MCOZ_FOURCC {
        return Err(format!("Invalid FourCC: expected EPKD or MCOZ, found {:X}", four_cc));
    }
    
    if four_cc == EPKD_FOURCC {
        let _version = u32::from_le_bytes(eix_data[pos..pos+4].try_into().unwrap());
        pos += 4;
        let _index_count = u32::from_le_bytes(eix_data[pos..pos+4].try_into().unwrap());
        pos += 4;
        // The remaining data should be MCOZ LZO block
    } else {
        // MCOZ without EPKD header
        pos = 0;
    }

    let lzo_data = &eix_data[pos..];

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
    
    let decompressed_buffer: Vec<u8> = if encrypt_size > 0 {
        if 16 + encrypt_size > lzo_data.len() {
            return Err("Invalid encrypt size".to_string());
        }
        
        // Decrypt the MCOZ block using TEA algorithm with the provided security key
        let mut decrypted_buffer = lzo_data[16..16 + encrypt_size].to_vec();
        decrypt_tea(&mut decrypted_buffer, eix_key);
        
        if decrypted_buffer.len() < 4 {
            return Err("Decrypted buffer too small".to_string());
        }
        
        let dec_four_cc = u32::from_le_bytes(decrypted_buffer[0..4].try_into().unwrap());
        if dec_four_cc != MCOZ_FOURCC {
            return Err("LZObject key incorrect after TEA decryption".to_string());
        }
        
        if 4 + compressed_size > decrypted_buffer.len() {
            return Err("Invalid compressed size inside encrypted buffer".to_string());
        }
        
        // Perform LZO decompression on the extracted payload
        decompress_lzo(&decrypted_buffer[4..4 + compressed_size], real_size)?
    } else {
        if 16 + compressed_size > lzo_data.len() {
            return Err("Invalid compressed size".to_string());
        }
        // Unencrypted block, perform LZO decompression directly
        decompress_lzo(&lzo_data[16..16 + compressed_size], real_size)?
    };
    
    if decompressed_buffer.len() != real_size {
        return Err(format!("Decompressed size differs: {} != {}", decompressed_buffer.len(), real_size));
    }
    
    let stride = std::mem::size_of::<TEterPackIndex>();
    if decompressed_buffer.len() % stride != 0 {
        return Err("Decompressed buffer size is not a multiple of TEterPackIndex size".to_string());
    }
    
    let mut map = HashMap::new();
    
    // Extract metadata from the decompressed LZO payload: filename, CRC, offset in EPK, compressed and uncompressed sizes
    for slice in decompressed_buffer.chunks_exact(stride) {
        let mut index: TEterPackIndex = unsafe { std::mem::zeroed() };
        unsafe {
            std::ptr::copy_nonoverlapping(
                slice.as_ptr(),
                &mut index as *mut TEterPackIndex as *mut u8,
                stride,
            );
        }
        
        let filename = index.get_filename();
        map.insert(filename, index);
    }
    
    Ok(map)
}

pub fn read_epk_file_data(epk_data: &[u8], index: &TEterPackIndex) -> Result<Vec<u8>, String> {
    let start = { index.data_position } as usize;
    let real_data_size = { index.real_data_size } as usize;
    let data_size = { index.data_size } as usize;
    let compressed_type = { index.compressed_type };
    
    let length = data_size;
    
    if start + length > epk_data.len() {
        return Err("Data out of bounds in EPK".to_string());
    }
    
    let slice = &epk_data[start..start + length];
    
    match compressed_type {
        COMPRESSED_TYPE_NONE => {
            if slice.len() != real_data_size {
                return Err("Uncompressed data size mismatch".to_string());
            }
            Ok(slice.to_vec())
        }
        COMPRESSED_TYPE_COMPRESS => {
            // Apply LZO block decompression for EPK file data
            let decompressed_payload = decompress_lzo(slice, real_data_size)?;
            Ok(decompressed_payload)
        }
        _ => Err(format!("Unsupported compression type: {}", compressed_type)),
    }
}
