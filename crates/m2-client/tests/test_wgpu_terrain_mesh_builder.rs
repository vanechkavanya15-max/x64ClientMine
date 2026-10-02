use m2_client::terrain_height_patch::{RawHeightMap, CELL_SIZE, TERRAIN_PATCHSIZE};
use m2_client::wgpu_terrain_mesh_builder::{TerrainMeshBuilder, TerrainVertex};

#[test]
fn test_terrain_vertex_layout() {
    let size = std::mem::size_of::<TerrainVertex>();
    let expected_size = std::mem::size_of::<[f32; 3]>() // position
        + std::mem::size_of::<[f32; 3]>() // normal
        + std::mem::size_of::<[f32; 2]>() // tex_coords
        + std::mem::size_of::<[f32; 4]>(); // splat_weights
    
    assert_eq!(size, expected_size);

    let layout = TerrainVertex::desc();
    assert_eq!(layout.array_stride, expected_size as wgpu::BufferAddress);
    assert_eq!(layout.attributes.len(), 4);
    assert_eq!(layout.attributes[0].format, wgpu::VertexFormat::Float32x3);
    assert_eq!(layout.attributes[1].format, wgpu::VertexFormat::Float32x3);
    assert_eq!(layout.attributes[2].format, wgpu::VertexFormat::Float32x2);
    assert_eq!(layout.attributes[3].format, wgpu::VertexFormat::Float32x4);
}

#[test]
fn test_normal_calculation_flat() {
    let width = 3;
    let height = 3;
    let heights = vec![0.0; width * height];
    let height_map = RawHeightMap::new(width, height, heights);

    let normal = TerrainMeshBuilder::calculate_normal(1, 1, 0, 0, &height_map);
    assert_eq!(normal, [0.0, 0.0, 1.0]);
}

#[test]
fn test_normal_calculation_slope() {
    let width = 3;
    let height = 3;
    // Slope upward along X axis
    let heights = vec![
        0.0, 100.0, 200.0,
        0.0, 100.0, 200.0,
        0.0, 100.0, 200.0,
    ];
    let height_map = RawHeightMap::new(width, height, heights);

    let normal = TerrainMeshBuilder::calculate_normal(1, 1, 0, 0, &height_map);
    
    // dzdx = (200.0 - 0.0) / (2.0 * CELL_SIZE) = 200.0 / 400.0 = 0.5
    // dzdy = (100.0 - 100.0) / (2.0 * CELL_SIZE) = 0.0
    // nx = -0.5, ny = 0.0, nz = 1.0
    // length = sqrt(0.25 + 1.0) = sqrt(1.25) = 1.11803398875
    // normalized nx = -0.5 / 1.11803398875 = -0.4472135955
    let expected_nx = -0.4472135955;
    let expected_ny = 0.0;
    let expected_nz = 1.0 / 1.25_f32.sqrt();
    
    assert!((normal[0] - expected_nx).abs() < 1e-5);
    assert!((normal[1] - expected_ny).abs() < 1e-5);
    assert!((normal[2] - expected_nz).abs() < 1e-5);
}

#[test]
fn test_build_patch_vertices() {
    let width = 17; // TERRAIN_PATCHSIZE + 1
    let height = 17;
    let heights = vec![0.0; width * height];
    let height_map = RawHeightMap::new(width, height, heights);

    let vertices = TerrainMeshBuilder::build_patch_vertices(0, 0, &height_map);
    assert_eq!(vertices.len(), (TERRAIN_PATCHSIZE + 1) * (TERRAIN_PATCHSIZE + 1));

    // Check first vertex (0, 0)
    let v0 = &vertices[0];
    assert_eq!(v0.position, [0.0, 0.0, 0.0]);
    assert_eq!(v0.tex_coords, [0.0, 0.0]);
    assert_eq!(v0.splat_weights, [1.0, 0.0, 0.0, 0.0]);

    // Check last vertex (16, 16)
    let vlast = &vertices.last().unwrap();
    assert_eq!(vlast.position, [TERRAIN_PATCHSIZE as f32 * CELL_SIZE, TERRAIN_PATCHSIZE as f32 * CELL_SIZE, 0.0]);
    assert_eq!(vlast.tex_coords, [1.0, 1.0]);
}

#[test]
fn test_lod0_indexing() {
    let indices = TerrainMeshBuilder::build_patch_indices(0);
    // TERRAIN_PATCHSIZE is 16. Each row has 16 squares, each square has 2 triangles (6 indices)
    let expected_count = 16 * 16 * 6;
    assert_eq!(indices.len(), expected_count);

    // First square (0,0) CCW winding
    let verts_per_row = (TERRAIN_PATCHSIZE + 1) as u16;
    let expected_first_square = [
        0, 1, verts_per_row + 1, // Triangle 1
        0, verts_per_row + 1, verts_per_row // Triangle 2
    ];
    assert_eq!(&indices[0..6], &expected_first_square);
}

#[test]
fn test_lod1_indexing() {
    let indices = TerrainMeshBuilder::build_patch_indices(1);
    // LOD 1 steps by 2. Grid size 16x16 -> 8x8 squares.
    let expected_count = 8 * 8 * 6;
    assert_eq!(indices.len(), expected_count);

    let verts_per_row = (TERRAIN_PATCHSIZE + 1) as u16;
    let step = 2;
    let current = 0;
    let next_x = current + step;
    let next_y = step * verts_per_row + current;
    let next_xy = next_y + step;

    let expected_first_square = [
        current, next_x, next_xy,
        current, next_xy, next_y
    ];
    assert_eq!(&indices[0..6], &expected_first_square);
}

#[test]
fn test_lod2_indexing() {
    let indices = TerrainMeshBuilder::build_patch_indices(2);
    // LOD 2 steps by 4. Grid size 16x16 -> 4x4 squares.
    let expected_count = 4 * 4 * 6;
    assert_eq!(indices.len(), expected_count);

    let verts_per_row = (TERRAIN_PATCHSIZE + 1) as u16;
    let step = 4;
    let current = 0;
    let next_x = current + step;
    let next_y = step * verts_per_row + current;
    let next_xy = next_y + step;

    let expected_first_square = [
        current, next_x, next_xy,
        current, next_xy, next_y
    ];
    assert_eq!(&indices[0..6], &expected_first_square);
}
