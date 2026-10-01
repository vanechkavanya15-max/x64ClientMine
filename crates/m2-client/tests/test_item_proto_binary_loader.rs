use std::collections::HashMap;
use m2_client::item_proto_binary_loader::{
    load_item_proto_binary, TItemTable, TItemLimit, TItemApply,
    ITEM_NAME_MAX_LEN, ITEM_LIMIT_MAX_NUM, ITEM_APPLY_MAX_NUM, ITEM_VALUES_MAX_NUM, ITEM_SOCKET_MAX_NUM
};
use m2_client::vfs_security_decrypt::{encrypt_tea, compress_zstd, SecurityKey}; // Need LZO actually, but we can mock unencrypted or construct LZO compressed data. Wait, we need LZO compress or we can use unencrypted with `compress_size == real_size` logic.
// wait `decompress_lzo` supports `compressed.len() >= expected_size && &compressed[..expected_size] == compressed` as uncompressed fallback!

// We can just construct a valid `MIPX` buffer with uncompressed fallback if we pass `encrypt_size = 0` and uncompressed payload for LZO.

fn create_dummy_item_table() -> TItemTable {
    let mut table: TItemTable = unsafe { std::mem::zeroed() };
    table.dwVnum = 12345;
    table.dwVnumRange = 0;
    
    let name = b"Dummy Sword";
    table.szName[..name.len()].copy_from_slice(name);
    let locale_name = b"Dummy Locale Sword";
    table.szLocaleName[..locale_name.len()].copy_from_slice(locale_name);
    
    table.bType = 1;
    table.bSubType = 2;
    table.bWeight = 10;
    table.bSize = 2;
    table.dwAntiFlags = 256;
    table.dwFlags = 512;
    table.dwWearFlags = 1024;
    table.dwImmuneFlag = 2048;
    table.dwIBuyItemPrice = 10000;
    table.dwISellItemPrice = 5000;
    
    table.aLimits[0] = TItemLimit { bType: 1, lValue: 30 };
    table.aApplies[0] = TItemApply { bType: 2, lValue: 50 };
    
    table.alValues[0] = 100;
    table.alValues[1] = 200;
    
    table.alSockets[0] = 1;
    
    table.dwRefinedVnum = 12346;
    table.wRefineSet = 10;
    table.bAlterToMagicItemPct = 5;
    table.bSpecular = 50;
    table.bGainSocketPct = 100;
    
    table
}

fn create_valid_mipx_buffer(encrypt: bool) -> Vec<u8> {
    let mut data = Vec::new();
    
    // MIPX Header
    data.extend_from_slice(&0x5850494Du32.to_le_bytes()); // 'MIPX'
    data.extend_from_slice(&1u32.to_le_bytes()); // Version
    let stride = std::mem::size_of::<TItemTable>() as u32;
    data.extend_from_slice(&stride.to_le_bytes()); // Stride
    data.extend_from_slice(&1u32.to_le_bytes()); // Elements
    
    // Construct TItemTable data
    let table = create_dummy_item_table();
    let table_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            &table as *const TItemTable as *const u8,
            std::mem::size_of::<TItemTable>()
        )
    };
    
    let real_size = table_bytes.len() as u32;
    
    // The "compressed" data. We will rely on decompress_lzo's uncompressed fallback:
    // "if compressed_data.len() >= expected_size && &compressed_data[..expected_size] == compressed_data { return out; }"
    let mut payload = table_bytes.to_vec();
    
    let mut clz_header = Vec::new();
    clz_header.extend_from_slice(&0x5A4F434Du32.to_le_bytes()); // 'MCOZ'
    
    if encrypt {
        // Pad payload to multiple of 8 for TEA
        while payload.len() % 8 != 0 {
            payload.push(0);
        }
        
        let encrypt_size = payload.len() as u32;
        let compressed_size = real_size;
        
        clz_header.extend_from_slice(&encrypt_size.to_le_bytes());
        clz_header.extend_from_slice(&compressed_size.to_le_bytes());
        clz_header.extend_from_slice(&real_size.to_le_bytes());
        
        let mut encrypted_payload = Vec::new();
        encrypted_payload.extend_from_slice(&0x5A4F434Du32.to_le_bytes()); // Decrypted fourCC check
        encrypted_payload.extend_from_slice(&payload);
        
        // Pad encrypted_payload to multiple of 8
        while encrypted_payload.len() % 8 != 0 {
            encrypted_payload.push(0);
        }
        
        let key: SecurityKey = [173217, 72619434, 408587239, 27973291];
        encrypt_tea(&mut encrypted_payload, &key);
        
        // Fix the clz header encrypt size to be the full encrypted length
        let final_encrypt_size = encrypted_payload.len() as u32;
        clz_header[4..8].copy_from_slice(&final_encrypt_size.to_le_bytes());
        
        clz_header.extend_from_slice(&encrypted_payload);
    } else {
        clz_header.extend_from_slice(&0u32.to_le_bytes()); // EncryptSize
        clz_header.extend_from_slice(&real_size.to_le_bytes()); // CompressedSize
        clz_header.extend_from_slice(&real_size.to_le_bytes()); // RealSize
        clz_header.extend_from_slice(&payload);
    }
    
    let data_size = clz_header.len() as u32;
    data.extend_from_slice(&data_size.to_le_bytes());
    data.extend_from_slice(&clz_header);
    
    data
}

#[test]
fn test_parse_valid_mipx_unencrypted() {
    let buffer = create_valid_mipx_buffer(false);
    let result = load_item_proto_binary(&buffer);
    assert!(result.is_ok(), "Failed to parse unencrypted MIPX: {:?}", result.err());
    
    let map = result.unwrap();
    assert_eq!(map.len(), 1);
    
    let item = map.get(&12345).expect("Item not found");
    assert_eq!(item.bType, 1);
    assert_eq!(u32::from_le_bytes(unsafe { std::ptr::read_unaligned(std::ptr::addr_of!(item.dwIBuyItemPrice) as *const [u8; 4]) }), 10000);
}

#[test]
fn test_parse_valid_mipx_encrypted() {
    let buffer = create_valid_mipx_buffer(true);
    let result = load_item_proto_binary(&buffer);
    assert!(result.is_ok(), "Failed to parse encrypted MIPX: {:?}", result.err());
    
    let map = result.unwrap();
    assert_eq!(map.len(), 1);
    
    let item = map.get(&12345).expect("Item not found");
    assert_eq!(item.bSubType, 2);
    assert_eq!(u32::from_le_bytes(unsafe { std::ptr::read_unaligned(std::ptr::addr_of!(item.dwRefinedVnum) as *const [u8; 4]) }), 12346);
}

#[test]
fn test_invalid_fourcc() {
    let mut buffer = create_valid_mipx_buffer(false);
    buffer[0..4].copy_from_slice(b"BADX");
    
    let result = load_item_proto_binary(&buffer);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid FourCC"));
}

#[test]
fn test_invalid_version() {
    let mut buffer = create_valid_mipx_buffer(false);
    buffer[4..8].copy_from_slice(&2u32.to_le_bytes()); // Set version to 2
    
    let result = load_item_proto_binary(&buffer);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("version"));
}

#[test]
fn test_invalid_stride() {
    let mut buffer = create_valid_mipx_buffer(false);
    buffer[8..12].copy_from_slice(&999u32.to_le_bytes()); // Set invalid stride
    
    let result = load_item_proto_binary(&buffer);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("stride"));
}

#[test]
fn test_data_too_small() {
    let buffer = vec![0u8; 10]; // Less than 16 bytes
    let result = load_item_proto_binary(&buffer);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("too small"));
}

#[test]
fn test_invalid_lzo_fourcc() {
    let mut buffer = create_valid_mipx_buffer(false);
    // Find where the LZO header starts (after 20 bytes of MIPX header)
    buffer[20..24].copy_from_slice(b"BADO");
    
    let result = load_item_proto_binary(&buffer);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("LZO FourCC"));
}
