use m2_client::terrain_raycast::terrain_raycast_2d;
use std::sync::OnceLock;
use std::sync::Mutex;

static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn get_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|e| e.into_inner())
}

#[test]
fn test_clean_path() {
    let _l = get_lock();
    
    let is_blocked = |_, _| false;

    let result = terrain_raycast_2d(0, 0, 5, 0, &is_blocked);
    assert_eq!(result.hit, false);
    assert_eq!(result.distance, 5.0);
    assert_eq!(result.traversed_cells, vec![(0,0), (1,0), (2,0), (3,0), (4,0), (5,0)]);
}

#[test]
fn test_hit_midway() {
    let _l = get_lock();
    
    let is_blocked = |x, y| {
        x == 3 && y == 0
    };

    let result = terrain_raycast_2d(0, 0, 5, 0, &is_blocked);
    assert_eq!(result.hit, true);
    assert_eq!(result.hit_point, (3, 0));
    assert_eq!(result.distance, 3.0);
    assert_eq!(result.traversed_cells, vec![(0,0), (1,0), (2,0), (3,0)]);
}

#[test]
fn test_start_in_blocked_cell() {
    let _l = get_lock();
    
    let is_blocked = |x, y| {
        x == 2 && y == 2
    };

    let result = terrain_raycast_2d(2, 2, 5, 5, &is_blocked);
    assert_eq!(result.hit, true);
    assert_eq!(result.hit_point, (2, 2));
    assert_eq!(result.distance, 0.0);
    assert_eq!(result.traversed_cells, vec![(2, 2)]);
}

#[test]
fn test_horizontal_vertical_diagonal_rays() {
    let _l = get_lock();
    
    let is_blocked = |_, _| false;
    
    // Vertical
    let result = terrain_raycast_2d(2, 2, 2, 5, &is_blocked);
    assert_eq!(result.hit, false);
    assert_eq!(result.distance, 3.0);
    assert_eq!(result.traversed_cells, vec![(2,2), (2,3), (2,4), (2,5)]);
    
    // Diagonal
    let result = terrain_raycast_2d(0, 0, 3, 3, &is_blocked);
    assert_eq!(result.hit, false);
    assert_eq!(result.distance, (3.0_f32).hypot(3.0));
    
    // Since dx=3, dy=3, the step_x=1, step_y=1, delta_x=0.333, delta_y=0.333
    // t_max_x = 0.166, t_max_y = 0.166
    // Initially t_max_x < t_max_y might be evaluated as false since they are equal, 
    // leading to taking step_y first.
    // The exact path taken depends on floating point comparison of equal values.
    // So let's just check length of traversed cells and start/end points, or exact sequence.
    assert_eq!(result.traversed_cells, vec![(0, 0), (0, 1), (1, 1), (1, 2), (2, 2), (2, 3), (3, 3)]);
}

#[test]
fn test_edge_case_same_start_end() {
    let _l = get_lock();
    
    let is_blocked = |_, _| false;
    
    let result = terrain_raycast_2d(4, 4, 4, 4, &is_blocked);
    assert_eq!(result.hit, false);
    assert_eq!(result.distance, 0.0);
    assert_eq!(result.traversed_cells, vec![(4,4)]);
    
    let is_blocked_2 = |x, y| x == 4 && y == 4;
    let result_2 = terrain_raycast_2d(4, 4, 4, 4, &is_blocked_2);
    assert_eq!(result_2.hit, true);
    assert_eq!(result_2.hit_point, (4, 4));
    assert_eq!(result_2.distance, 0.0);
    assert_eq!(result_2.traversed_cells, vec![(4,4)]);
}
