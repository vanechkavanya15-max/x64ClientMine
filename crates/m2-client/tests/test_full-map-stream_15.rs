#[path = "../src/map_outdoor_system_validator.rs"]
mod map_outdoor_system_validator;

#[path = "../src/terrain_attr_collision_v2.rs"]
mod terrain_attr_collision_v2;

#[path = "../src/terrain_bicubic_height_v2.rs"]
mod terrain_bicubic_height_v2;

use map_outdoor_system_validator::{MapOutdoorSystemValidator, SECTOR_SIZE_CM};
use terrain_attr_collision_v2::EXPECTED_GRID_LEN;
use terrain_bicubic_height_v2::HEIGHTMAP_RAW_SIZE;

fn create_dummy_height_data() -> Vec<u16> {
    vec![10; HEIGHTMAP_RAW_SIZE]
}

fn create_dummy_attr_data(fill: u8) -> Vec<u8> {
    vec![fill; EXPECTED_GRID_LEN]
}

#[test]
fn test_validator_initialization() {
    let validator = MapOutdoorSystemValidator::new(2);
    assert_eq!(validator.streaming_radius, 2);
    assert!(validator.active_sectors.is_empty());
}

#[test]
fn test_streaming_radius_unloads_old_sectors() {
    let mut validator = MapOutdoorSystemValidator::new(1);
    
    // Add sectors
    validator.add_sector_data(0, 0, create_dummy_height_data(), create_dummy_attr_data(0)).unwrap();
    validator.add_sector_data(1, 1, create_dummy_height_data(), create_dummy_attr_data(0)).unwrap();
    validator.add_sector_data(2, 2, create_dummy_height_data(), create_dummy_attr_data(0)).unwrap();
    
    assert_eq!(validator.active_sectors.len(), 3);
    
    // Player is at sector (0, 0). With radius 1, sector (2, 2) should be unloaded.
    validator.update_streaming_center(0, 0);
    
    assert_eq!(validator.active_sectors.len(), 2);
    assert!(validator.active_sectors.contains_key(&(0, 0)));
    assert!(validator.active_sectors.contains_key(&(1, 1)));
    assert!(!validator.active_sectors.contains_key(&(2, 2)));
}

#[test]
fn test_add_sector_data_success() {
    let mut validator = MapOutdoorSystemValidator::new(1);
    
    let result = validator.add_sector_data(0, 0, create_dummy_height_data(), create_dummy_attr_data(0));
    assert!(result.is_ok());
    assert_eq!(validator.active_sectors.len(), 1);
    
    // Invalid attr data length
    let invalid_result = validator.add_sector_data(1, 0, create_dummy_height_data(), vec![0; 10]);
    assert!(invalid_result.is_err());
    assert_eq!(validator.active_sectors.len(), 1);
}

#[test]
fn test_is_walkable_returns_correct_value() {
    let mut validator = MapOutdoorSystemValidator::new(1);
    
    // Sector (0, 0) is walkable (FLAG = 0)
    validator.add_sector_data(0, 0, create_dummy_height_data(), create_dummy_attr_data(0)).unwrap();
    
    // Sector (1, 0) is blocked (FLAG = 1 -> FLAG_BLOCKED)
    validator.add_sector_data(1, 0, create_dummy_height_data(), create_dummy_attr_data(1)).unwrap();
    
    // Within sector (0, 0)
    assert!(validator.is_walkable(100.0, 100.0));
    
    // Within sector (1, 0)
    let gx_1 = SECTOR_SIZE_CM as f32 + 100.0;
    assert!(!validator.is_walkable(gx_1, 100.0));
}

#[test]
fn test_get_height_returns_correct_value() {
    let mut validator = MapOutdoorSystemValidator::new(1);
    
    // Raw height 10 * 1.0 (scale) = 10.0
    validator.add_sector_data(0, 0, create_dummy_height_data(), create_dummy_attr_data(0)).unwrap();
    
    let height = validator.get_height(100.0, 100.0);
    assert!(height.is_some());
    assert_eq!(height.unwrap(), 10.0);
}

#[test]
fn test_out_of_bounds_queries_return_safe_defaults() {
    let validator = MapOutdoorSystemValidator::new(1);
    
    // No sectors loaded
    assert!(!validator.is_walkable(100.0, 100.0));
    assert!(validator.get_height(100.0, 100.0).is_none());
}
