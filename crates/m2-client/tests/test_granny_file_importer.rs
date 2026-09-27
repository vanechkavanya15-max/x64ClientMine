use m2_client::granny_file_importer::{parse_gr2_memory, extract_bone_names, Gr2Header, Gr2ModelInfo, GRANNY_MAGIC_BYTES};
use std::mem::size_of;

#[test]
fn test_parse_valid_gr2_header() {
    let mut data = Vec::new();
    // 16 bytes magic
    data.extend_from_slice(&GRANNY_MAGIC_BYTES);
    // 4 bytes format version = 1
    data.extend_from_slice(&1u32.to_le_bytes());
    // 4 bytes total size = 47 (16 + 4 + 4 + name_bytes (11 chars + 1 null = 11? No, test_model is 10 chars + \0 = 11 bytes) + 12 (counts))
    // len("test_model") == 10. + 1 = 11. 24 + 11 + 12 = 47.
    data.extend_from_slice(&47u32.to_le_bytes());
    
    // name "test_model\0" (10 chars + null = 11 bytes)
    data.extend_from_slice(b"test_model\0");
    
    // 4 bytes bone_count = 2
    data.extend_from_slice(&2u32.to_le_bytes());
    // 4 bytes mesh_count = 3
    data.extend_from_slice(&3u32.to_le_bytes());
    // 4 bytes animation_count = 1
    data.extend_from_slice(&1u32.to_le_bytes());
    
    // check exact match
    assert_eq!(data.len(), 47);

    let result = parse_gr2_memory(&data).unwrap();
    assert_eq!(result.name, "test_model");
    assert_eq!(result.bone_count, 2);
    assert_eq!(result.mesh_count, 3);
    assert_eq!(result.animation_count, 1);
}

#[test]
fn test_invalid_magic_bytes() {
    let mut data = Vec::new();
    let bad_magic = b"WRONG_MAGIC_VALS";
    data.extend_from_slice(bad_magic);
    data.extend_from_slice(&1u32.to_le_bytes());
    data.extend_from_slice(&24u32.to_le_bytes());

    let result = parse_gr2_memory(&data);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid magic bytes"));
}

#[test]
fn test_buffer_too_small_for_header() {
    let data = vec![0u8; 20]; // Less than 24 bytes
    let result = parse_gr2_memory(&data);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Buffer too small for Gr2Header"));
}

#[test]
fn test_total_size_mismatch() {
    let mut data = Vec::new();
    data.extend_from_slice(&GRANNY_MAGIC_BYTES);
    data.extend_from_slice(&1u32.to_le_bytes());
    // Declare total size as 100, but only provide 36 bytes
    data.extend_from_slice(&100u32.to_le_bytes());
    
    data.extend_from_slice(b"name\0");
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());

    assert_eq!(data.len(), 41);

    let result = parse_gr2_memory(&data);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("less than total_size"));
}

#[test]
fn test_extract_bone_names_success() {
    let mut data = Vec::new();
    data.extend_from_slice(&GRANNY_MAGIC_BYTES);
    data.extend_from_slice(&1u32.to_le_bytes());
    // Total size calculated: 24 (header) + 6 (name) + 12 (counts) + 12 (bones) = 54
    data.extend_from_slice(&54u32.to_le_bytes());
    
    data.extend_from_slice(b"model\0");
    
    data.extend_from_slice(&2u32.to_le_bytes()); // bone_count = 2
    data.extend_from_slice(&0u32.to_le_bytes()); // mesh_count
    data.extend_from_slice(&0u32.to_le_bytes()); // anim_count
    
    // Bones
    data.extend_from_slice(b"Bip01\0"); // 6 bytes
    data.extend_from_slice(b"Head\0");  // 5 bytes
    data.push(0); // padding to match 54 total optionally if needed, let's keep exact
    
    // Bip01\0 is 6 bytes. Head\0 is 5 bytes. total bytes added: 11. 
    // total size = 24 + 6 + 12 + 11 = 53
    let actual_size = 53u32;
    
    let mut corrected_data = Vec::new();
    corrected_data.extend_from_slice(&GRANNY_MAGIC_BYTES);
    corrected_data.extend_from_slice(&1u32.to_le_bytes());
    corrected_data.extend_from_slice(&actual_size.to_le_bytes());
    corrected_data.extend_from_slice(b"model\0");
    corrected_data.extend_from_slice(&2u32.to_le_bytes());
    corrected_data.extend_from_slice(&0u32.to_le_bytes());
    corrected_data.extend_from_slice(&0u32.to_le_bytes());
    corrected_data.extend_from_slice(b"Bip01\0");
    corrected_data.extend_from_slice(b"Head\0");
    
    assert_eq!(corrected_data.len(), 53);
    
    let bones = extract_bone_names(&corrected_data);
    assert_eq!(bones.len(), 2);
    assert_eq!(bones[0], "Bip01");
    assert_eq!(bones[1], "Head");
}

#[test]
fn test_extract_bone_names_truncated() {
    let mut data = Vec::new();
    data.extend_from_slice(&GRANNY_MAGIC_BYTES);
    data.extend_from_slice(&1u32.to_le_bytes());
    data.extend_from_slice(&55u32.to_le_bytes()); 
    data.extend_from_slice(b"model\0");
    data.extend_from_slice(&2u32.to_le_bytes()); // Claims 2 bones
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(&0u32.to_le_bytes());
    data.extend_from_slice(b"Bip01\0");
    // Missing the second bone completely but data buffer is artificially grown to pass total_size check
    data.extend_from_slice(&[0, 0, 0]); 
    // Wait, the string won't be null-terminated at a valid boundary or missing.
    // Here it just has zeros which parses as empty strings, but let's test missing null terminator.
    data.extend_from_slice(b"Hea"); // truncated "Head\0"
    
    // total size = 24 + 6 + 12 + 6 + 3 + 3 = 54. 
    
    let mut final_data = Vec::new();
    final_data.extend_from_slice(&GRANNY_MAGIC_BYTES);
    final_data.extend_from_slice(&1u32.to_le_bytes());
    let sz = (24 + 6 + 12 + 6 + 3) as u32;
    final_data.extend_from_slice(&sz.to_le_bytes());
    final_data.extend_from_slice(b"model\0");
    final_data.extend_from_slice(&2u32.to_le_bytes()); 
    final_data.extend_from_slice(&0u32.to_le_bytes());
    final_data.extend_from_slice(&0u32.to_le_bytes());
    final_data.extend_from_slice(b"Bip01\0");
    final_data.extend_from_slice(b"Hea"); // No null terminator
    
    let bones = extract_bone_names(&final_data);
    // Should extract first one safely, then fail gracefully on second
    assert_eq!(bones.len(), 1);
    assert_eq!(bones[0], "Bip01");
}
