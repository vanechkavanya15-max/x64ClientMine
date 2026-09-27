use std::convert::TryInto;
use std::ffi::CStr;

pub const GRANNY_MAGIC_BYTES: [u8; 16] = *b"GRANNY_MAGIC_VAL";

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Gr2Header {
    pub magic: [u8; 16],
    pub format_version: u32,
    pub total_size: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gr2ModelInfo {
    pub name: String,
    pub bone_count: u32,
    pub mesh_count: u32,
    pub animation_count: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gr2FileData {
    pub info: Gr2ModelInfo,
    pub bones: Vec<String>,
}

pub fn parse_gr2_memory(data: &[u8]) -> Result<Gr2ModelInfo, String> {
    if data.len() < 24 {
        return Err("Buffer too small for Gr2Header".to_string());
    }

    let magic: [u8; 16] = data[0..16].try_into().map_err(|_| "Failed to extract magic")?;
    if magic != GRANNY_MAGIC_BYTES {
        return Err(format!("Invalid magic bytes: {:?}", magic));
    }

    let format_version = u32::from_le_bytes(data[16..20].try_into().unwrap());
    let total_size = u32::from_le_bytes(data[20..24].try_into().unwrap());

    if data.len() < total_size as usize {
        return Err(format!("Buffer length ({}) is less than total_size ({})", data.len(), total_size));
    }

    let mut offset = 24;
    
    // Parse name (null-terminated string)
    let null_pos = data[offset..].iter().position(|&b| b == 0).ok_or("Name is not null-terminated")?;
    let name_bytes = &data[offset..offset + null_pos + 1];
    let name = CStr::from_bytes_with_nul(name_bytes)
        .map_err(|e| format!("Invalid C string for name: {}", e))?
        .to_str()
        .map_err(|e| format!("Name is not valid UTF-8: {}", e))?
        .to_string();
    
    offset += null_pos + 1;

    if data.len() < offset + 12 {
        return Err("Buffer truncated before model counts".to_string());
    }

    let bone_count = u32::from_le_bytes(data[offset..offset+4].try_into().unwrap());
    let mesh_count = u32::from_le_bytes(data[offset+4..offset+8].try_into().unwrap());
    let animation_count = u32::from_le_bytes(data[offset+8..offset+12].try_into().unwrap());

    Ok(Gr2ModelInfo {
        name,
        bone_count,
        mesh_count,
        animation_count,
    })
}

pub fn extract_bone_names(data: &[u8]) -> Vec<String> {
    let mut bones = Vec::new();
    
    // Attempt to parse info first to get bone_count and starting offset
    let info = match parse_gr2_memory(data) {
        Ok(info) => info,
        Err(_) => return bones, // Return empty if header parsing fails
    };

    let mut offset = 24;
    
    // Skip name
    if let Some(null_pos) = data[offset..].iter().position(|&b| b == 0) {
        offset += null_pos + 1;
    } else {
        return bones;
    }

    // Skip counts
    offset += 12;

    for _ in 0..info.bone_count {
        if offset >= data.len() {
            break;
        }

        if let Some(null_pos) = data[offset..].iter().position(|&b| b == 0) {
            let name_bytes = &data[offset..offset + null_pos + 1];
            if let Ok(cstr) = CStr::from_bytes_with_nul(name_bytes) {
                if let Ok(s) = cstr.to_str() {
                    bones.push(s.to_string());
                } else {
                    break;
                }
            } else {
                break;
            }
            offset += null_pos + 1;
        } else {
            break;
        }
    }

    bones
}
