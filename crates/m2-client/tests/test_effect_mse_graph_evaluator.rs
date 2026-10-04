use m2_client::effect_mse_graph_evaluator::*;
use glam::{Vec2, Vec3, Vec4};

#[test]
fn test_get_time_event_blend_value_f32() {
    let graph = vec![
        TimeEvent { time: 0.0, value: 0.0 },
        TimeEvent { time: 0.5, value: 10.0 },
        TimeEvent { time: 1.0, value: 0.0 },
    ];
    
    // Test boundaries
    assert_eq!(get_time_event_blend_value(-0.5, &graph), 0.0);
    assert_eq!(get_time_event_blend_value(0.0, &graph), 0.0);
    assert_eq!(get_time_event_blend_value(1.0, &graph), 0.0);
    assert_eq!(get_time_event_blend_value(1.5, &graph), 0.0);
    
    // Test interpolation
    assert_eq!(get_time_event_blend_value(0.25, &graph), 5.0);
    assert_eq!(get_time_event_blend_value(0.5, &graph), 10.0);
    assert_eq!(get_time_event_blend_value(0.75, &graph), 5.0);
}

#[test]
fn test_get_time_event_blend_value_color() {
    let graph = vec![
        TimeEvent { time: 0.0, value: Vec4::new(1.0, 0.0, 0.0, 1.0) }, // Red
        TimeEvent { time: 1.0, value: Vec4::new(0.0, 0.0, 1.0, 1.0) }, // Blue
    ];
    
    let mid = get_time_event_blend_value(0.5, &graph);
    assert_eq!(mid, Vec4::new(0.5, 0.0, 0.5, 1.0)); // Purple
}

#[test]
fn test_particle_system_data_parse() {
    let script = r#"
        maxemissioncount 100
        cyclelength 2.5
        timeeventsizex 0.0 1.0 1.0 2.0
        billboardtype 1
        timeeventcolor 0.0 1.0 0.0 0.0 1.0 1.0 0.0 0.0 1.0 0.5
    "#;
    
    let data = ParticleSystemData::parse(script).unwrap();
    
    assert_eq!(data.emitter_property.max_emission_count, 100);
    assert_eq!(data.emitter_property.cycle_length, 2.5);
    
    assert_eq!(data.emitter_property.time_event_size_x.len(), 2);
    assert_eq!(data.emitter_property.time_event_size_x[0].time, 0.0);
    assert_eq!(data.emitter_property.time_event_size_x[0].value, 1.0);
    assert_eq!(data.emitter_property.time_event_size_x[1].time, 1.0);
    assert_eq!(data.emitter_property.time_event_size_x[1].value, 2.0);
    
    assert_eq!(data.particle_property.billboard_type, BillboardType::All);
    
    assert_eq!(data.particle_property.time_event_color.len(), 2);
    assert_eq!(data.particle_property.time_event_color[0].time, 0.0);
    assert_eq!(data.particle_property.time_event_color[0].value, Vec4::new(1.0, 0.0, 0.0, 1.0));
    assert_eq!(data.particle_property.time_event_color[1].time, 1.0);
    assert_eq!(data.particle_property.time_event_color[1].value, Vec4::new(0.0, 0.0, 1.0, 0.5));
}

#[test]
fn test_particle_physics_and_billboard_lifecycle() {
    let mut position = Vec3::new(0.0, 0.0, 10.0);
    let mut velocity = Vec3::new(10.0, 0.0, 0.0);
    let gravity = 9.8;
    let air_res = 0.1;
    let elapsed = 0.1;
    
    // Step physics
    m2_particle_physics_step(&mut position, &mut velocity, elapsed, gravity, air_res);
    
    // Check gravity affected Z velocity
    assert!(velocity.z < 0.0);
    // Check air resistance reduced X velocity
    assert!(velocity.x < 10.0);
    
    let camera_view = glam::Mat4::IDENTITY;
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    let camera_cross = Vec3::new(1.0, 0.0, 0.0);
    
    let quads = calculate_billboard_transform(
        position,
        Vec3::ZERO,
        45.0,
        Vec2::new(1.0, 1.0),
        Vec2::new(2.0, 2.0),
        BillboardType::All,
        camera_view,
        camera_up,
        camera_cross,
        false
    );
    
    assert_eq!(quads.len(), 4);
    assert_ne!(quads[0].position, position);
    assert_eq!(quads[0].tex_coord, Vec2::new(0.0, 1.0));
    assert_eq!(quads[3].tex_coord, Vec2::new(1.0, 0.0));
}
