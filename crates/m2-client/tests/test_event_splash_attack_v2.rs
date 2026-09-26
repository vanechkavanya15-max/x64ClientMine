#[path = "../src/event_splash_attack_v2.rs"]
mod event_splash_attack_v2;

use event_splash_attack_v2::*;
use glam::Vec3;
use std::ffi::CString;

#[test]
fn test_parse_msa_basic() {
    let msa_content = "
        Group HitSphere
        {
            StartTime 0.1
            EndTime 0.5
            Radius 200.0
            ConeAngle 90.0
            Position 10.0 20.0 30.0
            Direction 0.0 1.0 0.0
        }
    ";

    let spheres = HitSphere::parse_from_msa(msa_content);
    assert_eq!(spheres.len(), 1);
    
    let s = &spheres[0];
    assert_eq!(s.start_time, 0.1);
    assert_eq!(s.end_time, 0.5);
    assert_eq!(s.radius, 200.0);
    assert_eq!(s.position, Vec3::new(10.0, 20.0, 30.0));
    assert_eq!(s.direction, Vec3::new(0.0, 1.0, 0.0));
}

#[test]
fn test_parse_msa_multiple() {
    let msa_content = "
        Group HitSphere
        {
            StartTime 0.1
            EndTime 0.5
            Radius 200.0
        }
        Group HitSphere
        {
            StartTime 0.6
            EndTime 1.0
            Radius 300.0
        }
    ";

    let spheres = HitSphere::parse_from_msa(msa_content);
    assert_eq!(spheres.len(), 2);
    assert_eq!(spheres[0].radius, 200.0);
    assert_eq!(spheres[1].radius, 300.0);
}

#[test]
fn test_hit_moment() {
    let s = HitSphere {
        start_time: 0.5,
        end_time: 1.0,
        radius: 100.0,
        cone_angle_cos: -1.0,
        position: Vec3::ZERO,
        direction: Vec3::Y,
    };

    assert!(!s.is_hit_moment(0.4));
    assert!(s.is_hit_moment(0.5));
    assert!(s.is_hit_moment(0.7));
    assert!(s.is_hit_moment(1.0));
    assert!(!s.is_hit_moment(1.1));
}

#[test]
fn test_target_in_range_and_cone() {
    let s = HitSphere {
        start_time: 0.0,
        end_time: 1.0,
        radius: 100.0,
        cone_angle_cos: (45.0_f32 * std::f32::consts::PI / 180.0).cos(), // 90 degree cone
        position: Vec3::ZERO,
        direction: Vec3::Y, // facing Y
    };

    let actor_pos = Vec3::ZERO;
    
    // Target directly in front, within range
    let target1 = Vec3::new(0.0, 50.0, 0.0);
    assert!(s.is_target_in_cone_and_range(target1, actor_pos));

    // Target directly in front, out of range
    let target2 = Vec3::new(0.0, 150.0, 0.0);
    assert!(!s.is_target_in_cone_and_range(target2, actor_pos));

    // Target behind, within range
    let target3 = Vec3::new(0.0, -50.0, 0.0);
    assert!(!s.is_target_in_cone_and_range(target3, actor_pos));
}

#[test]
fn test_ffi_integration() {
    let msa_content = "
        Group HitSphere
        {
            StartTime 0.2
            EndTime 0.4
            Radius 150.0
            ConeAngle 180.0
            Position 0.0 0.0 0.0
            Direction 0.0 1.0 0.0
        }
    ";
    let c_str = CString::new(msa_content).unwrap();
    
    m2_splash_v2_clear_spheres();
    
    let loaded = m2_splash_v2_load_msa(c_str.as_ptr());
    assert!(loaded);
    
    // Check hit moment
    assert!(m2_motion_is_hit_moment(0.3));
    assert!(!m2_motion_is_hit_moment(0.1));
    assert!(!m2_motion_is_hit_moment(0.5));
    
    // Check target in range (actor at origin, target in front)
    let hit = m2_splash_v2_check_target(0.3, 0.0, 50.0, 0.0, 0.0, 0.0, 0.0);
    assert!(hit);
    
    // Check target out of range
    let hit2 = m2_splash_v2_check_target(0.3, 0.0, 200.0, 0.0, 0.0, 0.0, 0.0);
    assert!(!hit2);
    
    // Check target correct time but not hit moment
    let hit3 = m2_splash_v2_check_target(0.1, 0.0, 50.0, 0.0, 0.0, 0.0, 0.0);
    assert!(!hit3);
    
    m2_splash_v2_clear_spheres();
}
