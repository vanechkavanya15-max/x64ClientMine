use std::collections::HashMap;
use std::mem::size_of;

use m2_client::mob_proto_binary_loader::{
    load_mob_proto_binary, LzoHeader, TMobTable, CHARACTER_NAME_MAX_LEN, MCOZ_FOURCC, MMPT_FOURCC,
    MOB_PROTO_KEY,
};
use m2_client::vfs_security_decrypt::{encrypt_tea, compress_zstd, SecurityKey}; // Warning: we must simulate LZO for test

// Helper function to simulate LZO encoding with unencrypted payload (for simpler test)
fn create_mock_mob_proto() -> Vec<u8> {
    let mut table1: TMobTable = unsafe { std::mem::zeroed() };
    table1.dwVnum = 101;
    let name1 = b"Wild Dog\0";
    table1.szName[..name1.len()].copy_from_slice(name1);
    table1.szLocaleName[..name1.len()].copy_from_slice(name1);
    table1.bLevel = 1;
    table1.dwMaxHP = 200;
    table1.sAttackSpeed = 100;
    table1.sMovingSpeed = 100;
    table1.bType = 0; // Monster
    table1.bRank = 0; // PAWN
    
    let mut table2: TMobTable = unsafe { std::mem::zeroed() };
    table2.dwVnum = 1093;
    let name2 = b"Demon King\0";
    table2.szName[..name2.len()].copy_from_slice(name2);
    table2.szLocaleName[..name2.len()].copy_from_slice(name2);
    table2.bLevel = 75;
    table2.dwMaxHP = 50000;
    table2.sAttackSpeed = 200;
    table2.sMovingSpeed = 200;
    table2.bType = 0; // Monster
    table2.bRank = 3; // BOSS

    let mut raw_data = Vec::new();
    let table_size = size_of::<TMobTable>();
    
    unsafe {
        let p1 = &table1 as *const TMobTable as *const u8;
        raw_data.extend_from_slice(std::slice::from_raw_parts(p1, table_size));
        
        let p2 = &table2 as *const TMobTable as *const u8;
        raw_data.extend_from_slice(std::slice::from_raw_parts(p2, table_size));
    }

    // Since we don't have a pure Rust LZO compressor in vfs_security_decrypt, 
    // we will simulate an uncompressed block for testing (lzo can copy uncompressed if configured, 
    // but our decompress_lzo expects actual LZO stream).
    // Let's create a minimal valid LZO1X block.
    // In LZO1X, an uncompressible block can be encoded as:
    // Length > 17: opcode 17+len, followed by raw data. Wait, for larger chunks we need multiple ops.
    // A simpler way is to just build an uncompressed LZO stream.
    // LZO1X uncompressed data:
    // while len > 0:
    //    chunk = min(len, 238)
    //    if chunk <= 17: opcode = chunk + 17
    //    else: 
    //       we need to encode large literal run.
    let mut lzo_compressed = Vec::new();
    let mut pos = 0;
    while pos < raw_data.len() {
        let rem = raw_data.len() - pos;
        let chunk = rem.min(238);
        if chunk <= 17 {
            lzo_compressed.push(17 + chunk as u8);
        } else {
            // chunk > 17
            // opcode: 17 + chunk? No, LZO1X literal runs > 17:
            // 17 + 0 = 17, then (chunk - 18) as bytes, etc.
            // Let's use small chunks of 17 bytes to be safe and simple.
            let chunk_small = rem.min(17);
            lzo_compressed.push(17 + chunk_small as u8);
            lzo_compressed.extend_from_slice(&raw_data[pos..pos + chunk_small]);
            pos += chunk_small;
            continue;
        }
        lzo_compressed.extend_from_slice(&raw_data[pos..pos + chunk]);
        pos += chunk;
    }
    
    // We will use TEA encryption for the test to ensure it works.
    let mut enc_payload = Vec::new();
    enc_payload.extend_from_slice(&MCOZ_FOURCC.to_le_bytes());
    enc_payload.extend_from_slice(&lzo_compressed);
    
    // pad to 8 bytes for TEA
    let pad = (8 - (enc_payload.len() % 8)) % 8;
    enc_payload.extend(vec![0u8; pad]);
    
    encrypt_tea(&mut enc_payload, &MOB_PROTO_KEY);
    
    let mut final_blob = Vec::new();
    final_blob.extend_from_slice(&MMPT_FOURCC.to_le_bytes()); // magic
    final_blob.extend_from_slice(&2u32.to_le_bytes()); // elements
    
    let lzo_header = LzoHeader {
        dwFourCC: MCOZ_FOURCC,
        dwEncryptSize: enc_payload.len() as u32,
        dwCompressedSize: lzo_compressed.len() as u32,
        dwRealSize: raw_data.len() as u32,
    };
    
    let lzo_block_size = size_of::<LzoHeader>() + enc_payload.len();
    final_blob.extend_from_slice(&(lzo_block_size as u32).to_le_bytes());
    
    unsafe {
        let hp = &lzo_header as *const LzoHeader as *const u8;
        final_blob.extend_from_slice(std::slice::from_raw_parts(hp, size_of::<LzoHeader>()));
    }
    final_blob.extend_from_slice(&enc_payload);

    final_blob
}

#[test]
fn test_mob_proto_struct_size() {
    // 64+1 + 64+1 + 4 = 134 bytes for name
    // Verify struct size matches C++ exactly
    assert_eq!(size_of::<TMobTable>(), 335);
}

#[test]
fn test_mob_proto_binary_loader() {
    let data = create_mock_mob_proto();
    let result = load_mob_proto_binary(&data);
    
    assert!(result.is_ok(), "Failed to load mob proto: {:?}", result.err());
    
    let map = result.unwrap();
    assert_eq!(map.len(), 2);
    
    let mob1 = map.get(&101).expect("Missing vnum 101");
    assert_eq!({mob1.dwVnum}, 101);
    assert_eq!({mob1.bLevel}, 1);
    assert_eq!({mob1.dwMaxHP}, 200);
    
    let name_c = mob1.szName;
    let name = String::from_utf8(name_c.iter().copied().take_while(|&c| c != 0).collect()).unwrap();
    assert_eq!(name, "Wild Dog");

    let mob2 = map.get(&1093).expect("Missing vnum 1093");
    assert_eq!({mob2.dwVnum}, 1093);
    assert_eq!({mob2.bLevel}, 75);
    assert_eq!({mob2.bRank}, 3);
    assert_eq!({mob2.dwMaxHP}, 50000);
}

#[test]
fn test_invalid_header_rejection() {
    let data = vec![0u8; 100];
    let result = load_mob_proto_binary(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid Mob proto type (not MMPT)");
}
