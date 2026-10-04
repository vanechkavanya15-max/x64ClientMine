use m2_client::speedtree_billboard_oscillator::{
    SpeedTreeInstance, SpeedTreeLodLevel, SpeedTreeManager, ViewFrustum, m2_speedtree_add_instance,
    m2_speedtree_advance_time, m2_speedtree_clear_instances, m2_speedtree_evaluate_lod,
    m2_speedtree_get_deflection, m2_speedtree_set_wind_strength, m2_speedtree_get_visible_instances,
};

#[test]
fn test_wind_sway_deflection() {
    let mut mgr = SpeedTreeManager::new();
    mgr.get_oscillator_mut().set_wind_strength(1.0);
    // Move time by 0.5 to reach sin(0.5 * 2 * PI * 0.5) = sin(PI/2) = 1.0
    mgr.get_oscillator_mut().advance_time(0.5);

    let deflection = mgr
        .get_oscillator_mut()
        .calculate_deflection(50.0, 100.0);
    
    // deflection = sin(PI/2) * 1.0 * (50/100) * 0.1 = 1.0 * 1.0 * 0.5 * 0.1 = 0.05
    assert!((deflection - 0.05).abs() < f32::EPSILON);
    
    // Test base case 0
    let deflection_base = mgr
        .get_oscillator_mut()
        .calculate_deflection(0.0, 100.0);
    assert_eq!(deflection_base, 0.0);
}

#[test]
fn test_lod_distance_thresholds() {
    let mgr = SpeedTreeManager::new();
    
    // Mesh3D (< 1000)
    let res = mgr.evaluate_lod(500.0);
    assert_eq!(res.lod_level, SpeedTreeLodLevel::Mesh3D);
    assert_eq!(res.alpha, 1.0);

    // CrossBillboard (< 3000)
    let res = mgr.evaluate_lod(2000.0);
    assert_eq!(res.lod_level, SpeedTreeLodLevel::CrossBillboard);
    assert_eq!(res.alpha, 1.0);

    // SingleBillboard (< 10000)
    let res = mgr.evaluate_lod(5000.0);
    assert_eq!(res.lod_level, SpeedTreeLodLevel::SingleBillboard);
    assert_eq!(res.alpha, 1.0);

    // Culled (>= 10000)
    let res = mgr.evaluate_lod(12000.0);
    assert_eq!(res.lod_level, SpeedTreeLodLevel::Culled);
    assert_eq!(res.alpha, 0.0);
}

#[test]
fn test_alpha_dither_crossfade() {
    let mgr = SpeedTreeManager::new();
    
    // Mesh3D to CrossBillboard overlap is 800.0 to 1000.0
    let dist = 900.0;
    let res = mgr.evaluate_lod(dist);
    assert_eq!(res.lod_level, SpeedTreeLodLevel::Mesh3D);
    // overlap_start = 800. distance = 900. overlap = 100. alpha = 1.0 - (100 / 200) = 0.5
    assert!((res.alpha - 0.5).abs() < f32::EPSILON);
    
    // CrossBillboard to SingleBillboard overlap is 2800.0 to 3000.0
    let dist2 = 2950.0;
    let res2 = mgr.evaluate_lod(dist2);
    assert_eq!(res2.lod_level, SpeedTreeLodLevel::CrossBillboard);
    // overlap_start = 2800. distance = 2950. overlap = 150. alpha = 1.0 - (150 / 200) = 0.25
    assert!((res2.alpha - 0.25).abs() < f32::EPSILON);
}

#[test]
fn test_frustum_culling() {
    let mut mgr = SpeedTreeManager::new();
    mgr.add_instance(SpeedTreeInstance { id: 1, x: 100.0, y: 100.0, z: 0.0, tree_height: 50.0 });
    mgr.add_instance(SpeedTreeInstance { id: 2, x: 500.0, y: 500.0, z: 0.0, tree_height: 50.0 });
    
    let frustum = ViewFrustum { x: 0.0, y: 0.0, radius: 200.0 };
    let visible = mgr.get_visible_instances(&frustum);
    
    assert_eq!(visible.len(), 1);
    assert_eq!(visible[0].id, 1);
}

#[test]
fn test_ffi_integration() {
    // Clear state before testing
    m2_speedtree_clear_instances();
    
    m2_speedtree_add_instance(1, 100.0, 100.0, 0.0, 50.0);
    
    m2_speedtree_set_wind_strength(2.0);
    m2_speedtree_advance_time(0.5);
    
    // 2.0 * sin(PI/2) * (25/50) * 0.1 = 2.0 * 1.0 * 0.5 * 0.1 = 0.1
    let deflection = m2_speedtree_get_deflection(25.0, 50.0);
    assert!((deflection - 0.1).abs() < f32::EPSILON);
    
    let mut out_alpha: f32 = -1.0;
    // 500 is Mesh3D -> return 0
    let lod_val = m2_speedtree_evaluate_lod(500.0, &mut out_alpha);
    assert_eq!(lod_val, 0);
    assert_eq!(out_alpha, 1.0);
    
    // Test frustum culling via FFI
    let mut visible_instances = vec![
        SpeedTreeInstance { id: 0, x: 0.0, y: 0.0, z: 0.0, tree_height: 0.0 }; 5
    ];
    let count = m2_speedtree_get_visible_instances(100.0, 100.0, 50.0, visible_instances.as_mut_ptr(), 5);
    assert_eq!(count, 1);
    assert_eq!(visible_instances[0].id, 1);
    assert_eq!(visible_instances[0].x, 100.0);
}
