#[path = "../src/wgpu_weapon_trail_strip_v2.rs"]
mod wgpu_weapon_trail_strip_v2;

use glam::Vec3;
use wgpu_weapon_trail_strip_v2::WeaponTrailStripV2;

#[test]
fn test_weapon_trail_state() {
    let mut trail = WeaponTrailStripV2::new();
    assert!(!trail.is_playing);

    trail.turn_on();
    assert!(trail.is_playing);

    trail.turn_off();
    assert!(!trail.is_playing);

    trail.turn_on();
    trail.add_point(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(trail.short_points.len(), 1);
    assert_eq!(trail.long_points.len(), 1);

    trail.clear();
    assert_eq!(trail.short_points.len(), 0);
    assert_eq!(trail.long_points.len(), 0);
}

#[test]
fn test_weapon_trail_update_culling() {
    let mut trail = WeaponTrailStripV2::new();
    trail.turn_on();
    trail.set_life_time(0.5);

    // Add point at t=0
    trail.add_point(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

    // Update by 0.3s
    trail.update(0.3);

    // Add point at t=0.3
    trail.add_point(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));

    assert_eq!(trail.short_points.len(), 2);

    // Update by 0.3s more (total time 0.6s)
    // The first point is now 0.6s old, which is > 0.5s life_time
    // The second point is 0.3s old, which is < 0.5s life_time
    trail.update(0.3);

    // Wait! Based on the C++ port, we keep one extra point that exceeds life time to interpolate!
    // The first point is > 0.5. The second is < 0.5. Since the second point doesn't exceed, it won't trigger pop.
    // If we had three points: 0.9, 0.6, 0.3. 0.9 and 0.6 exceed. The 0.9 point would be popped.
    
    // Let's add a third point after another 0.3s
    trail.update(0.3); // now ages are 0.9, 0.6
    trail.add_point(Vec3::new(2.0, 0.0, 0.0), Vec3::new(2.0, 1.0, 0.0)); // 0.0
    // currently: 0.0 (idx 0), 0.6 (idx 1), 0.9 (idx 2)
    
    trail.update(0.0); // trigger culling loop
    
    // It should pop idx 2, because idx 1 also exceeds life_time (0.6 > 0.5).
    // It keeps idx 1 because idx 0 is <= 0.5.
    assert_eq!(trail.short_points.len(), 2);
    
    // Ensure the remaining points are the newest ones
    assert_eq!(trail.short_points.front().unwrap().pos, Vec3::new(2.0, 0.0, 0.0));
    assert_eq!(trail.short_points.back().unwrap().pos, Vec3::new(1.0, 0.0, 0.0));
}

#[test]
fn test_weapon_trail_build_insufficient_points() {
    let mut trail = WeaponTrailStripV2::new();
    trail.turn_on();

    // 0 points
    assert!(!trail.build_vertex());
    assert_eq!(trail.vertices.len(), 0);

    // 1 point
    trail.add_point(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    assert!(!trail.build_vertex());
    assert_eq!(trail.vertices.len(), 0);
}

#[test]
fn test_weapon_trail_build_spline() {
    let mut trail = WeaponTrailStripV2::new();
    trail.turn_on();
    trail.set_life_time(0.5);
    trail.set_sampling_time(0.1);

    // Add 3 points
    trail.add_point(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    trail.update(0.2);
    trail.add_point(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
    trail.update(0.2);
    trail.add_point(Vec3::new(2.0, 0.0, 0.0), Vec3::new(2.0, 1.0, 0.0));

    // build the spline
    assert!(trail.build_vertex());

    // We expect some vertices generated from spline
    assert!(trail.vertices.len() >= 4);

    // Verify interleaving (long, short, long, short...)
    for i in 0..trail.vertices.len() {
        if i % 2 == 0 {
            // long vertex
            assert_eq!(trail.vertices[i].tex_coord.v, 0.0);
        } else {
            // short vertex
            assert_eq!(trail.vertices[i].tex_coord.v, 1.0);
            
            // diffuse is encoded as (a << 24) | ...
            // short point alpha is always 0.0, so the alpha channel (top 8 bits) should be 0
            assert_eq!(trail.vertices[i].diffuse >> 24, 0); 
        }
    }
}
