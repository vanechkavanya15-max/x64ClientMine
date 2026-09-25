use m2_client::instance_transform::{InstanceTransform, PixelPosition};

#[test]
fn test_integration_instance_transform() {
    let instance = InstanceTransform::new();
    assert!(instance.is_ready());
}

#[test]
fn test_distance_calculations() {
    let p1 = PixelPosition::new(0.0, 0.0, 0.0);
    let p2 = PixelPosition::new(3.0, 4.0, 0.0);
    let p3 = PixelPosition::new(0.0, 0.0, 5.0);

    assert!((p1.distance_2d(&p2) - 5.0).abs() < f32::EPSILON);
    assert!((p1.distance_3d(&p2) - 5.0).abs() < f32::EPSILON);
    assert!((p1.distance_3d(&p3) - 5.0).abs() < f32::EPSILON);
}

#[test]
fn test_pixel_position_get_set() {
    let mut instance = InstanceTransform::new();
    let pos = PixelPosition::new(10.0, 20.0, 30.0);
    
    instance.new_set_pixel_position(&pos);
    let retrieved_pos = instance.new_get_pixel_position();
    
    assert_eq!(retrieved_pos.x, 10.0);
    assert_eq!(retrieved_pos.y, 20.0);
    assert_eq!(retrieved_pos.z, 30.0);
}

#[test]
fn test_script_set_pixel_position() {
    let mut instance = InstanceTransform::new();
    let height_resolver = |_x: f32, _y: f32| 42.0;

    instance.script_set_pixel_position(&height_resolver, 15.0, 25.0);
    
    let pos = instance.new_get_pixel_position();
    assert_eq!(pos.x, 15.0);
    assert_eq!(pos.y, 25.0);
    assert_eq!(pos.z, 42.0);
}

#[test]
fn test_rotation_get_set() {
    let mut instance = InstanceTransform::new();
    instance.set_rotation(90.0);
    assert_eq!(instance.get_rotation(), 90.0);

    instance.blend_rotation(180.0, 0.5);
    assert_eq!(instance.get_rotation(), 180.0);

    instance.set_advancing_rotation(270.0);
    assert_eq!(instance.get_advancing_rotation(), 270.0);
}

#[test]
fn test_direction() {
    let mut instance = InstanceTransform::new();
    
    assert_eq!(InstanceTransform::get_degree_from_direction(-1), 0.0);
    assert_eq!(InstanceTransform::get_degree_from_direction(8), 0.0);
    assert_eq!(InstanceTransform::get_degree_from_direction(0), 180.0); // 45 * 4
    assert_eq!(InstanceTransform::get_degree_from_direction(3), 45.0);
    assert_eq!(InstanceTransform::get_degree_from_direction(4), 0.0);

    instance.set_direction(3);
    assert_eq!(instance.get_rotation(), 45.0);
    assert_eq!(instance.get_advancing_rotation(), 45.0);

    instance.blend_direction(0, 1.0);
    assert_eq!(instance.get_rotation(), 180.0);
}

#[test]
fn test_look_at() {
    let mut instance1 = InstanceTransform::new();
    instance1.new_set_pixel_position(&PixelPosition::new(0.0, 0.0, 0.0));

    let target_pos1 = PixelPosition::new(0.0, 10.0, 0.0);
    instance1.new_look_at_dest_pixel_position(&target_pos1);
    
    // dx = 0, dy = -10. atan2(0, -10) = 180 or -180.
    // In our calculation: dx = 0.0, dy = -(10.0 - 0.0) = -10.0. atan2(0.0, -10.0) is 180 degrees.
    assert_eq!(instance1.get_rotation(), 180.0);

    let target_pos2 = PixelPosition::new(10.0, 0.0, 0.0);
    instance1.new_look_at_dest_pixel_position(&target_pos2);
    
    // dx = 10, dy = 0. atan2(10, 0) = 90 degrees.
    assert_eq!(instance1.get_rotation(), 90.0);

    let mut instance2 = InstanceTransform::new();
    instance2.new_set_pixel_position(&PixelPosition::new(0.0, -10.0, 0.0));
    instance1.new_look_at_dest_instance(&instance2);
    // dx = 0, dy = -(-10 - 0) = 10. atan2(0, 10) = 0.
    assert_eq!(instance1.get_rotation(), 0.0);
}
