use glam::{Mat4, Vec3};
use m2_client::engine_root_motion_actor_sync::RootMotionActorSync;

#[test]
fn test_extract_root_bone_delta() {
    let prev_matrix = Mat4::from_translation(Vec3::new(10.0, 5.0, 0.0));
    let curr_matrix = Mat4::from_translation(Vec3::new(12.0, 6.0, 1.0));

    let delta = RootMotionActorSync::extract_root_bone_delta(&prev_matrix, &curr_matrix);

    assert_eq!(delta, Vec3::new(2.0, 1.0, 1.0));
}

#[test]
fn test_accumulate_movement_no_rotation() {
    let mut sync = RootMotionActorSync::new();
    let local_movement = Vec3::new(5.0, 0.0, 0.0);
    
    // Brak rotacji, wektor powinien zostac bez zmian
    sync.accumulate_movement(local_movement, 0.0);

    assert_eq!(sync.accumulated_movement, Vec3::new(5.0, 0.0, 0.0));
}

#[test]
fn test_accumulate_movement_90_deg_rotation() {
    let mut sync = RootMotionActorSync::new();
    let local_movement = Vec3::new(5.0, 0.0, 0.0);
    
    // Obrot o 90 stopni wzgledem osi Z
    // Zatem wektor (5, 0, 0) po obrocie wzgledem Z o 90st (w lewo) powinien byc bliski (0, 5, 0)
    sync.accumulate_movement(local_movement, 90.0);

    let acc = sync.accumulated_movement;
    assert!((acc.x - 0.0).abs() < 1e-5, "x: {}", acc.x);
    assert!((acc.y - 5.0).abs() < 1e-5, "y: {}", acc.y);
    assert_eq!(acc.z, 0.0);
}

#[test]
fn test_apply_root_motion_and_clear() {
    let mut sync = RootMotionActorSync::new();
    sync.accumulated_movement = Vec3::new(1.0, 2.0, 3.0);

    let actor_position = Vec3::new(10.0, 10.0, 10.0);
    let new_position = sync.apply_root_motion_and_clear(actor_position);

    assert_eq!(new_position, Vec3::new(11.0, 12.0, 13.0));
    // Akumulator powinien byc wyzerowany
    assert_eq!(sync.accumulated_movement, Vec3::ZERO);
}

#[test]
fn test_apply_root_motion_mut() {
    let mut sync = RootMotionActorSync::new();
    sync.accumulated_movement = Vec3::new(5.0, -5.0, 2.0);

    let mut actor_position = Vec3::new(0.0, 0.0, 0.0);
    sync.apply_root_motion(&mut actor_position);

    assert_eq!(actor_position, Vec3::new(5.0, -5.0, 2.0));
    assert_eq!(sync.accumulated_movement, Vec3::ZERO);
}
