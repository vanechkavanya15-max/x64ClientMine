use m2_client::terrain_raw_loader::{RawHeightMap, HEIGHTMAP_RAW_BYTES, HEIGHTMAP_RAW_XSIZE};

#[test]
fn test_reject_invalid_size_too_short() {
    let data = vec![0u8; HEIGHTMAP_RAW_BYTES - 1];
    let result = RawHeightMap::from_bytes(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid raw heightmap data size");
}

#[test]
fn test_reject_invalid_size_too_long() {
    let data = vec![0u8; HEIGHTMAP_RAW_BYTES + 1];
    let result = RawHeightMap::from_bytes(&data);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid raw heightmap data size");
}

#[test]
fn test_parse_valid_buffer() {
    let mut data = vec![0u8; HEIGHTMAP_RAW_BYTES];
    // Set some known values
    // Index 0: value 100
    data[0] = 100;
    data[1] = 0;
    
    // Index 1: value 258 (1 * 256 + 2) => [2, 1]
    data[2] = 2;
    data[3] = 1;

    // Index HEIGHTMAP_RAW_XSIZE (row 1, col 0): value 500 => [244, 1]
    let row1_col0_idx = HEIGHTMAP_RAW_XSIZE * 2;
    data[row1_col0_idx] = 244;
    data[row1_col0_idx + 1] = 1;

    let result = RawHeightMap::from_bytes(&data);
    assert!(result.is_ok());
    let map = result.unwrap();

    assert_eq!(map.get_height_raw(0, 0), Some(100));
    assert_eq!(map.get_height_raw(1, 0), Some(258));
    assert_eq!(map.get_height_raw(0, 1), Some(500));
    assert_eq!(map.get_height_raw(130, 130), Some(0));
}

#[test]
fn test_min_max_height_computation() {
    let mut data = vec![0u8; HEIGHTMAP_RAW_BYTES];
    // Fill with base value 50
    for i in 0..HEIGHTMAP_RAW_BYTES / 2 {
        data[i * 2] = 50;
        data[i * 2 + 1] = 0;
    }

    // Set a minimum value of 10
    data[10 * 2] = 10;
    data[10 * 2 + 1] = 0;

    // Set a maximum value of 1000
    data[20 * 2] = (1000 % 256) as u8;
    data[20 * 2 + 1] = (1000 / 256) as u8;

    let map = RawHeightMap::from_bytes(&data).unwrap();
    let (min, max) = map.min_max_height();
    assert_eq!(min, 10);
    assert_eq!(max, 1000);
}

#[test]
fn test_out_of_bounds_coordinates() {
    let data = vec![0u8; HEIGHTMAP_RAW_BYTES];
    let map = RawHeightMap::from_bytes(&data).unwrap();

    // Valid max
    assert_eq!(map.get_height_raw(130, 130), Some(0));

    // Invalid x
    assert_eq!(map.get_height_raw(131, 0), None);
    assert_eq!(map.get_height_raw(200, 50), None);

    // Invalid y
    assert_eq!(map.get_height_raw(0, 131), None);
    assert_eq!(map.get_height_raw(50, 200), None);

    // Invalid both
    assert_eq!(map.get_height_raw(131, 131), None);
}
