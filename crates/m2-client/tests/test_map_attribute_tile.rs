#[path = "../src/map_attribute_tile_parser.rs"]
mod map_attribute_tile_parser;

use map_attribute_tile_parser::*;

#[test]
fn test_attr_map_load_success() {
    let mut buffer = Vec::new();
    
    // Header
    buffer.extend_from_slice(&ATTRMAP_MAGIC.to_ne_bytes());
    buffer.extend_from_slice(&(ATTRMAP_XSIZE as u16).to_ne_bytes());
    buffer.extend_from_slice(&(ATTRMAP_YSIZE as u16).to_ne_bytes());

    // Data
    let mut data = vec![0u8; ATTRMAP_XSIZE * ATTRMAP_YSIZE];
    
    // Set some specific flags
    data[0] = ATTRIBUTE_BLOCK;
    data[1] = ATTRIBUTE_WATER;
    data[2] = ATTRIBUTE_BANPK;
    data[3] = ATTRIBUTE_BLOCK | ATTRIBUTE_WATER | ATTRIBUTE_BANPK;

    buffer.extend_from_slice(&data);

    let tile = MapAttributeTile::load_from_buffer(&buffer).unwrap();

    assert!(tile.is_block(0, 0));
    assert!(!tile.is_water(0, 0));
    assert!(!tile.is_banpk(0, 0));

    assert!(!tile.is_block(1, 0));
    assert!(tile.is_water(1, 0));
    assert!(!tile.is_banpk(1, 0));

    assert!(!tile.is_block(2, 0));
    assert!(!tile.is_water(2, 0));
    assert!(tile.is_banpk(2, 0));

    assert!(tile.is_block(3, 0));
    assert!(tile.is_water(3, 0));
    assert!(tile.is_banpk(3, 0));
}

#[test]
fn test_attr_map_load_invalid_magic() {
    let mut buffer = Vec::new();
    
    // Header
    let invalid_magic: u16 = 1234;
    buffer.extend_from_slice(&invalid_magic.to_ne_bytes());
    buffer.extend_from_slice(&(ATTRMAP_XSIZE as u16).to_ne_bytes());
    buffer.extend_from_slice(&(ATTRMAP_YSIZE as u16).to_ne_bytes());

    // Data
    let data = vec![0u8; ATTRMAP_XSIZE * ATTRMAP_YSIZE];
    buffer.extend_from_slice(&data);

    let result = MapAttributeTile::load_from_buffer(&buffer);
    assert_eq!(result.err(), Some(AttrMapError::InvalidMagicNumber(invalid_magic)));
}

#[test]
fn test_attr_map_load_invalid_size() {
    let mut buffer = Vec::new();
    
    // Header
    buffer.extend_from_slice(&ATTRMAP_MAGIC.to_ne_bytes());
    buffer.extend_from_slice(&(ATTRMAP_XSIZE as u16).to_ne_bytes());
    buffer.extend_from_slice(&(ATTRMAP_YSIZE as u16).to_ne_bytes());

    // Data (Missing 1 byte)
    let data = vec![0u8; ATTRMAP_XSIZE * ATTRMAP_YSIZE - 1];
    buffer.extend_from_slice(&data);

    let result = MapAttributeTile::load_from_buffer(&buffer);
    assert_eq!(result.err(), Some(AttrMapError::DataSizeMismatch {
        expected: ATTRMAP_XSIZE * ATTRMAP_YSIZE,
        actual: ATTRMAP_XSIZE * ATTRMAP_YSIZE - 1,
    }));
}

#[test]
fn test_attr_map_load_invalid_width() {
    let mut buffer = Vec::new();
    
    // Header
    let invalid_width: u16 = 123;
    buffer.extend_from_slice(&ATTRMAP_MAGIC.to_ne_bytes());
    buffer.extend_from_slice(&invalid_width.to_ne_bytes());
    buffer.extend_from_slice(&(ATTRMAP_YSIZE as u16).to_ne_bytes());

    // Data
    let data = vec![0u8; ATTRMAP_XSIZE * ATTRMAP_YSIZE];
    buffer.extend_from_slice(&data);

    let result = MapAttributeTile::load_from_buffer(&buffer);
    assert_eq!(result.err(), Some(AttrMapError::InvalidWidth(invalid_width)));
}

#[test]
fn test_attr_map_load_invalid_height() {
    let mut buffer = Vec::new();
    
    // Header
    let invalid_height: u16 = 123;
    buffer.extend_from_slice(&ATTRMAP_MAGIC.to_ne_bytes());
    buffer.extend_from_slice(&(ATTRMAP_XSIZE as u16).to_ne_bytes());
    buffer.extend_from_slice(&invalid_height.to_ne_bytes());

    // Data
    let data = vec![0u8; ATTRMAP_XSIZE * ATTRMAP_YSIZE];
    buffer.extend_from_slice(&data);

    let result = MapAttributeTile::load_from_buffer(&buffer);
    assert_eq!(result.err(), Some(AttrMapError::InvalidHeight(invalid_height)));
}

#[test]
fn test_attr_map_load_too_small() {
    let mut buffer = Vec::new();
    
    // Header (Incomplete)
    buffer.extend_from_slice(&ATTRMAP_MAGIC.to_ne_bytes());

    let result = MapAttributeTile::load_from_buffer(&buffer);
    assert_eq!(result.err(), Some(AttrMapError::InvalidFileSize));
}
