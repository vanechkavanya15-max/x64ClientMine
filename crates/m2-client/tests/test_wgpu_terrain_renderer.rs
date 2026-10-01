use m2_client::terrain_height_patch::{RawHeightMap, CELL_SIZE};
use m2_client::wgpu_terrain_renderer::{TerrainVertex, generate_terrain_mesh};
use std::mem;

#[test]
fn test_terrain_vertex_memory_layout() {
    assert_eq!(
        mem::size_of::<TerrainVertex>(),
        32,
        "TerrainVertex should be exactly 32 bytes (3*4 + 3*4 + 2*4)"
    );
    assert_eq!(
        mem::align_of::<TerrainVertex>(),
        4,
        "TerrainVertex should have an alignment of 4 bytes (f32)"
    );
}

#[test]
fn test_terrain_vertex_desc() {
    let desc = TerrainVertex::desc();
    assert_eq!(desc.array_stride, 32);
    assert_eq!(desc.step_mode, wgpu::VertexStepMode::Vertex);
    assert_eq!(desc.attributes.len(), 3);
    
    assert_eq!(desc.attributes[0].offset, 0);
    assert_eq!(desc.attributes[0].format, wgpu::VertexFormat::Float32x3);
    
    assert_eq!(desc.attributes[1].offset, 12);
    assert_eq!(desc.attributes[1].format, wgpu::VertexFormat::Float32x3);

    assert_eq!(desc.attributes[2].offset, 24);
    assert_eq!(desc.attributes[2].format, wgpu::VertexFormat::Float32x2);
}

#[test]
fn test_generate_terrain_mesh_empty() {
    let empty_map = RawHeightMap::new(0, 0, vec![]);
    let vertices = generate_terrain_mesh(&empty_map);
    assert!(vertices.is_empty(), "Mesh for empty map should be empty");

    let small_map = RawHeightMap::new(1, 1, vec![0.0]);
    let vertices2 = generate_terrain_mesh(&small_map);
    assert!(vertices2.is_empty(), "Mesh for 1x1 map (cannot form quad) should be empty");
}

#[test]
fn test_generate_terrain_mesh_flat_2x2() {
    let heights = vec![0.0, 0.0, 0.0, 0.0];
    let map = RawHeightMap::new(2, 2, heights);
    let vertices = generate_terrain_mesh(&map);
    
    // A 2x2 height map forms a single cell (1x1 square) which consists of 2 triangles -> 6 vertices.
    assert_eq!(vertices.len(), 6);
    
    // Check first triangle (v00, v10, v11)
    assert_eq!(vertices[0].position, [0.0, 0.0, 0.0]);
    assert_eq!(vertices[0].tex_coord, [0.0, 0.0]);
    assert_eq!(vertices[0].normal, [0.0, 0.0, 1.0]);

    assert_eq!(vertices[1].position, [CELL_SIZE, 0.0, 0.0]);
    assert_eq!(vertices[1].tex_coord, [1.0, 0.0]);

    assert_eq!(vertices[2].position, [CELL_SIZE, CELL_SIZE, 0.0]);
    assert_eq!(vertices[2].tex_coord, [1.0, 1.0]);

    // Check second triangle (v00, v11, v01)
    assert_eq!(vertices[3].position, [0.0, 0.0, 0.0]);
    assert_eq!(vertices[4].position, [CELL_SIZE, CELL_SIZE, 0.0]);
    assert_eq!(vertices[5].position, [0.0, CELL_SIZE, 0.0]);
    assert_eq!(vertices[5].tex_coord, [0.0, 1.0]);
}

#[test]
fn test_generate_terrain_mesh_heights() {
    // 2x2 map with distinct heights
    let heights = vec![
        10.0, 20.0,
        30.0, 40.0
    ];
    let map = RawHeightMap::new(2, 2, heights);
    let vertices = generate_terrain_mesh(&map);
    
    assert_eq!(vertices.len(), 6);
    
    // Triangle 1: (0,0), (1,0), (1,1)
    assert_eq!(vertices[0].position[2], 10.0);
    assert_eq!(vertices[1].position[2], 20.0);
    assert_eq!(vertices[2].position[2], 40.0);

    // Triangle 2: (0,0), (1,1), (0,1)
    assert_eq!(vertices[3].position[2], 10.0);
    assert_eq!(vertices[4].position[2], 40.0);
    assert_eq!(vertices[5].position[2], 30.0);
}

#[test]
fn test_generate_terrain_mesh_16x16_patch() {
    // A MapOutdoor typical terrain patch is 16x16 grid cells, meaning it requires 17x17 heights
    let mut heights = vec![0.0; 17 * 17];
    heights[0] = 50.0;
    
    let map = RawHeightMap::new(17, 17, heights);
    let vertices = generate_terrain_mesh(&map);
    
    // 16x16 squares, each 2 triangles, each 3 vertices -> 16 * 16 * 2 * 3 = 1536
    assert_eq!(vertices.len(), 1536);
    assert_eq!(vertices[0].position[2], 50.0);
}
