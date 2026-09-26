use m2_client::terrain_water_map::WaterMap;

fn create_valid_header(layer_count: u8) -> Vec<u8> {
    let mut data = Vec::new();
    let magic: u16 = 5426;
    let width: u16 = 128;
    let height: u16 = 128;
    
    data.extend_from_slice(&magic.to_le_bytes());
    data.extend_from_slice(&width.to_le_bytes());
    data.extend_from_slice(&height.to_le_bytes());
    data.push(layer_count);
    
    data
}

#[test]
fn test_invalid_magic() {
    let mut data = create_valid_header(0);
    // Overwrite magic
    let wrong_magic: u16 = 9999;
    data[0..2].copy_from_slice(&wrong_magic.to_le_bytes());
    data.extend_from_slice(&[0xFF; 128 * 128]);
    
    let result = WaterMap::from_bytes(&data);
    assert_eq!(result.err(), Some("Invalid magic number"));
}

#[test]
fn test_invalid_dimensions() {
    let mut data = Vec::new();
    let magic: u16 = 5426;
    let width: u16 = 127;
    let height: u16 = 128;
    
    data.extend_from_slice(&magic.to_le_bytes());
    data.extend_from_slice(&width.to_le_bytes());
    data.extend_from_slice(&height.to_le_bytes());
    data.push(0);
    data.extend_from_slice(&[0xFF; 128 * 128]);
    
    let result = WaterMap::from_bytes(&data);
    assert_eq!(result.err(), Some("Invalid dimensions"));
}

#[test]
fn test_data_too_small_for_header() {
    let data = vec![0u8; 5];
    let result = WaterMap::from_bytes(&data);
    assert_eq!(result.err(), Some("Data too small to contain header"));
}

#[test]
fn test_data_too_small_for_layers() {
    let data = create_valid_header(5);
    // Data has header, but no layer data
    let result = WaterMap::from_bytes(&data);
    assert_eq!(result.err(), Some("Data too small to contain layers and grid"));
}

#[test]
fn test_data_too_small_for_grid() {
    let mut data = create_valid_header(1);
    let layer_val: u16 = 100;
    data.extend_from_slice(&layer_val.to_le_bytes());
    // Only 10 bytes of grid
    data.extend_from_slice(&[0xFF; 10]);
    
    let result = WaterMap::from_bytes(&data);
    assert_eq!(result.err(), Some("Data too small to contain layers and grid"));
}

#[test]
fn test_valid_no_water() {
    let mut data = create_valid_header(0);
    data.extend_from_slice(&[0xFF; 128 * 128]);
    
    let map = WaterMap::from_bytes(&data).expect("Should parse successfully");
    
    assert_eq!(map.has_water(0, 0), false);
    assert_eq!(map.has_water(127, 127), false);
    assert_eq!(map.get_water_height(0, 0), None);
    assert_eq!(map.get_water_height(127, 127), None);
}

#[test]
fn test_valid_with_water_layers() {
    let mut data = create_valid_header(3);
    
    let layer1: u16 = 150;
    let layer2: u16 = 300;
    let layer3: u16 = 450;
    
    data.extend_from_slice(&layer1.to_le_bytes());
    data.extend_from_slice(&layer2.to_le_bytes());
    data.extend_from_slice(&layer3.to_le_bytes());
    
    let mut grid = vec![0xFF; 128 * 128];
    // Set some water spots
    grid[0] = 0; // Layer 1 (index 0)
    grid[1] = 1; // Layer 2 (index 1)
    grid[128] = 2; // Layer 3 (index 2)
    grid[128 * 127 + 127] = 1; // Layer 2 at bottom right
    
    // Set invalid layer index
    grid[10] = 10;
    
    data.extend_from_slice(&grid);
    
    let map = WaterMap::from_bytes(&data).expect("Should parse successfully");
    
    assert_eq!(map.has_water(0, 0), true);
    assert_eq!(map.has_water(1, 0), true);
    assert_eq!(map.has_water(0, 1), true);
    assert_eq!(map.has_water(127, 127), true);
    assert_eq!(map.has_water(2, 0), false);
    
    assert_eq!(map.get_water_height(0, 0), Some(150.0));
    assert_eq!(map.get_water_height(1, 0), Some(300.0));
    assert_eq!(map.get_water_height(0, 1), Some(450.0));
    assert_eq!(map.get_water_height(127, 127), Some(300.0));
    assert_eq!(map.get_water_height(2, 0), None);
    
    // Invalid layer index case
    assert_eq!(map.has_water(10, 0), true); // Still has water (not 0xFF)
    assert_eq!(map.get_water_height(10, 0), None); // But layer is out of bounds
}

#[test]
fn test_out_of_bounds_coordinates() {
    let mut data = create_valid_header(0);
    data.extend_from_slice(&[0xFF; 128 * 128]);
    
    let map = WaterMap::from_bytes(&data).expect("Should parse successfully");
    
    assert_eq!(map.has_water(128, 0), false);
    assert_eq!(map.has_water(0, 128), false);
    assert_eq!(map.has_water(255, 255), false);
    
    assert_eq!(map.get_water_height(128, 0), None);
    assert_eq!(map.get_water_height(0, 128), None);
    assert_eq!(map.get_water_height(255, 255), None);
}

#[test]
fn test_valid_cpp_water_layout() {
    // Official Metin2 C++ layout: Header (7B) -> Grid (16384B) -> Height Layers (2B each)
    let mut data = create_valid_header(2);
    
    let mut grid = vec![0xFF; 128 * 128];
    grid[0] = 0; // Layer 0
    grid[1] = 1; // Layer 1
    data.extend_from_slice(&grid);

    let layer0: u16 = 250;
    let layer1: u16 = 500;
    data.extend_from_slice(&layer0.to_le_bytes());
    data.extend_from_slice(&layer1.to_le_bytes());

    let map = WaterMap::from_bytes(&data).expect("Should parse official Metin2 C++ watermap");
    assert!(map.has_water(0, 0));
    assert!(map.has_water(1, 0));
    assert!(!map.has_water(2, 0));

    assert_eq!(map.get_water_height(0, 0), Some(250.0));
    assert_eq!(map.get_water_height(1, 0), Some(500.0));
    assert_eq!(map.get_water_height(2, 0), None);
}
