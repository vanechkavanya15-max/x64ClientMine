#[path = "../src/game_fishing_water_detection.rs"]
#[allow(dead_code)]
mod game_fishing_water_detection;
use game_fishing_water_detection::{Position, get_rotated_position, get_fishing_rotation, FishingMinigame, FISHING_DISTANCE, LineSegment, ATTRIBUTE_WATER};

#[test]
fn test_get_rotated_position() {
    let center = Position { x: 100.0, y: 100.0 };
    let p = get_rotated_position(center, 100.0, 90.0);
    // sin(90) = 1, cos(90) = 0
    assert!((p.x - 200.0).abs() < 0.001);
    assert!((p.y - 100.0).abs() < 0.001);

    let p2 = get_rotated_position(center, 100.0, 0.0);
    // sin(0) = 0, cos(0) = 1
    assert!((p2.x - 100.0).abs() < 0.001);
    assert!((p2.y - 200.0).abs() < 0.001);
}

#[test]
fn test_get_fishing_rotation_found_right() {
    // We check if water is found at exactly character_rotation + 20.0
    let check_water_fn = |x: f32, y: f32| -> bool {
        let expected_p = get_rotated_position(Position { x: 100.0, y: 100.0 }, FISHING_DISTANCE, 50.0 + 20.0);
        (x - expected_p.x).abs() < 0.1 && (y - expected_p.y).abs() < 0.1
    };

    let rot = get_fishing_rotation(100.0, 100.0, 50.0, &check_water_fn);
    assert_eq!(rot, Some(70.0));
}

#[test]
fn test_get_fishing_rotation_found_left() {
    // Water is found at exactly char_rot - 30.0
    let check_water_fn = |x: f32, y: f32| -> bool {
        let expected_p = get_rotated_position(Position { x: 100.0, y: 100.0 }, FISHING_DISTANCE, 50.0 - 30.0);
        (x - expected_p.x).abs() < 0.1 && (y - expected_p.y).abs() < 0.1
    };

    let rot = get_fishing_rotation(100.0, 100.0, 50.0, &check_water_fn);
    assert_eq!(rot, Some(20.0));
}

#[test]
fn test_get_fishing_rotation_not_found() {
    let check_water_fn = |_x: f32, _y: f32| -> bool {
        false
    };
    let rot = get_fishing_rotation(100.0, 100.0, 50.0, &check_water_fn);
    assert_eq!(rot, None);
}

#[test]
fn test_fishing_minigame_success() {
    let mut minigame = FishingMinigame::new(3, 5000);
    assert!(minigame.is_active);
    assert!(!minigame.is_success);

    minigame.register_hit();
    minigame.register_hit();
    assert!(minigame.is_active);

    minigame.register_hit(); // 3rd hit
    assert!(!minigame.is_active);
    assert!(minigame.is_success);
}

#[test]
fn test_fishing_minigame_timeout() {
    let mut minigame = FishingMinigame::new(3, 5000);
    minigame.register_hit();
    
    minigame.update(3000);
    assert!(minigame.is_active);
    
    minigame.update(2000); // reaches 5000ms
    assert!(!minigame.is_active);
    assert!(!minigame.is_success); // failed due to timeout
}

#[test]
fn test_raycast_line_segment() {
    let segment = LineSegment {
        start: Position { x: 0.0, y: 0.0 },
        end: Position { x: 100.0, y: 0.0 },
    };
    
    // hit if x is exactly 50
    let block_fn = |x: f32, _y: f32| -> bool {
        (x - 50.0).abs() < 1.0
    };
    
    assert!(segment.raycast_hit(&block_fn, 5.0));
    
    // no hit
    let block_fn2 = |x: f32, _y: f32| -> bool {
        (x - 150.0).abs() < 1.0
    };
    
    assert!(!segment.raycast_hit(&block_fn2, 5.0));
}

#[test]
fn test_attribute_water() {
    assert_eq!(ATTRIBUTE_WATER, 1 << 1);
}

extern "C" fn mock_check_water_cb(x: f32, y: f32) -> bool {
    let expected_p = get_rotated_position(Position { x: 100.0, y: 100.0 }, FISHING_DISTANCE, 70.0);
    (x - expected_p.x).abs() < 0.1 && (y - expected_p.y).abs() < 0.1
}

extern "C" fn mock_check_block_cb(x: f32, _y: f32) -> bool {
    (x - 50.0).abs() < 1.0
}

#[test]
fn test_ffi_get_rotation() {
    let rot = game_fishing_water_detection::m2_fishing_get_rotation_ffi(
        100.0,
        100.0,
        50.0,
        mock_check_water_cb,
    );
    assert_eq!(rot, 70.0);
}

#[test]
fn test_ffi_raycast_hit() {
    let hit = game_fishing_water_detection::m2_fishing_raycast_hit_ffi(
        0.0,
        0.0,
        100.0,
        0.0,
        5.0,
        mock_check_block_cb,
    );
    assert!(hit);
}

#[test]
fn test_ffi_minigame_lifecycle() {
    unsafe {
        let ptr = game_fishing_water_detection::m2_fishing_minigame_new_ffi(2, 5000);
        assert!(!ptr.is_null());

        assert!(game_fishing_water_detection::m2_fishing_minigame_is_active_ffi(ptr));
        assert!(!game_fishing_water_detection::m2_fishing_minigame_is_success_ffi(ptr));

        game_fishing_water_detection::m2_fishing_minigame_register_hit_ffi(ptr);
        assert!(game_fishing_water_detection::m2_fishing_minigame_is_active_ffi(ptr));

        game_fishing_water_detection::m2_fishing_minigame_update_ffi(ptr, 1000);
        
        game_fishing_water_detection::m2_fishing_minigame_register_hit_ffi(ptr);
        assert!(!game_fishing_water_detection::m2_fishing_minigame_is_active_ffi(ptr));
        assert!(game_fishing_water_detection::m2_fishing_minigame_is_success_ffi(ptr));

        game_fishing_water_detection::m2_fishing_minigame_free_ffi(ptr);
    }
}
