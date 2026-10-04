use m2_client::wgpu_lod_dither_crossfade::{
    generate_wgsl_dither_function, LodCrossfadeState, LodDistanceThresholds, BAYER_MATRIX_4X4,
};

#[test]
fn test_bayer_matrix_properties() {
    assert_eq!(BAYER_MATRIX_4X4.len(), 4);
    assert_eq!(BAYER_MATRIX_4X4[0].len(), 4);

    // Kazda wartosc powinna byc w [0.0, 1.0)
    for row in &BAYER_MATRIX_4X4 {
        for &val in row {
            assert!(val >= 0.0 && val < 1.0);
        }
    }
}

#[test]
fn test_lod_distance_evaluation() {
    let thresholds = LodDistanceThresholds::default();

    // Bardzo blisko: LOD 0
    let s0 = LodCrossfadeState::evaluate(500.0, &thresholds);
    assert_eq!(s0.active_lod, 0);
    assert_eq!(s0.target_lod, 0);
    assert!(!s0.in_transition);

    // W pasmie przejscia 1200 - 1500
    let s_trans = LodCrossfadeState::evaluate(1350.0, &thresholds);
    assert_eq!(s_trans.active_lod, 0);
    assert_eq!(s_trans.target_lod, 1);
    assert!(s_trans.in_transition);
    assert!((s_trans.blend_factor - 0.5).abs() < 0.05);

    // LOD 1 stabilne
    let s1 = LodCrossfadeState::evaluate(2000.0, &thresholds);
    assert_eq!(s1.active_lod, 1);
    assert!(!s1.in_transition);

    // LOD 2 dalekie
    let s2 = LodCrossfadeState::evaluate(4000.0, &thresholds);
    assert_eq!(s2.active_lod, 2);
    assert!(!s2.in_transition);
}

#[test]
fn test_dither_pixel_discard() {
    let thresholds = LodDistanceThresholds::default();
    let s = LodCrossfadeState::evaluate(1350.0, &thresholds);
    assert!(s.in_transition);

    // Test dopasowania pikseli
    let mut lod0_survived = 0;
    let mut lod1_survived = 0;

    for y in 0..4 {
        for x in 0..4 {
            if !s.should_discard_pixel(0, x, y) {
                lod0_survived += 1;
            }
            if !s.should_discard_pixel(1, x, y) {
                lod1_survived += 1;
            }
        }
    }

    // W polowie blendu suma przetrwanych pikseli powinna byc pelnym pokryciem
    assert_eq!(lod0_survived + lod1_survived, 16);
}

#[test]
fn test_wgsl_shader_generation() {
    let shader = generate_wgsl_dither_function();
    assert!(shader.contains("get_bayer_threshold"));
    assert!(shader.contains("apply_lod_dither_crossfade"));
}
