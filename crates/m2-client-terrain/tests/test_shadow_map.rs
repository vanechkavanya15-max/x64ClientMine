use m2_client_terrain::shadow_map::{ShadowMap, ShadowMapError};

#[test]
fn test_shadow_map_creation() {
    let width = 2;
    let height = 2;
    let data = vec![
        0x1020, 0x3040, // [ (sun:16, amb:32), (sun:48, amb:64) ]
        0x5060, 0x7080, // [ (sun:80, amb:96), (sun:112, amb:128) ]
    ];
    let sm = ShadowMap::new(width, height, data).unwrap();
    
    assert_eq!(sm.width, 2);
    assert_eq!(sm.height, 2);
}

#[test]
fn test_shadow_map_invalid_dimensions() {
    let sm = ShadowMap::new(0, 2, vec![0x1020, 0x3040]);
    assert_eq!(sm.err(), Some(ShadowMapError::InvalidDimensions));

    let sm2 = ShadowMap::new(2, 0, vec![0x1020, 0x3040]);
    assert_eq!(sm2.err(), Some(ShadowMapError::InvalidDimensions));
}

#[test]
fn test_shadow_map_invalid_data_size() {
    let data = vec![0x1020, 0x3040, 0x5060];
    let sm = ShadowMap::new(2, 2, data);
    assert_eq!(sm.err(), Some(ShadowMapError::InvalidDataSize));
}

#[test]
fn test_get_sun_and_ambient() {
    let data = vec![
        0x1020, 0x3040,
        0x5060, 0x7080,
    ];
    let sm = ShadowMap::new(2, 2, data).unwrap();

    assert_eq!(sm.get_sun_shadow(0, 0), Some(0x10));
    assert_eq!(sm.get_static_ambient(0, 0), Some(0x20));

    assert_eq!(sm.get_sun_shadow(1, 0), Some(0x30));
    assert_eq!(sm.get_static_ambient(1, 0), Some(0x40));

    assert_eq!(sm.get_sun_shadow(0, 1), Some(0x50));
    assert_eq!(sm.get_static_ambient(0, 1), Some(0x60));

    assert_eq!(sm.get_sun_shadow(1, 1), Some(0x70));
    assert_eq!(sm.get_static_ambient(1, 1), Some(0x80));

    // Poza zakresem
    assert_eq!(sm.get_sun_shadow(2, 0), None);
    assert_eq!(sm.get_static_ambient(0, 2), None);
}

#[test]
fn test_interpolated_shadow_and_ambient() {
    let data = vec![
        0x1020, 0x3040,
        0x5060, 0x7080,
    ];
    let sm = ShadowMap::new(2, 2, data).unwrap();

    // Dla srodka pierwszego i drugiego punktu X: 0.5, Y: 0.0
    // Sun: X0Y0=16, X1Y0=48 => 32
    assert_eq!(sm.get_interpolated_sun_shadow(0.5, 0.0), Some(32.0));
    
    // Ambient: X0Y0=32, X1Y0=64 => 48
    assert_eq!(sm.get_interpolated_static_ambient(0.5, 0.0), Some(48.0));

    // Dla X: 0.0, Y: 0.5
    // Sun: X0Y0=16, X0Y1=80 => 48
    assert_eq!(sm.get_interpolated_sun_shadow(0.0, 0.5), Some(48.0));
    
    // Dla X: 0.5, Y: 0.5
    // Top Sun: 32, Bottom Sun: X0Y1=80, X1Y1=112 => 96
    // Avg Sun: (32 + 96) / 2 = 64
    assert_eq!(sm.get_interpolated_sun_shadow(0.5, 0.5), Some(64.0));
    
    // Poza zakresem
    assert_eq!(sm.get_interpolated_sun_shadow(1.5, 0.0), None);
}

#[test]
fn test_calculate_shading() {
    let data = vec![
        0xFF80, 0xFF80, // Sun = 255, Ambient = 128
        0xFF80, 0xFF80,
    ];
    let sm = ShadowMap::new(2, 2, data).unwrap();

    // Sprawdzamy dokladnie z kafelka (0.0, 0.0)
    // Sun factor = 255 / 255 = 1.0
    // Ambient factor = 128 / 255 = ~0.50196
    // Total = 1.0 * light_intensity + 0.50196 * global_ambient
    let shading = sm.calculate_shading(0.0, 0.0, 0.5, 0.5).unwrap();
    
    let expected = (1.0 * 0.5) + (128.0 / 255.0 * 0.5);
    assert!((shading - expected as f32).abs() < f32::EPSILON);
    
    // Powinno byc przyciete do 1.0
    let shading_max = sm.calculate_shading(0.0, 0.0, 2.0, 2.0).unwrap();
    assert_eq!(shading_max, 1.0);
}
