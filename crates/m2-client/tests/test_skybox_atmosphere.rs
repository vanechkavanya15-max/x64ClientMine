use m2_client::wgpu_skybox_atmosphere_renderer::{SkyboxAtmosphereRenderer, AtmosphereState};
use glam::{Mat4, Vec3};

#[test]
fn test_skybox_matrix_translation_blocked() {
    let mut renderer = SkyboxAtmosphereRenderer::new();
    
    // Create a view matrix with some translation
    let view = Mat4::look_at_rh(
        Vec3::new(10.0, 20.0, 30.0), // eye position (translation)
        Vec3::new(10.0, 20.0, 29.0), // looking down -Z relative to eye
        Vec3::new(0.0, 1.0, 0.0),    // up
    );
    
    // Create a simple projection matrix
    let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    
    renderer.update_camera(view, proj);
    
    // The w_axis (translation) of the view matrix should have been zeroed out before multiplication.
    // If we multiply the zero translation vector (0,0,0,1) with the resulting view_proj, 
    // we should get a projection of (0,0,0,1) instead of the translated coordinate.
    // The skybox must remain centered around the origin.
    
    let origin = glam::Vec4::new(0.0, 0.0, 0.0, 1.0);
    let projected_origin = renderer.view_proj_matrix * origin;
    
    // The view_proj matrix must map the world origin exactly to the camera's local origin.
    // Because the translation was zeroed in the view matrix, world (0,0,0,1) stays at local (0,0,0,1).
    // And multiplied by proj, x and y are 0.
    assert_eq!(projected_origin.x, 0.0);
    assert_eq!(projected_origin.y, 0.0);
}

#[test]
fn test_atmosphere_change_to_night_blending() {
    let mut renderer = SkyboxAtmosphereRenderer::new();
    renderer.transition_duration = 2.0;
    
    assert_eq!(renderer.state, AtmosphereState::Day);
    
    // Change to night starts transition
    renderer.change_to_night();
    assert_eq!(renderer.state, AtmosphereState::TransitioningToNight);
    assert_eq!(renderer.transition_progress, 0.0);
    
    // Halfway through transition
    renderer.update_atmosphere_with_delta(1.0);
    assert_eq!(renderer.state, AtmosphereState::TransitioningToNight);
    assert_eq!(renderer.transition_progress, 0.5);
    
    // Values should be exactly halfway between day and night
    let expected_ambient = [
        (renderer.day_ambient[0] + renderer.night_ambient[0]) / 2.0,
        (renderer.day_ambient[1] + renderer.night_ambient[1]) / 2.0,
        (renderer.day_ambient[2] + renderer.night_ambient[2]) / 2.0,
        1.0
    ];
    
    
    assert!((renderer.current_ambient[0] - expected_ambient[0]).abs() < 1e-6);
    assert!((renderer.current_ambient[1] - expected_ambient[1]).abs() < 1e-6);
    assert!((renderer.current_ambient[2] - expected_ambient[2]).abs() < 1e-6);
    assert!((renderer.current_ambient[3] - expected_ambient[3]).abs() < 1e-6);

    
    // Complete transition
    renderer.update_atmosphere_with_delta(1.5); // Overshoot
    assert_eq!(renderer.state, AtmosphereState::Night);
    assert_eq!(renderer.transition_progress, 1.0);
    
    assert!((renderer.current_ambient[0] - renderer.night_ambient[0]).abs() < 1e-6);
    assert!((renderer.current_ambient[1] - renderer.night_ambient[1]).abs() < 1e-6);
    assert!((renderer.current_ambient[2] - renderer.night_ambient[2]).abs() < 1e-6);
    assert!((renderer.current_ambient[3] - renderer.night_ambient[3]).abs() < 1e-6);

}

#[test]
fn test_atmosphere_change_to_day_blending() {
    let mut renderer = SkyboxAtmosphereRenderer::new();
    renderer.transition_duration = 2.0;
    
    // Force state to night
    renderer.state = AtmosphereState::Night;
    
    renderer.change_to_day();
    assert_eq!(renderer.state, AtmosphereState::TransitioningToDay);
    assert_eq!(renderer.transition_progress, 0.0);
    
    // Update a little bit
    renderer.update_atmosphere_with_delta(0.5);
    assert_eq!(renderer.transition_progress, 0.25);
    
    // A quarter of the way from night to day
    let expected_fog = [
        renderer.night_fog[0] + (renderer.day_fog[0] - renderer.night_fog[0]) * 0.25,
        renderer.night_fog[1] + (renderer.day_fog[1] - renderer.night_fog[1]) * 0.25,
        renderer.night_fog[2] + (renderer.day_fog[2] - renderer.night_fog[2]) * 0.25,
        1.0
    ];
    
    
    assert!((renderer.current_fog[0] - expected_fog[0]).abs() < 1e-6);
    assert!((renderer.current_fog[1] - expected_fog[1]).abs() < 1e-6);
    assert!((renderer.current_fog[2] - expected_fog[2]).abs() < 1e-6);
    assert!((renderer.current_fog[3] - expected_fog[3]).abs() < 1e-6);

    
    // Finish transition
    renderer.update_atmosphere_with_delta(2.0);
    assert_eq!(renderer.state, AtmosphereState::Day);
    assert_eq!(renderer.transition_progress, 1.0);
    
    assert!((renderer.current_fog[0] - renderer.day_fog[0]).abs() < 1e-6);
    assert!((renderer.current_fog[1] - renderer.day_fog[1]).abs() < 1e-6);
    assert!((renderer.current_fog[2] - renderer.day_fog[2]).abs() < 1e-6);
    assert!((renderer.current_fog[3] - renderer.day_fog[3]).abs() < 1e-6);

}

#[test]
fn test_atmosphere_color_interpolation() {
    let c1 = [0.0, 0.5, 1.0, 1.0];
    let c2 = [1.0, 0.5, 0.0, 0.0];
    
    let res = SkyboxAtmosphereRenderer::interpolate_color(c1, c2, 0.5);
    assert_eq!(res, [0.5, 0.5, 0.5, 0.5]);
    
    // Clamping checks
    let res_low = SkyboxAtmosphereRenderer::interpolate_color(c1, c2, -0.5);
    assert_eq!(res_low, c1);
    
    let res_high = SkyboxAtmosphereRenderer::interpolate_color(c1, c2, 1.5);
    assert_eq!(res_high, c2);
}
