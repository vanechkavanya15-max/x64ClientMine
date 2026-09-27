use m2_client::py_bridge_minimap::*;
use std::sync::Mutex;
use std::sync::OnceLock;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn get_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap()
}

#[test]
fn test_minimap_initial_state() {
    let _guard = get_lock();
    reset_minimap_state();
    
    assert_eq!(m2_minimap_get_scale(), 1.0);
    assert_eq!(m2_minimap_is_atlas_open(), false);
    assert_eq!(m2_minimap_is_visible(), true);
    
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = 0.0;
    assert_eq!(m2_minimap_get_waypoint(0, &mut x, &mut y, &mut z), false);
}

#[test]
fn test_minimap_set_scale() {
    let _guard = get_lock();
    reset_minimap_state();
    
    assert!(m2_minimap_set_scale(2.5));
    assert_eq!(m2_minimap_get_scale(), 2.5);
    
    // Test negative clamp if implemented, or just bounds
    assert!(m2_minimap_set_scale(-1.0));
    assert_eq!(m2_minimap_get_scale(), 0.0);
}

#[test]
fn test_minimap_atlas_window_toggle() {
    let _guard = get_lock();
    reset_minimap_state();
    
    assert!(m2_minimap_open_atlas_window());
    assert_eq!(m2_minimap_is_atlas_open(), true);
    
    assert!(m2_minimap_close_atlas_window());
    assert_eq!(m2_minimap_is_atlas_open(), false);
}

#[test]
fn test_minimap_visibility_toggle() {
    let _guard = get_lock();
    reset_minimap_state();
    
    assert!(m2_minimap_hide_minimap());
    assert_eq!(m2_minimap_is_visible(), false);
    
    assert!(m2_minimap_show_minimap());
    assert_eq!(m2_minimap_is_visible(), true);
}

#[test]
fn test_minimap_waypoints() {
    let _guard = get_lock();
    reset_minimap_state();
    
    assert!(m2_minimap_add_waypoint(100.0, 200.0, 300.0));
    assert!(m2_minimap_add_waypoint(400.0, 500.0, 600.0));
    
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = 0.0;
    
    assert!(m2_minimap_get_waypoint(0, &mut x, &mut y, &mut z));
    assert_eq!(x, 100.0);
    assert_eq!(y, 200.0);
    assert_eq!(z, 300.0);
    
    assert!(m2_minimap_get_waypoint(1, &mut x, &mut y, &mut z));
    assert_eq!(x, 400.0);
    assert_eq!(y, 500.0);
    assert_eq!(z, 600.0);
    
    // Out of bounds
    assert_eq!(m2_minimap_get_waypoint(2, &mut x, &mut y, &mut z), false);
    
    // Clear
    assert!(m2_minimap_clear_waypoints());
    assert_eq!(m2_minimap_get_waypoint(0, &mut x, &mut y, &mut z), false);
}

#[test]
fn test_minimap_waypoint_atlas_pos() {
    let _guard = get_lock();
    reset_minimap_state();
    
    assert!(m2_minimap_add_waypoint(100.0, 200.0, 300.0));
    
    let mut atlas_x: f32 = 0.0;
    let mut atlas_y: f32 = 0.0;
    
    assert!(m2_minimap_get_waypoint_atlas_pos(0, 1024.0, 1024.0, &mut atlas_x, &mut atlas_y));
    
    // Expected atlas position given scale (0.5 uniformly based on image size 512.0 and map size 1024.0)
    // 100 * 0.5 = 50, 200 * 0.5 = 100
    assert_eq!(atlas_x, 50.0);
    assert_eq!(atlas_y, 100.0);
}
