use m2_client::terrain_attr_parser::AttrMap;

#[test]
fn test_valid_attr_map_parsing() {
    let mut data = vec![0u8; 65542];
    
    // Magic: 2634
    data[0] = 74; // 0x4A
    data[1] = 10; // 0x0A
    
    // Width: 256
    data[2] = 0;
    data[3] = 1;
    
    // Height: 256
    data[4] = 0;
    data[5] = 1;
    
    // Attribute at (0, 0)
    data[6] = 1; // BLOCK
    
    // Attribute at (10, 10) -> index = 6 + 10 * 256 + 10 = 2576
    data[2576] = 2; // WATER
    
    // Attribute at (255, 255) -> index = 6 + 255 * 256 + 255 = 65541
    data[65541] = 4; // BANPK

    let map_result = AttrMap::from_bytes(&data);
    assert!(map_result.is_ok());
    let map = map_result.unwrap();

    assert!(map.is_blocked(0, 0));
    assert!(!map.is_water(0, 0));
    assert!(!map.is_safe_zone(0, 0));

    assert!(!map.is_blocked(10, 10));
    assert!(map.is_water(10, 10));
    assert!(!map.is_safe_zone(10, 10));

    assert!(!map.is_blocked(255, 255));
    assert!(!map.is_water(255, 255));
    assert!(map.is_safe_zone(255, 255));
    
    // Out of bounds should return false/0 and not panic
    assert!(!map.is_blocked(256, 256));
    assert_eq!(map.get_attr(256, 256), 0);
}

#[test]
fn test_invalid_magic_number() {
    let mut data = vec![0u8; 65542];
    
    // Magic: 1234
    data[0] = 210; // 0xD2
    data[1] = 4; // 0x04
    
    // Width: 256
    data[2] = 0;
    data[3] = 1;
    
    // Height: 256
    data[4] = 0;
    data[5] = 1;
    
    let map_result = AttrMap::from_bytes(&data);
    assert!(map_result.is_err());
    assert_eq!(map_result.err().unwrap(), "MAGIC NUMBER ERROR");
}

#[test]
fn test_invalid_width() {
    let mut data = vec![0u8; 65542];
    
    // Magic: 2634
    data[0] = 74;
    data[1] = 10;
    
    // Width: 128
    data[2] = 128;
    data[3] = 0;
    
    // Height: 256
    data[4] = 0;
    data[5] = 1;
    
    let map_result = AttrMap::from_bytes(&data);
    assert!(map_result.is_err());
    assert_eq!(map_result.err().unwrap(), "WIDTH ERROR");
}

#[test]
fn test_invalid_height() {
    let mut data = vec![0u8; 65542];
    
    // Magic: 2634
    data[0] = 74;
    data[1] = 10;
    
    // Width: 256
    data[2] = 0;
    data[3] = 1;
    
    // Height: 128
    data[4] = 128;
    data[5] = 0;
    
    let map_result = AttrMap::from_bytes(&data);
    assert!(map_result.is_err());
    assert_eq!(map_result.err().unwrap(), "HEIGHT ERROR");
}

#[test]
fn test_invalid_file_size_small() {
    let data = vec![0u8; 5]; // smaller than header
    let map_result = AttrMap::from_bytes(&data);
    assert!(map_result.is_err());
    assert_eq!(map_result.err().unwrap(), "FILE SIZE ERROR");
}

#[test]
fn test_invalid_data_size_short() {
    let mut data = vec![0u8; 10000]; // enough for header, but not enough for map
    
    // Magic: 2634
    data[0] = 74;
    data[1] = 10;
    
    // Width: 256
    data[2] = 0;
    data[3] = 1;
    
    // Height: 256
    data[4] = 0;
    data[5] = 1;
    
    let map_result = AttrMap::from_bytes(&data);
    assert!(map_result.is_err());
    assert_eq!(map_result.err().unwrap(), "FILE DATA SIZE ERROR");
}

#[test]
fn test_invalid_data_size_large() {
    let mut data = vec![0u8; 70000]; // too much data
    
    // Magic: 2634
    data[0] = 74;
    data[1] = 10;
    
    // Width: 256
    data[2] = 0;
    data[3] = 1;
    
    // Height: 256
    data[4] = 0;
    data[5] = 1;
    
    let map_result = AttrMap::from_bytes(&data);
    assert!(map_result.is_err());
    assert_eq!(map_result.err().unwrap(), "FILE DATA SIZE ERROR");
}
