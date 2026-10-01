use glam::{Mat4, Vec2, Vec3};
use m2_client::player_mouse_walking::{unproject_screen_to_ray, Camera, PlayerMovement, Terrain};

fn create_test_camera() -> Camera {
    // A simple orthographic or perspective projection doesn't matter too much for unit tests
    // as long as we can predictably invert it. Let's use a standard perspective.
    let projection = Mat4::perspective_rh_gl(std::f32::consts::FRAC_PI_4, 800.0 / 600.0, 0.1, 1000.0);
    // Looking straight down at the origin from Z = 100
    let view = Mat4::look_at_rh(
        Vec3::new(0.0, 0.0, 100.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    Camera {
        view_matrix: view,
        projection_matrix: projection,
        viewport_width: 800.0,
        viewport_height: 600.0,
    }
}

#[test]
fn test_unproject_screen_to_ray() {
    let camera = create_test_camera();

    // Center of the screen
    let ray = unproject_screen_to_ray(400.0, 300.0, &camera);

    // The origin should be near (0, 0, 100) (the camera position)
    // The direction should be approximately (0, 0, -1) (looking down)
    
    assert!(
        (ray.origin.x).abs() < 1.0,
        "Ray origin X {} is not close to 0",
        ray.origin.x
    );
    assert!(
        (ray.origin.y).abs() < 1.0,
        "Ray origin Y {} is not close to 0",
        ray.origin.y
    );
    assert!(
        (ray.origin.z - 100.0).abs() < 2.0,
        "Ray origin Z {} is not close to 100",
        ray.origin.z
    );

    assert!(
        (ray.direction.x).abs() < 0.1,
        "Ray direction X {} is not close to 0",
        ray.direction.x
    );
    assert!(
        (ray.direction.y).abs() < 0.1,
        "Ray direction Y {} is not close to 0",
        ray.direction.y
    );
    assert!(
        (ray.direction.z + 1.0).abs() < 0.1,
        "Ray direction Z {} is not close to -1",
        ray.direction.z
    );
}

#[test]
fn test_terrain_raycast() {
    let mut terrain = Terrain::new(10, 10, 10.0);
    
    // Set a height for the cell at (50, 50) grid index (5, 5)
    terrain.set_height(5, 5, 10.0);

    let ray_hit = m2_client::player_mouse_walking::Ray {
        origin: Vec3::new(55.0, 55.0, 100.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
    };

    let hit = terrain.raycast(&ray_hit, 200.0, 1.0);
    assert!(hit.is_some());
    let hit_pos = hit.unwrap();
    assert!((hit_pos.x - 55.0).abs() < 0.1);
    assert!((hit_pos.y - 55.0).abs() < 0.1);

    let ray_miss = m2_client::player_mouse_walking::Ray {
        origin: Vec3::new(150.0, 150.0, 100.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
    };

    let miss = terrain.raycast(&ray_miss, 200.0, 1.0);
    assert!(miss.is_some()); // Will hit Z=0 default height
    let miss_pos = miss.unwrap();
    assert!((miss_pos.x - 150.0).abs() < 0.1);
    assert!((miss_pos.y - 150.0).abs() < 0.1);
}

#[test]
fn test_blocked_wall() {
    let mut terrain = Terrain::new(10, 10, 10.0);
    terrain.set_walkable(5, 5, false);
    
    assert!(!terrain.is_walkable(55.0, 55.0));
    assert!(terrain.is_walkable(45.0, 45.0));
}

#[test]
fn test_set_movement_target() {
    let terrain = Terrain::new(100, 100, 10.0);
    let camera = create_test_camera();
    
    let mut player = PlayerMovement::new(Vec2::new(0.0, 0.0), 300.0);
    
    // Click center of screen, should hit origin
    let handled = player.handle_mouse_click(400.0, 300.0, &camera, &terrain);
    
    assert!(handled);
    assert!(player.is_moving);
    assert!(player.target_position.is_some());
    
    let target = player.target_position.unwrap();
    assert!((target.x).abs() < 5.0, "Expected close to 0, got {}", target.x);
    assert!((target.y).abs() < 5.0, "Expected close to 0, got {}", target.y);
}

#[test]
fn test_unwalkable_click() {
    let mut terrain = Terrain::new(100, 100, 10.0);
    terrain.set_walkable(0, 0, false);
    
    let camera = create_test_camera();
    let mut player = PlayerMovement::new(Vec2::new(100.0, 100.0), 300.0);
    
    let handled = player.handle_mouse_click(400.0, 300.0, &camera, &terrain);
    
    assert!(!handled);
    assert!(!player.is_moving);
    assert!(player.target_position.is_none());
}

#[test]
fn test_wasd_cancellation() {
    let mut player = PlayerMovement::new(Vec2::new(0.0, 0.0), 300.0);
    player.target_position = Some(Vec2::new(100.0, 100.0));
    player.is_moving = true;
    
    player.handle_wasd_input(true); // WASD pressed
    
    assert!(!player.is_moving);
    assert!(player.target_position.is_none());
}

#[test]
fn test_calculate_angle_to_target() {
    let mut player = PlayerMovement::new(Vec2::new(0.0, 0.0), 300.0);
    player.target_position = Some(Vec2::new(100.0, 100.0));
    
    let angle = player.calculate_angle_to_target().unwrap();
    // Angle should be PI/4 (45 degrees)
    assert!((angle - std::f32::consts::FRAC_PI_4).abs() < 0.01);
}

#[test]
fn test_movement_and_stop_condition() {
    // Distance from (0,0) to (50, 0) is 50.
    // We start at (0, 0) and target (100, 0).
    let mut player = PlayerMovement::new(Vec2::new(0.0, 0.0), 10.0);
    player.target_position = Some(Vec2::new(100.0, 0.0));
    player.is_moving = true;
    
    // Update player, moving 10 units at a time.
    for _ in 0..5 {
        player.update(1.0); // 50 units total
    }
    
    assert!(player.is_moving);
    
    // Next update crosses to position 60 (distance 40), then the next one stops it,
    // or we just call it twice
    player.update(1.0); // pos = 60, dist = 40
    player.update(1.0); // dist 40 < 50, stops.
    
    assert!(!player.is_moving, "Player should stop moving when distance < 50");
    assert!(player.target_position.is_none());
}
