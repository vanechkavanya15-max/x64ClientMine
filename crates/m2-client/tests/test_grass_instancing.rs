use m2_client::wgpu_grass_billboard_instancing::{
    generate_grass_mesh, generate_grass_instances, get_grass_shader_wgsl,
    GrassVertex, GrassInstance,
};

#[test]
fn test_grass_mesh_generation() {
    let width = 10.0;
    let height = 15.0;
    let mesh = generate_grass_mesh(width, height);

    // 3 quads = 6 triangles = 18 vertices
    assert_eq!(mesh.len(), 18, "Grass mesh should contain 18 vertices (3 intersecting quads).");

    let mut wind_1_count = 0;
    let mut wind_0_count = 0;

    for vertex in mesh.iter() {
        if vertex.wind_weight == 1.0 {
            wind_1_count += 1;
        } else if vertex.wind_weight == 0.0 {
            wind_0_count += 1;
        }
    }

    // Polowa wierzcholkow na dole (wind_weight=0) i polowa na gorze (wind_weight=1)
    assert_eq!(wind_0_count, 9, "Bottom vertices should have wind_weight = 0.0");
    assert_eq!(wind_1_count, 9, "Top vertices should have wind_weight = 1.0");
}

#[test]
fn test_grass_instance_generation() {
    let patch_x = 100.0;
    let patch_y = 200.0;
    let width = 50.0;
    let height = 50.0;
    let count = 1000;
    let seed = 12345;

    let instances = generate_grass_instances(patch_x, patch_y, width, height, count, seed);

    assert_eq!(instances.len(), count, "Should generate exactly {} instances.", count);

    for instance in instances.iter() {
        assert!(
            instance.position[0] >= patch_x && instance.position[0] <= patch_x + width,
            "Instance X position out of bounds: {}", instance.position[0]
        );
        assert!(
            instance.position[1] >= patch_y && instance.position[1] <= patch_y + height,
            "Instance Y position out of bounds: {}", instance.position[1]
        );
        assert!(
            instance.scale >= 0.8 && instance.scale <= 1.2,
            "Instance scale out of bounds: {}", instance.scale
        );
        assert!(
            instance.rotation >= 0.0 && instance.rotation <= std::f32::consts::PI * 2.0,
            "Instance rotation out of bounds: {}", instance.rotation
        );
    }
}

#[test]
fn test_grass_vertex_buffer_layouts() {
    let vertex_desc = GrassVertex::desc();
    assert_eq!(vertex_desc.array_stride, std::mem::size_of::<GrassVertex>() as wgpu::BufferAddress);
    assert_eq!(vertex_desc.step_mode, wgpu::VertexStepMode::Vertex);
    assert_eq!(vertex_desc.attributes.len(), 3);
    assert_eq!(vertex_desc.attributes[0].shader_location, 0); // position
    assert_eq!(vertex_desc.attributes[1].shader_location, 1); // tex_coord
    assert_eq!(vertex_desc.attributes[2].shader_location, 2); // wind_weight

    let instance_desc = GrassInstance::desc();
    assert_eq!(instance_desc.array_stride, std::mem::size_of::<GrassInstance>() as wgpu::BufferAddress);
    assert_eq!(instance_desc.step_mode, wgpu::VertexStepMode::Instance);
    assert_eq!(instance_desc.attributes.len(), 3);
    assert_eq!(instance_desc.attributes[0].shader_location, 3); // position
    assert_eq!(instance_desc.attributes[1].shader_location, 4); // scale
    assert_eq!(instance_desc.attributes[2].shader_location, 5); // rotation
}

#[test]
fn test_grass_wgsl_shader_contains_wind_logic() {
    let shader = get_grass_shader_wgsl();

    // Weryfikacja efektu wiatru w shaderze
    assert!(shader.contains("sin("), "Shader should contain sin() function for wind effect.");
    assert!(shader.contains("cos("), "Shader should contain cos() function for wind effect.");
    assert!(shader.contains("camera.time"), "Shader should use camera.time for animation.");
    assert!(shader.contains("model.wind_weight"), "Shader should apply wind weight from vertex data.");
    assert!(shader.contains("discard;"), "Shader should contain alpha discarding logic.");
}
