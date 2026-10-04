use m2_client::actor_melee_hit_sampling::{
    detect_collision_dynamic_z_cylinder_vs_dynamic_z_cylinder, intersect_line_segments_2d, process_melee_hit_sampling, AttackData, DynamicSphereInstance, HitData, HitDataId, HitDataMap, MotionType, Vec3
};
use std::collections::BTreeMap;

#[test]
fn test_intersect_line_segments_2d() {
    let a1 = Vec3::new(0.0, 0.0, 0.0);
    let a2 = Vec3::new(10.0, 0.0, 0.0);
    let b1 = Vec3::new(5.0, -5.0, 0.0);
    let b2 = Vec3::new(5.0, 5.0, 0.0);

    let (pa, pb) = intersect_line_segments_2d(a1, a2, b1, b2);
    assert!((pa.x - 5.0).abs() < 1e-5);
    assert!((pa.y - 0.0).abs() < 1e-5);
    assert!((pb.x - 5.0).abs() < 1e-5);
    assert!((pb.y - 0.0).abs() < 1e-5);
}

#[test]
fn test_collision_dynamic_z_cylinder() {
    let s1 = DynamicSphereInstance::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 0.0),
        2.0,
    );
    let s2 = DynamicSphereInstance::new(
        Vec3::new(5.0, 5.0, 0.0),
        Vec3::new(5.0, -5.0, 0.0),
        2.0,
    );

    assert!(detect_collision_dynamic_z_cylinder_vs_dynamic_z_cylinder(&s1, &s2));

    let s3 = DynamicSphereInstance::new(
        Vec3::new(0.0, 10.0, 0.0),
        Vec3::new(10.0, 10.0, 0.0),
        2.0,
    );
    let s4 = DynamicSphereInstance::new(
        Vec3::new(5.0, -10.0, 0.0),
        Vec3::new(5.0, -5.0, 0.0),
        2.0,
    );

    assert!(!detect_collision_dynamic_z_cylinder_vs_dynamic_z_cylinder(&s3, &s4));
}

#[test]
fn test_process_melee_hit_sampling_basic_hit() {
    let attacker_pos = Vec3::new(0.0, 0.0, 0.0);
    let attacker_rot_deg = 0.0;
    let attacker_reach_scale = 1.0;
    let attack_radius = 5.0;
    let motion_time = 1.0;
    let elapsed_second = 0.1;
    let local_time = 100.0;

    let attack_data = AttackData {
        f_invisible_time: 1.0,
        i_hit_limit_count: 16,
        i_motion_type: MotionType::Normal,
    };

    let mut map_hit_position = BTreeMap::new();
    map_hit_position.insert(950, DynamicSphereInstance::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
        0.0, // Overwritten by attack_radius
    ));

    let hit_data = HitData {
        id: HitDataId(1),
        f_attack_start_time: 0.0,
        f_attack_end_time: 2.0,
        map_hit_position,
    };

    let mut hit_data_map = HitDataMap::new();

    let victim_spheres = vec![DynamicSphereInstance::new(
        Vec3::new(0.0, 5.0, 0.0),
        Vec3::new(0.0, 5.0, 0.0),
        2.0,
    )];
    let victims = vec![(1001, Vec3::new(0.0, 5.0, 0.0), victim_spheres)];

    let results = process_melee_hit_sampling(
        attacker_pos,
        attacker_rot_deg,
        attacker_reach_scale,
        attack_radius,
        motion_time,
        elapsed_second,
        local_time,
        &attack_data,
        &[hit_data],
        &mut hit_data_map,
        &victims,
    );

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].victim_id, 1001);
}

#[test]
fn test_process_melee_hit_sampling_invincibility() {
    let attacker_pos = Vec3::new(0.0, 0.0, 0.0);
    let attacker_rot_deg = 0.0;
    let attacker_reach_scale = 1.0;
    let attack_radius = 5.0;
    let motion_time = 1.0;
    let elapsed_second = 0.1;
    let local_time = 100.0;

    let attack_data = AttackData {
        f_invisible_time: 1.0,
        i_hit_limit_count: 16,
        i_motion_type: MotionType::Normal,
    };

    let mut map_hit_position = BTreeMap::new();
    map_hit_position.insert(950, DynamicSphereInstance::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
        0.0,
    ));

    let hit_data = HitData {
        id: HitDataId(1),
        f_attack_start_time: 0.0,
        f_attack_end_time: 2.0,
        map_hit_position,
    };

    let mut hit_data_map = HitDataMap::new();
    
    // Set victim as already hit, and still invincible
    let mut hitted = std::collections::HashMap::new();
    hitted.insert(1001, 100.5); // expires at 100.5, current local_time is 100.0
    hit_data_map.insert(HitDataId(1), hitted);

    let victim_spheres = vec![DynamicSphereInstance::new(
        Vec3::new(0.0, 5.0, 0.0),
        Vec3::new(0.0, 5.0, 0.0),
        2.0,
    )];
    let victims = vec![(1001, Vec3::new(0.0, 5.0, 0.0), victim_spheres)];

    let results = process_melee_hit_sampling(
        attacker_pos,
        attacker_rot_deg,
        attacker_reach_scale,
        attack_radius,
        motion_time,
        elapsed_second,
        local_time,
        &attack_data,
        &[hit_data],
        &mut hit_data_map,
        &victims,
    );

    assert_eq!(results.len(), 0); // Should not hit due to invincibility
}

#[test]
fn test_process_melee_hit_sampling_limit() {
    let attacker_pos = Vec3::new(0.0, 0.0, 0.0);
    let attacker_rot_deg = 0.0;
    let attacker_reach_scale = 1.0;
    let attack_radius = 5.0;
    let motion_time = 1.0;
    let elapsed_second = 0.1;
    let local_time = 100.0;

    let attack_data = AttackData {
        f_invisible_time: 1.0,
        i_hit_limit_count: 2, // Low limit for test
        i_motion_type: MotionType::Skill,
    };

    let mut map_hit_position = BTreeMap::new();
    map_hit_position.insert(950, DynamicSphereInstance::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0),
        0.0,
    ));

    let hit_data = HitData {
        id: HitDataId(1),
        f_attack_start_time: 0.0,
        f_attack_end_time: 2.0,
        map_hit_position,
    };

    let mut hit_data_map = HitDataMap::new();

    // Already hit 2 targets
    let mut hitted = std::collections::HashMap::new();
    hitted.insert(2001, 100.5);
    hitted.insert(2002, 100.5);
    hit_data_map.insert(HitDataId(1), hitted);

    let victim_spheres = vec![DynamicSphereInstance::new(
        Vec3::new(0.0, 5.0, 0.0),
        Vec3::new(0.0, 5.0, 0.0),
        2.0,
    )];
    let victims = vec![(1001, Vec3::new(0.0, 5.0, 0.0), victim_spheres)];

    let results = process_melee_hit_sampling(
        attacker_pos,
        attacker_rot_deg,
        attacker_reach_scale,
        attack_radius,
        motion_time,
        elapsed_second,
        local_time,
        &attack_data,
        &[hit_data],
        &mut hit_data_map,
        &victims,
    );

    assert_eq!(results.len(), 0); // Limit reached, should be 0
}
