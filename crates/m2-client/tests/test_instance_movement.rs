use m2_client::instance_movement::{
    InstanceMovement, PixelPosition, get_degree_from_position, clamp_degree
};
 
 #[test]
fn test_clamp_degree() {
    assert_eq!(clamp_degree(365.0), 5.0);
    assert_eq!(clamp_degree(-10.0), 350.0);
}

#[test]
fn test_skeleton_instance_movement() {
     let instance = InstanceMovement::new();
     assert!(instance.is_ready());
    assert!(!instance.is_going());
}

#[test]
fn test_pixel_position_sub() {
    let p1 = PixelPosition { x: 10.0, y: 5.0, z: 2.0 };
    let p2 = PixelPosition { x: 3.0, y: 2.0, z: 1.0 };
    
    let res = p1 - p2;
    assert_eq!(res.x, 7.0);
    assert_eq!(res.y, 3.0);
    assert_eq!(res.z, 1.0);
}

#[test]
fn test_get_degree_from_position() {
    let deg1 = get_degree_from_position(0.0, -1.0);
    assert!((deg1 - 0.0).abs() < 0.1 || (deg1 - 360.0).abs() < 0.1);

    let deg2 = get_degree_from_position(1.0, 0.0);
    assert!((deg2 - 90.0).abs() < 0.1);

    let deg3 = get_degree_from_position(0.0, 1.0);
    assert!((deg3 - 180.0).abs() < 0.1);

    let deg4 = get_degree_from_position(-1.0, 0.0);
    assert!((deg4 - 270.0).abs() < 0.1);
}

#[test]
fn test_new_goto_cancels_if_syncing() {
    let mut instance = InstanceMovement::new();
    instance.is_syncing = true;
    
    let dest = PixelPosition { x: 100.0, y: 100.0, z: 0.0 };
    let result = instance.new_goto(&dest, 90.0);
    
    assert_eq!(result, false);
    assert_eq!(instance.is_going(), false);
}

#[test]
fn test_new_move_to_direction_calculates_dst() {
    let mut instance = InstanceMovement::new();
    instance.cur_pos = PixelPosition { x: 0.0, y: 0.0, z: 0.0 };
    
    // Move forward (0 degrees)
    instance.new_move_to_direction(0.0);
    
    // In C++, m_isGoing is explicitly set to FALSE at the beginning of NEW_MoveToDirection
    // and not set back to true inside this function (only new_goto does it).
    // Let's adapt the test to reflect original C++ behavior.
    assert!(!instance.is_going);
    // Since we start at 0,0 and move by 0 degrees (forward, i.e. along negative Y in Metin2)
    // with distance 300
    // C++ logic: cur.y = 0.0, kD3DVt3Cur.y = -0.0
    // adv = (0, -1) * 300 = (0, -300)
    // Dst.y = - (0 + -300) = 300
    assert!((instance.dst_pos.x - 0.0).abs() < 1.0);
    assert!((instance.dst_pos.y - 300.0).abs() < 1.0);

    // Reset and move 90 degrees
    instance.cur_pos = PixelPosition { x: 0.0, y: 0.0, z: 0.0 };
    instance.new_move_to_direction(90.0);
    // rot_x = 0 * c - (-1) * s = 1.0
    // rot_y = 0 * s + (-1) * c = 0.0
    // Dst.x = 300
    // Dst.y = -(-0 + 0) = 0
    assert!((instance.dst_pos.x - 300.0).abs() < 1.0);
    assert!((instance.dst_pos.y - 0.0).abs() < 1.0);
 }
