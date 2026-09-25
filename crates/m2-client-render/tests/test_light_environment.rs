use m2_client_render::light_environment::{Color, FogParameters, LightEnvironment, Vector3};

#[test]
fn test_integration_light_environment() {
    let instance = LightEnvironment::new();
    assert!(instance.is_ready());
}

#[test]
fn test_light_environment_setters() {
    let mut env = LightEnvironment::new();

    // Test sun vector
    let v = Vector3::new(1.0, 1.0, 1.0);
    env.set_sun_vector(v);
    
    // Wektor powinien byc znormalizowany, dlugosc 1
    // x = y = z = 1/sqrt(3) ~ 0.57735
    let expected = 1.0 / 3.0_f32.sqrt();
    assert!((env.sun_vector.x - expected).abs() < 1e-5);
    assert!((env.sun_vector.y - expected).abs() < 1e-5);
    assert!((env.sun_vector.z - expected).abs() < 1e-5);

    // Test directional light color
    let light_color = Color::new(1.0, 0.9, 0.8, 1.0);
    env.set_directional_light_color(light_color);
    assert_eq!(env.directional_light_color.r, 1.0);
    assert_eq!(env.directional_light_color.g, 0.9);
    assert_eq!(env.directional_light_color.b, 0.8);
    assert_eq!(env.directional_light_color.a, 1.0);

    // Test ambient color
    let ambient_color = Color::new(0.2, 0.2, 0.2, 1.0);
    env.set_ambient_color(ambient_color);
    assert_eq!(env.ambient_color.r, 0.2);
    assert_eq!(env.ambient_color.g, 0.2);
    assert_eq!(env.ambient_color.b, 0.2);
    assert_eq!(env.ambient_color.a, 1.0);

    // Test fog parameters
    let fog_color = Color::new(0.5, 0.5, 0.5, 1.0);
    let fog = FogParameters::new(true, fog_color, 10.0, 100.0);
    env.set_fog_parameters(fog);
    assert!(env.fog_parameters.enabled);
    assert_eq!(env.fog_parameters.color.r, 0.5);
    assert_eq!(env.fog_parameters.near, 10.0);
    assert_eq!(env.fog_parameters.far, 100.0);
}
