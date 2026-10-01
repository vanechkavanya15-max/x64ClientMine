use m2_client::wgpu_particle_billboard_renderer::{
    get_blend_state, BillboardType, ParticleBillboardRenderer, BlendType,
};
use glam::{Vec2, Vec3};
use std::f32::consts::PI;

#[test]
fn test_blend_state_additive() {
    // 2 is ONE in D3D9 / C++
    let blend = get_blend_state(2, 2);
    assert_eq!(blend.src_blend, BlendType::One);
    assert_eq!(blend.dest_blend, BlendType::One);
    assert!(blend.is_additive());
    assert!(!blend.is_alpha_blend());
}

#[test]
fn test_blend_state_alpha() {
    // 5 is SRCALPHA, 6 is INVSRCALPHA
    let blend = get_blend_state(5, 6);
    assert_eq!(blend.src_blend, BlendType::SrcAlpha);
    assert_eq!(blend.dest_blend, BlendType::InvSrcAlpha);
    assert!(blend.is_alpha_blend());
    assert!(!blend.is_additive());
}

#[test]
fn test_billboard_type_lie() {
    let position = Vec3::new(10.0, 10.0, 0.0);
    let scale = Vec2::new(1.0, 1.0);
    let half_size = Vec2::new(1.0, 1.0);
    let rotation = 90.0; // 90 degrees

    let vertices = ParticleBillboardRenderer::calculate_billboard_vertices(
        position,
        scale,
        half_size,
        rotation,
        BillboardType::Lie,
        Vec3::ZERO,
        Vec3::ZERO,
        Vec3::ZERO,
    );

    // With 90 deg rotation:
    // f_cos = 0, f_sin = 1
    // v3_up = (0, -1, 0)
    // v3_cross = (1, 0, 0)
    // Up gets scaled by (half_size.y * scale.y) = 1.0 -> (0, -1, 0)
    // Cross gets scaled by -(half_size.x * scale.x) = -1.0 -> (-1, 0, 0)
    
    // v0: pos - up + cross = (10, 10, 0) - (0, -1, 0) + (-1, 0, 0) = (9, 11, 0)
    let eps = 1e-4;
    assert!((vertices[0].position[0] - 9.0).abs() < eps);
    assert!((vertices[0].position[1] - 11.0).abs() < eps);
    assert!((vertices[0].position[2] - 0.0).abs() < eps);
}

#[test]
fn test_billboard_type_y() {
    let position = Vec3::new(0.0, 0.0, 0.0);
    let scale = Vec2::new(1.0, 1.0);
    let half_size = Vec2::new(1.0, 1.0);
    let rotation = 0.0;
    
    let camera_view = Vec3::new(1.0, 0.0, 0.0);

    let vertices = ParticleBillboardRenderer::calculate_billboard_vertices(
        position,
        scale,
        half_size,
        rotation,
        BillboardType::Y,
        Vec3::ZERO,
        Vec3::ZERO,
        camera_view,
    );

    // v3_up = (0, 0, 1)
    // v3_up.x * view.y - v3_up.y * view.x = 0*0 - 0*1 = 0
    // cross = up(0,0,1) x view_xy(1,0,0) = (0, 1, 0)
    // after scaling: up = (0, 0, 1), cross = (0, -1, 0)
    
    // v0: pos - up + cross = (0, -1, -1)
    let eps = 1e-4;
    assert!((vertices[0].position[0] - 0.0).abs() < eps);
    assert!((vertices[0].position[1] - -1.0).abs() < eps);
    assert!((vertices[0].position[2] - -1.0).abs() < eps);
}

#[test]
fn test_billboard_type_all_no_rot() {
    let position = Vec3::new(0.0, 0.0, 0.0);
    let scale = Vec2::new(1.0, 1.0);
    let half_size = Vec2::new(2.0, 2.0);
    let rotation = 0.0;
    
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    let camera_cross = Vec3::new(1.0, 0.0, 0.0);
    let camera_view = Vec3::new(0.0, 0.0, 1.0);

    let vertices = ParticleBillboardRenderer::calculate_billboard_vertices(
        position,
        scale,
        half_size,
        rotation,
        BillboardType::All,
        camera_up,
        camera_cross,
        camera_view,
    );

    // v3_up = -cross = (-1, 0, 0) -> up becomes (-2, 0, 0)
    // v3_cross = up = (0, 1, 0) -> cross becomes (0, -2, 0)
    
    // v0 = pos - up + cross = (2, -2, 0)
    let eps = 1e-4;
    assert!((vertices[0].position[0] - 2.0).abs() < eps);
    assert!((vertices[0].position[1] - -2.0).abs() < eps);
    assert!((vertices[0].position[2] - 0.0).abs() < eps);
}

#[test]
fn test_get_time_event_blend_value_color() {
    let events = vec![
        (0.0, [1.0, 0.0, 0.0, 1.0]), // Red at 0
        (1.0, [0.0, 1.0, 0.0, 0.5]), // Green with 0.5 alpha at 1
    ];
    
    // Before start
    let c = ParticleBillboardRenderer::get_time_event_blend_value_color(-1.0, &events);
    assert_eq!(c, [1.0, 0.0, 0.0, 1.0]);

    // Middle
    let c = ParticleBillboardRenderer::get_time_event_blend_value_color(0.5, &events);
    assert_eq!(c, [0.5, 0.5, 0.0, 0.75]);
    
    // End
    let c = ParticleBillboardRenderer::get_time_event_blend_value_color(1.5, &events);
    assert_eq!(c, [0.0, 1.0, 0.0, 0.5]);
}

#[test]
fn test_get_time_event_blend_value_color_empty() {
    let events: Vec<(f32, [f32; 4])> = vec![];
    let c = ParticleBillboardRenderer::get_time_event_blend_value_color(0.5, &events);
    assert_eq!(c, [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn test_get_time_event_blend_value_color_single() {
    let events = vec![(0.5, [1.0, 1.0, 0.0, 1.0])];
    let c = ParticleBillboardRenderer::get_time_event_blend_value_color(0.1, &events);
    assert_eq!(c, [1.0, 1.0, 0.0, 1.0]);
    
    let c = ParticleBillboardRenderer::get_time_event_blend_value_color(0.9, &events);
    assert_eq!(c, [1.0, 1.0, 0.0, 1.0]);
}