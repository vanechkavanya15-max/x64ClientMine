use m2_client_render::pipeline_skybox::{
    CloudLayer, Color, PipelineSkybox, SkyColorGradient, SkyDomeMesh,
};

#[test]
fn test_color_lerp() {
    let c1 = Color::new(0.0, 0.0, 0.0, 1.0);
    let c2 = Color::new(1.0, 1.0, 1.0, 1.0);

    let mid = c1.lerp(&c2, 0.5);
    assert!((mid.r - 0.5).abs() < f32::EPSILON);
    assert!((mid.g - 0.5).abs() < f32::EPSILON);
    assert!((mid.b - 0.5).abs() < f32::EPSILON);
    assert!((mid.a - 1.0).abs() < f32::EPSILON);

    let start = c1.lerp(&c2, -1.0);
    assert_eq!(start, c1);

    let end = c1.lerp(&c2, 2.0);
    assert_eq!(end, c2);
}

#[test]
fn test_sky_color_gradient() {
    let mut gradient = SkyColorGradient::new();
    let black = Color::new(0.0, 0.0, 0.0, 1.0);
    let white = Color::new(1.0, 1.0, 1.0, 1.0);

    gradient.add_keyframe(0.0, black);
    gradient.add_keyframe(12.0, white);
    gradient.add_keyframe(24.0, black);

    let c_midday = gradient.get_color_at_time(12.0);
    assert_eq!(c_midday, white);

    let c_morning = gradient.get_color_at_time(6.0);
    assert!((c_morning.r - 0.5).abs() < f32::EPSILON);

    let c_midnight = gradient.get_color_at_time(0.0);
    assert_eq!(c_midnight, black);
    
    // Sprawdzenie przejscia przez granice 24.0 -> 0.0
    let mut gradient2 = SkyColorGradient::new();
    let blue = Color::new(0.0, 0.0, 1.0, 1.0);
    let red = Color::new(1.0, 0.0, 0.0, 1.0);
    gradient2.add_keyframe(22.0, blue);
    gradient2.add_keyframe(2.0, red); // Przez polnoc
    
    // W polowie miedzy 22:00 a 2:00 jest 0:00
    let c_midnight2 = gradient2.get_color_at_time(0.0);
    assert!((c_midnight2.r - 0.5).abs() < f32::EPSILON);
    assert!((c_midnight2.b - 0.5).abs() < f32::EPSILON);
}

#[test]
fn test_cloud_layer_rotation() {
    let mut cloud = CloudLayer::new(1, 10.0); // Predkosc: 10 stopni na sekunde

    cloud.update(1.0);
    assert!((cloud.rotation - 10.0).abs() < f32::EPSILON);

    cloud.update(36.0); // Osiagamy 370.0
    assert!((cloud.rotation - 10.0).abs() < 0.001); // Normalizacja do 10.0

    cloud.rotation_speed = -10.0;
    cloud.update(2.0); // 10.0 - 20.0 = -10.0 -> 350.0
    assert!((cloud.rotation - 350.0).abs() < 0.001);
}

#[test]
fn test_skydome_mesh_generation() {
    let rings = 16;
    let segments = 32;
    let mesh = SkyDomeMesh::generate(100.0, rings, segments);

    // Oczekiwana liczba wierzcholkow: (rings + 1) * (segments + 1)
    let expected_vertices = (rings + 1) * (segments + 1);
    assert_eq!(mesh.vertices.len(), expected_vertices as usize);

    // Oczekiwana liczba indeksow: rings * segments * 2 (trojkaty) * 3 (indeksy)
    let expected_indices = rings * segments * 6;
    assert_eq!(mesh.indices.len(), expected_indices as usize);
}

#[test]
fn test_pipeline_skybox_integration() {
    let mut pipeline = PipelineSkybox::new();
    assert!(pipeline.is_ready());

    let mut gradient = SkyColorGradient::new();
    let day_color = Color::new(0.5, 0.7, 1.0, 1.0);
    gradient.add_keyframe(12.0, day_color);
    pipeline.color_gradient = gradient;

    pipeline.set_time_of_day(12.0);
    assert_eq!(pipeline.current_color, day_color);
    assert!((pipeline.time_of_day - 12.0).abs() < f32::EPSILON);

    pipeline.set_time_of_day(25.0); // Normalizacja do 1.0
    assert!((pipeline.time_of_day - 1.0).abs() < f32::EPSILON);

    let cloud = CloudLayer::new(100, 5.0);
    pipeline.add_cloud_layer(cloud);
    assert_eq!(pipeline.cloud_layers.len(), 1);

    pipeline.update(2.0);
    assert!((pipeline.cloud_layers[0].rotation - 10.0).abs() < f32::EPSILON);
}
