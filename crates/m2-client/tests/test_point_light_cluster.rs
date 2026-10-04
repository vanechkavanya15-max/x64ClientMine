#[path = "../src/wgpu_dynamic_point_light_cluster.rs"]
mod wgpu_dynamic_point_light_cluster;

use glam::Vec3;
use wgpu_dynamic_point_light_cluster::{
    calculate_blinn_phong_lighting, DynamicPointLightCluster, MAX_POINT_LIGHTS,
};

#[test]
fn test_add_and_clear_lights() {
    let mut cluster = DynamicPointLightCluster::new();
    assert_eq!(cluster.uniforms.light_count, 0);

    // Add a basic point light
    let success = cluster.add_point_light(
        Vec3::new(10.0, 20.0, 30.0),
        Vec3::new(1.0, 0.0, 0.0),
        2.0,
        1.0,
        0.1,
        0.01,
        500.0,
    );

    assert!(success);
    assert_eq!(cluster.uniforms.light_count, 1);
    
    let light = &cluster.uniforms.lights[0];
    assert_eq!(light.position, [10.0, 20.0, 30.0, 1.0]);
    assert_eq!(light.color, [1.0, 0.0, 0.0, 2.0]);
    assert_eq!(light.attenuation, [1.0, 0.1, 0.01, 500.0]);

    // Clear lights
    cluster.clear();
    assert_eq!(cluster.uniforms.light_count, 0);
    assert_eq!(cluster.uniforms.lights[0].position[3], 0.0); // inactive
}

#[test]
fn test_max_point_lights_limit() {
    let mut cluster = DynamicPointLightCluster::new();

    // Fill up to max
    for i in 0..MAX_POINT_LIGHTS {
        let success = cluster.add_point_light(
            Vec3::new(i as f32, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 1.0),
            1.0,
            1.0,
            0.1,
            0.01,
            100.0,
        );
        assert!(success);
    }

    assert_eq!(cluster.uniforms.light_count, MAX_POINT_LIGHTS as u32);

    // Try adding one more
    let overflow_success = cluster.add_point_light(
        Vec3::new(999.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
        1.0,
        1.0,
        0.1,
        0.01,
        100.0,
    );
    assert!(!overflow_success); // Should fail to add
    assert_eq!(cluster.uniforms.light_count, MAX_POINT_LIGHTS as u32); // Count remains max
}

#[test]
fn test_add_torch_and_magic_flash_presets() {
    let mut cluster = DynamicPointLightCluster::new();

    // Add torch light
    cluster.add_torch_light(Vec3::new(100.0, 50.0, 10.0), 0.5); // +0.5 fluctuation
    assert_eq!(cluster.uniforms.light_count, 1);
    let torch = &cluster.uniforms.lights[0];
    
    assert_eq!(torch.position, [100.0, 50.0, 10.0, 1.0]);
    assert_eq!(torch.color, [1.0, 0.6, 0.2, 1.7]); // base 1.2 + 0.5
    assert_eq!(torch.attenuation, [1.0, 0.09, 0.032, 600.0]);

    // Add magic flash light
    cluster.add_magic_flash_light(Vec3::new(200.0, 100.0, 20.0), Vec3::new(0.0, 1.0, 1.0));
    assert_eq!(cluster.uniforms.light_count, 2);
    let flash = &cluster.uniforms.lights[1];

    assert_eq!(flash.position, [200.0, 100.0, 20.0, 1.0]);
    assert_eq!(flash.color, [0.0, 1.0, 1.0, 5.0]); // color and fixed intensity 5.0
    assert_eq!(flash.attenuation, [1.0, 0.045, 0.0075, 800.0]);
}

#[test]
fn test_calculate_blinn_phong_lighting() {
    let mut cluster = DynamicPointLightCluster::new();
    cluster.add_point_light(
        Vec3::new(0.0, 10.0, 0.0), // light at y=10
        Vec3::new(1.0, 1.0, 1.0),  // white light
        1.0,                       // intensity 1.0
        1.0,                       // constant
        0.0,                       // linear (no linear falloff)
        0.0,                       // quadratic (no quad falloff)
        20.0,                      // radius 20
    );

    let light = &cluster.uniforms.lights[0];
    
    // Fragment directly under the light
    let frag_pos = Vec3::new(0.0, 0.0, 0.0);
    let normal = Vec3::new(0.0, 1.0, 0.0); // pointing UP
    let view_dir = Vec3::new(0.0, 1.0, 1.0).normalize(); // looking slightly from z

    let shininess = 32.0;

    let (diffuse, specular) = calculate_blinn_phong_lighting(light, frag_pos, normal, view_dir, shininess);

    // Diffuse should be strong since normal aligns perfectly with light direction
    // Light is at distance 10, radius 20. Attenuation should be ~1.0 with some windowing reduction
    assert!(diffuse.x > 0.5 && diffuse.x < 1.0);
    assert_eq!(diffuse.x, diffuse.y); // White light
    assert_eq!(diffuse.y, diffuse.z);

    // Specular should be positive but smaller depending on halfway vector
    assert!(specular.x > 0.0);

    // Fragment outside radius
    let frag_pos_far = Vec3::new(30.0, 0.0, 0.0); // distance ~31.6 > radius 20
    let (diff_far, spec_far) = calculate_blinn_phong_lighting(light, frag_pos_far, normal, view_dir, shininess);
    
    assert_eq!(diff_far, Vec3::ZERO);
    assert_eq!(spec_far, Vec3::ZERO);
}
