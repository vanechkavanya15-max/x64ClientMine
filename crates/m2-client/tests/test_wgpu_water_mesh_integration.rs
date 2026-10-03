#[path = "../src/wgpu_water_mesh_integration.rs"]
mod wgpu_water_mesh_integration;

use m2_client::terrain_height_patch::{RawHeightMap, CELL_SIZE};
use wgpu_water_mesh_integration::{
    calculate_water_foam, calculate_water_fresnel, WaterMeshBuilder, WaterVertex,
};

#[test]
fn test_wgpu_water_mesh_integration_is_accessible() {
    let _v = std::mem::size_of::<WaterVertex>();
    assert!(_v > 0);
}

#[test]
fn test_calculate_water_fresnel_straight() {
    let normal = glam::Vec3::new(0.0, 0.0, 1.0);
    // Looking straight down: cos_theta = 1.0 -> fresnel = R0
    let view_dir_straight = glam::Vec3::new(0.0, 0.0, 1.0);
    let r0 = 0.02;
    let f1 = calculate_water_fresnel(normal, view_dir_straight, r0);
    assert!((f1 - 0.02).abs() < 1e-6);
}

#[test]
fn test_calculate_water_fresnel_glancing() {
    let normal = glam::Vec3::new(0.0, 0.0, 1.0);
    // Looking perpendicular (glancing angle): cos_theta = 0.0 -> fresnel = 1.0
    let view_dir_glancing = glam::Vec3::new(1.0, 0.0, 0.0);
    let r0 = 0.02;
    let f2 = calculate_water_fresnel(normal, view_dir_glancing, r0);
    assert!((f2 - 1.0).abs() < 1e-6);
}

#[test]
fn test_calculate_water_foam_levels() {
    // Deep water (alpha > threshold) -> 0 foam
    let foam1 = calculate_water_foam(0.5, 0.0, 0.3);
    assert_eq!(foam1, 0.0);

    // Near shore (alpha < threshold) -> foam generated based on time and depth
    let foam2 = calculate_water_foam(0.1, 0.0, 0.3); // alpha = 0.1, depth factor = 1.0 - (0.1/0.3) = 0.666
    assert!((foam2 - 0.6666667).abs() < 1e-5);

    // Exactly at shore (alpha = 0.0) -> max depth factor = 1.0
    let foam3 = calculate_water_foam(0.0, std::f32::consts::PI / 4.0, 0.3);
    assert!((foam3 - 1.0).abs() < 1e-6);
}

#[test]
fn test_water_mesh_builder_empty() {
    let height_map = RawHeightMap::new(17, 17, vec![0.0; 17 * 17]);
    let water_map = vec![0xFF; 17 * 17];
    let water_heights = vec![-1; 256];

    let vertices = WaterMeshBuilder::build_water_vertices(
        0,
        0,
        &height_map,
        &water_map,
        &water_heights,
        1.0,
    );
    assert_eq!(vertices.len(), 0);
}

#[test]
fn test_water_mesh_builder_with_water() {
    let mut heights = vec![0.0; 17 * 17];
    heights[0] = 100.0; // Raise terrain at 0,0

    let height_map = RawHeightMap::new(17, 17, heights);
    let mut water_map = vec![0xFF; 17 * 17];
    water_map[0] = 0; // Water at 0,0 uses index 0

    let mut water_heights = vec![-1; 256];
    water_heights[0] = 200; // Water level is 200

    let vertices = WaterMeshBuilder::build_water_vertices(
        0,
        0,
        &height_map,
        &water_map,
        &water_heights,
        1.0,
    );

    // 1 cell has water -> 6 vertices
    assert_eq!(vertices.len(), 6);

    // First vertex (top-left) at 0, 0 should have position [0, 0, 200]
    let v0 = vertices[0];
    assert_eq!(v0.position, [0.0, 0.0, 200.0]);

    // Check diffuse logic, diff at 0,0 is 200-100 = 100
    // alpha0 = (100 / 400 * 255) = 63 = 0x3F
    // diffuse = 0x3F0000FF
    assert_eq!(v0.diffuse, 0x3F0000FF);
}

#[test]
fn test_water_vertex_desc() {
    let desc = WaterVertex::desc();
    assert_eq!(
        desc.array_stride,
        std::mem::size_of::<WaterVertex>() as wgpu::BufferAddress
    );
    assert_eq!(desc.step_mode, wgpu::VertexStepMode::Vertex);
    assert_eq!(desc.attributes.len(), 2);
    assert_eq!(desc.attributes[0].format, wgpu::VertexFormat::Float32x3);
    assert_eq!(desc.attributes[1].format, wgpu::VertexFormat::Uint32);
}
