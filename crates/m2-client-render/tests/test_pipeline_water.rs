use m2_client_render::pipeline_water::{PipelineWater, WaterParams};

#[test]
fn test_integration_pipeline_water() {
    let instance = PipelineWater::new();
    assert!(instance.is_ready());
}

#[test]
fn test_update_time() {
    let mut water = PipelineWater::new();
    assert_eq!(water.time, 0.0);
    water.update_time(1.5);
    assert_eq!(water.time, 1.5);
    water.update_time(0.5);
    assert_eq!(water.time, 2.0);
}

#[test]
fn test_set_params() {
    let mut water = PipelineWater::new();
    let params = WaterParams {
        wave_speed: 2.0,
        wave_amplitude: 0.5,
        fresnel_bias: 0.2,
        fresnel_scale: 0.8,
        fresnel_power: 3.0,
        refraction_distortion_factor: 0.1,
    };
    water.set_params(params.clone());

    assert_eq!(water.params.wave_speed, 2.0);
    assert_eq!(water.params.wave_amplitude, 0.5);
    assert_eq!(water.params.fresnel_bias, 0.2);
    assert_eq!(water.params.fresnel_scale, 0.8);
    assert_eq!(water.params.fresnel_power, 3.0);
    assert_eq!(water.params.refraction_distortion_factor, 0.1);
}

#[test]
fn test_calculate_wave_offset() {
    let mut water = PipelineWater::new();
    water.params.wave_speed = 1.0;
    water.params.wave_amplitude = 0.5;
    water.time = 0.0;
    
    // For x=0, z=0, time=0: wave1 = sin(0) = 0, wave2 = cos(0) = 1.0
    // offset = (0 + 1) * 0.5 = 0.5
    let offset = water.calculate_wave_offset(0.0, 0.0);
    assert!((offset - 0.5).abs() < f32::EPSILON);
    
    // time = pi / 2
    water.time = std::f32::consts::PI / 2.0;
    let offset2 = water.calculate_wave_offset(0.0, 0.0);
    // phase = pi/2
    // wave1 = sin(pi/2) = 1.0
    // wave2 = cos(1.2 * pi/2) = cos(0.6 * pi) = -0.30901699
    // offset = (1.0 - 0.30901699) * 0.5 = 0.690983 * 0.5 = 0.3454915
    let expected = (1.0_f32 + (0.6 * std::f32::consts::PI).cos()) * 0.5;
    assert!((offset2 - expected).abs() < 1e-5);
}

#[test]
fn test_calculate_fresnel() {
    let mut water = PipelineWater::new();
    water.params.fresnel_bias = 0.1;
    water.params.fresnel_scale = 0.5;
    water.params.fresnel_power = 2.0;

    // test cos_theta = 1.0 (looking straight down)
    // reflection = 0.1 + 0.5 * (1.0 - 1.0)^2 = 0.1
    let r1 = water.calculate_fresnel(1.0);
    assert!((r1 - 0.1).abs() < f32::EPSILON);

    // test cos_theta = 0.0 (looking parallel to surface)
    // reflection = 0.1 + 0.5 * (1.0 - 0.0)^2 = 0.6
    let r2 = water.calculate_fresnel(0.0);
    assert!((r2 - 0.6).abs() < f32::EPSILON);

    // test out-of-bounds (negative cos_theta clamped to 0.0)
    let r3 = water.calculate_fresnel(-0.5);
    assert!((r3 - 0.6).abs() < f32::EPSILON);

    // test out-of-bounds (cos_theta > 1.0 clamped to 1.0)
    let r4 = water.calculate_fresnel(1.5);
    assert!((r4 - 0.1).abs() < f32::EPSILON);
    
    // check final clamping
    water.params.fresnel_scale = 5.0; // will make reflection > 1.0
    let r5 = water.calculate_fresnel(0.0); // 0.1 + 5.0 * 1.0 = 5.1 -> clamped to 1.0
    assert!((r5 - 1.0).abs() < f32::EPSILON);
}

#[test]
fn test_calculate_refraction_distortion() {
    let mut water = PipelineWater::new();
    water.params.refraction_distortion_factor = 0.1;
    
    let (dx, dz) = water.calculate_refraction_distortion(0.5, -0.5);
    assert!((dx - 0.05).abs() < f32::EPSILON);
    assert!((dz - (-0.05)).abs() < f32::EPSILON);
}
