use m2_client::building_interior_cull_v2::*;

#[test]
fn test_m2_building_is_inside_null_pointers() {
    let center = [0.0; 3];
    let extent = [1.0; 3];
    let axes = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    assert_eq!(m2_building_is_inside(0.0, 0.0, 0.0, std::ptr::null(), &extent, &axes), false);
    assert_eq!(m2_building_is_inside(0.0, 0.0, 0.0, &center, std::ptr::null(), &axes), false);
    assert_eq!(m2_building_is_inside(0.0, 0.0, 0.0, &center, &extent, std::ptr::null()), false);
}

#[test]
fn test_m2_building_is_inside_center() {
    let center = [0.0, 0.0, 0.0];
    let extent = [10.0, 20.0, 30.0];
    // Identity rotation
    let axes = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];

    assert_eq!(m2_building_is_inside(0.0, 0.0, 0.0, &center, &extent, &axes), true);
}

#[test]
fn test_m2_building_is_inside_boundaries() {
    let center = [0.0, 0.0, 0.0];
    let extent = [10.0, 20.0, 30.0];
    let axes = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];

    // On the edges (should be inside)
    assert_eq!(m2_building_is_inside(10.0, 20.0, 30.0, &center, &extent, &axes), true);
    assert_eq!(m2_building_is_inside(-10.0, -20.0, -30.0, &center, &extent, &axes), true);
    assert_eq!(m2_building_is_inside(10.0, 0.0, 0.0, &center, &extent, &axes), true);
    
    // Just outside
    assert_eq!(m2_building_is_inside(10.1, 0.0, 0.0, &center, &extent, &axes), false);
    assert_eq!(m2_building_is_inside(0.0, 20.1, 0.0, &center, &extent, &axes), false);
    assert_eq!(m2_building_is_inside(0.0, 0.0, 30.1, &center, &extent, &axes), false);
}

#[test]
fn test_m2_building_is_inside_rotated_obb() {
    let center = [50.0, 50.0, 10.0];
    let extent = [10.0, 5.0, 20.0];
    
    // 90 degrees rotation around Z axis
    // X axis becomes Y
    // Y axis becomes -X
    // Z axis stays Z
    let axes = [
        [0.0, 1.0, 0.0],
        [-1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
    ];

    // Inside the rotated box
    assert_eq!(m2_building_is_inside(50.0, 55.0, 10.0, &center, &extent, &axes), true); // +5 on world Y, maps to +5 on local X (which has extent 10)
    assert_eq!(m2_building_is_inside(45.0, 50.0, 10.0, &center, &extent, &axes), true); // -5 on world X, maps to +5 on local Y (which has extent 5)
    
    // Outside the rotated box
    assert_eq!(m2_building_is_inside(50.0, 61.0, 10.0, &center, &extent, &axes), false); // +11 on world Y -> +11 on local X -> exceeds 10
    assert_eq!(m2_building_is_inside(44.0, 50.0, 10.0, &center, &extent, &axes), false); // -6 on world X -> +6 on local Y -> exceeds 5
}

#[test]
fn test_m2_building_is_inside_arbitrary_position() {
    let center = [100.0, -200.0, 500.0];
    let extent = [100.0, 100.0, 100.0];
    let axes = [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ];

    assert_eq!(m2_building_is_inside(150.0, -150.0, 550.0, &center, &extent, &axes), true);
    assert_eq!(m2_building_is_inside(250.0, -150.0, 550.0, &center, &extent, &axes), false);
    assert_eq!(m2_building_is_inside(100.0, -350.0, 500.0, &center, &extent, &axes), false);
}
