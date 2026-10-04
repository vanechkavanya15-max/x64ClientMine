#[path = "../src/wgpu_dynamic_point_light_cluster.rs"]
mod wgpu_dynamic_point_light_cluster;

use wgpu_dynamic_point_light_cluster::{DynamicPointLightCluster, PointLight, calculate_blinn_phong_lighting};
use glam::Vec3;

#[test]
fn test_capacity_limits() {
    let mut cluster = DynamicPointLightCluster::new(2);
    
    // Adding first two lights should succeed
    assert!(cluster.add_light(PointLight::new(Vec3::ZERO, Vec3::ONE, 10.0, 1.0, 0.0, 0.0)).is_ok());
    assert!(cluster.add_light(PointLight::new(Vec3::ZERO, Vec3::ONE, 10.0, 1.0, 0.0, 0.0)).is_ok());
    
    // Adding a third light should fail because max capacity is 2
    assert!(cluster.add_light(PointLight::new(Vec3::ZERO, Vec3::ONE, 10.0, 1.0, 0.0, 0.0)).is_err());
    
    assert_eq!(cluster.get_lights().len(), 2);
    
    // Clearing should reset the count
    cluster.clear_lights();
    assert_eq!(cluster.get_lights().len(), 0);
}

#[test]
fn test_dynamic_flare_behavior() {
    let mut cluster = DynamicPointLightCluster::new(10);
    
    let base_radius = 100.0;
    
    // Add a light without dynamic flare
    let static_light = PointLight::new(Vec3::new(10.0, 0.0, 0.0), Vec3::ONE, base_radius, 1.0, 0.0, 0.0);
    cluster.add_light(static_light).unwrap();
    
    // Add a light with dynamic flare
    let flare_light = PointLight::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::ONE, base_radius, 1.0, 0.0, 0.0).with_dynamic_flare(true);
    cluster.add_light(flare_light).unwrap();
    
    // Update flares with time = 1.0
    cluster.update_dynamic_flares(1.0);
    
    let lights = cluster.get_lights();
    
    // Static light radius should remain unchanged
    assert_eq!(lights[0].radius, base_radius);
    
    // Flare light radius should change due to flicker simulation
    assert!(lights[1].radius != base_radius);
    
    // Update flares with time = 2.0 to ensure radius keeps changing
    let radius_at_t1 = lights[1].radius;
    cluster.update_dynamic_flares(2.0);
    let lights = cluster.get_lights();
    
    assert!(lights[1].radius != radius_at_t1);
    // Should still be close to base_radius
    assert!(lights[1].radius > base_radius * 0.5);
    assert!(lights[1].radius < base_radius * 1.5);
}

#[test]
fn test_torch_and_magic_presets() {
    let mut cluster = DynamicPointLightCluster::new(10);
    
    cluster.add_torch_light(Vec3::ZERO).unwrap();
    cluster.add_magic_flash_light(Vec3::new(10.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)).unwrap();
    
    let lights = cluster.get_lights();
    assert_eq!(lights.len(), 2);
    
    // Torch
    assert_eq!(lights[0].color, Vec3::new(1.0, 0.6, 0.2));
    assert!(lights[0].is_dynamic_flare);
    assert_eq!(lights[0].attenuation0, 1.0);
    assert_eq!(lights[0].attenuation1, 0.005);
    assert_eq!(lights[0].attenuation2, 0.0001);
    
    // Magic
    assert_eq!(lights[1].color, Vec3::new(0.0, 1.0, 0.0));
    assert!(lights[1].is_dynamic_flare);
    assert_eq!(lights[1].attenuation0, 1.0);
    assert_eq!(lights[1].attenuation1, 0.01);
    assert_eq!(lights[1].attenuation2, 0.0005);
}

#[test]
fn test_blinn_phong_calculations() {
    let light = PointLight::new(Vec3::new(0.0, 10.0, 0.0), Vec3::new(1.0, 1.0, 1.0), 50.0, 1.0, 0.01, 0.0001);
    let lights = vec![light];
    
    let frag_pos = Vec3::ZERO;
    let normal = Vec3::new(0.0, 1.0, 0.0); // Facing straight up at the light
    let view_pos = Vec3::new(10.0, 10.0, 0.0); // Viewer off to the side
    
    let ambient_color = Vec3::new(0.1, 0.1, 0.1);
    let diffuse_color = Vec3::new(0.8, 0.8, 0.8);
    let specular_color = Vec3::new(1.0, 1.0, 1.0);
    let shininess = 32.0;

    let result = calculate_blinn_phong_lighting(
        view_pos,
        frag_pos,
        normal,
        &lights,
        ambient_color,
        diffuse_color,
        specular_color,
        shininess,
    );

    // Assert that the resulting color has light contribution (more than just ambient)
    assert!(result.x > ambient_color.x);
    assert!(result.y > ambient_color.y);
    assert!(result.z > ambient_color.z);
    
    // Assert that the result is clamped to maximum 1.0
    assert!(result.x <= 1.0);
    assert!(result.y <= 1.0);
    assert!(result.z <= 1.0);
}

#[test]
fn test_blinn_phong_out_of_range() {
    // Light is very far away, beyond its radius
    let light = PointLight::new(Vec3::new(0.0, 100.0, 0.0), Vec3::new(1.0, 1.0, 1.0), 10.0, 1.0, 0.0, 0.0);
    let lights = vec![light];
    
    let frag_pos = Vec3::ZERO;
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let view_pos = Vec3::new(10.0, 10.0, 0.0);
    
    let ambient_color = Vec3::new(0.1, 0.1, 0.1);
    let diffuse_color = Vec3::new(0.8, 0.8, 0.8);
    let specular_color = Vec3::new(1.0, 1.0, 1.0);
    let shininess = 32.0;

    let result = calculate_blinn_phong_lighting(
        view_pos,
        frag_pos,
        normal,
        &lights,
        ambient_color,
        diffuse_color,
        specular_color,
        shininess,
    );

    // Because the fragment is out of the light's radius, only ambient color should remain
    assert_eq!(result, ambient_color);
}

#[test]
fn test_get_uniform_data() {
    let mut cluster = DynamicPointLightCluster::new(2);
    let light = PointLight::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.5, 0.5, 0.5), 10.0, 1.0, 0.1, 0.01);
    cluster.add_light(light).unwrap();

    let uniforms = cluster.get_uniform_data();
    assert_eq!(uniforms.light_count, 1);
    assert_eq!(uniforms.lights[0].position, [1.0, 2.0, 3.0]);
    assert_eq!(uniforms.lights[0].radius, 10.0);
    assert_eq!(uniforms.lights[0].color, [0.5, 0.5, 0.5]);
    assert_eq!(uniforms.lights[0].attenuation0, 1.0);
    assert_eq!(uniforms.lights[0].attenuation1, 0.1);
    assert_eq!(uniforms.lights[0].attenuation2, 0.01);
}
