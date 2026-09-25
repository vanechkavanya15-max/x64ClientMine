use m2_client_terrain::attr_map::AttrMap;

#[test]
fn test_attr_map_new_is_not_ready() {
    let instance = AttrMap::new();
    assert!(!instance.is_ready());
}

#[test]
fn test_parse_valid_data() {
    // 2x2 map, header: width=2 (LE), height=2 (LE)
    // data:
    // [0, 0] = BLOCK (1)
    // [1, 0] = WATER (2)
    // [0, 1] = WARP (4)
    // [1, 1] = SAFE_ZONE (8)
    let buffer: [u8; 8] = [
        2, 0, // width = 2
        2, 0, // height = 2
        1, 2, 4, 8, // data
    ];

    let result = AttrMap::parse(&buffer);
    assert!(result.is_ok());

    let attr_map = result.unwrap();
    assert!(attr_map.is_ready());
    assert_eq!(attr_map.width, 2);
    assert_eq!(attr_map.height, 2);
    assert_eq!(attr_map.data.len(), 4);

    assert!(attr_map.is_block(0, 0));
    assert!(!attr_map.is_water(0, 0));
    
    assert!(attr_map.is_water(1, 0));
    assert!(!attr_map.is_block(1, 0));
    
    assert!(attr_map.is_warp(0, 1));
    assert!(!attr_map.is_safe_zone(0, 1));
    
    assert!(attr_map.is_safe_zone(1, 1));
    assert!(!attr_map.is_warp(1, 1));
}

#[test]
fn test_parse_short_buffer() {
    let buffer: [u8; 3] = [2, 0, 2];
    let result = AttrMap::parse(&buffer);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Bufor jest zbyt krotki by zawierac naglowek mapy atrybutow");
}

#[test]
fn test_parse_incomplete_data() {
    // Header says 2x2 (4 bytes of data), but only 2 bytes provided
    let buffer: [u8; 6] = [
        2, 0, // width = 2
        2, 0, // height = 2
        1, 2, // incomplete data
    ];
    let result = AttrMap::parse(&buffer);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Bufor jest mniejszy niz zadeklarowany rozmiar mapy");
}

#[test]
fn test_out_of_bounds_access() {
    let buffer: [u8; 8] = [
        2, 0, // width = 2
        2, 0, // height = 2
        1, 2, 4, 8, // data
    ];

    let attr_map = AttrMap::parse(&buffer).unwrap();

    // Accessing out of bounds coordinates should return false safely without panic
    assert!(!attr_map.is_block(2, 0));
    assert!(!attr_map.is_water(0, 2));
    assert!(!attr_map.is_warp(2, 2));
    assert!(!attr_map.is_safe_zone(100, 100));
}
