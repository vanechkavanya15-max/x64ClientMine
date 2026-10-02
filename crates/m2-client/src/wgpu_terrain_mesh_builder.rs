use crate::terrain_height_patch::{RawHeightMap, CELL_SIZE, TERRAIN_PATCHSIZE};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TerrainVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
    pub splat_weights: [f32; 4],
}

impl TerrainVertex {
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<TerrainVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() * 2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() * 2 + std::mem::size_of::<[f32; 2]>())
                        as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct TerrainMeshBuilder;

impl TerrainMeshBuilder {
    pub fn calculate_normal(
        x: usize,
        y: usize,
        start_x: usize,
        start_y: usize,
        height_map: &RawHeightMap,
    ) -> [f32; 3] {
        let global_x = start_x + x;
        let global_y = start_y + y;

        // Get heights of adjacent vertices for central difference
        let h_l = height_map.get_height(global_x.saturating_sub(1), global_y);
        let h_r = height_map.get_height(global_x + 1, global_y);
        let h_d = height_map.get_height(global_x, global_y.saturating_sub(1));
        let h_u = height_map.get_height(global_x, global_y + 1);

        // Grid spacing is CELL_SIZE
        // We use central difference:
        // dx = (h_r - h_l) / (2.0 * CELL_SIZE)
        // dy = (h_u - h_d) / (2.0 * CELL_SIZE)
        // But since we want the cross product directly, we can use the vectors:
        // vx = (2 * CELL_SIZE, 0, h_r - h_l)
        // vy = (0, 2 * CELL_SIZE, h_u - h_d)
        // normal = vx cross vy = (-2*CELL_SIZE*(h_r - h_l), -2*CELL_SIZE*(h_u - h_d), 4*CELL_SIZE*CELL_SIZE)
        // Then normalize.
        // Simplification for the normal direction pointing up (Z+):
        
        // C++ Metin2 engine uses a specific method. Here we implement a robust central diff.
        let dzdx = (h_r - h_l) / (2.0 * CELL_SIZE);
        let dzdy = (h_u - h_d) / (2.0 * CELL_SIZE);
        
        let nx = -dzdx;
        let ny = -dzdy;
        let nz = 1.0;
        
        let length = (nx * nx + ny * ny + nz * nz).sqrt();
        if length > 0.0 {
            [nx / length, ny / length, nz / length]
        } else {
            [0.0, 0.0, 1.0]
        }
    }

    pub fn build_patch_vertices(
        patch_x: usize,
        patch_y: usize,
        height_map: &RawHeightMap,
    ) -> Vec<TerrainVertex> {
        let start_x = patch_x * TERRAIN_PATCHSIZE;
        let start_y = patch_y * TERRAIN_PATCHSIZE;
        
        let num_vertices = (TERRAIN_PATCHSIZE + 1) * (TERRAIN_PATCHSIZE + 1);
        let mut vertices = Vec::with_capacity(num_vertices);

        for y in 0..=TERRAIN_PATCHSIZE {
            for x in 0..=TERRAIN_PATCHSIZE {
                let global_x = start_x + x;
                let global_y = start_y + y;

                let px = global_x as f32 * CELL_SIZE;
                let py = global_y as f32 * CELL_SIZE;
                let pz = height_map.get_height(global_x, global_y);

                let normal = Self::calculate_normal(x, y, start_x, start_y, height_map);

                // UVs mapped 0..1 over the patch
                let u = x as f32 / TERRAIN_PATCHSIZE as f32;
                let v = y as f32 / TERRAIN_PATCHSIZE as f32;

                vertices.push(TerrainVertex {
                    position: [px, py, pz],
                    normal,
                    tex_coords: [u, v],
                    // Default to 1st texture weight fully, rest 0
                    splat_weights: [1.0, 0.0, 0.0, 0.0],
                });
            }
        }

        vertices
    }

    pub fn build_patch_indices(lod_level: u32) -> Vec<u16> {
        // Calculate step based on LOD. LOD 0 = 1, LOD 1 = 2, LOD 2 = 4
        let step = match lod_level {
            0 => 1,
            1 => 2,
            _ => 4,
        };

        let mut indices = Vec::new();
        let verts_per_row = (TERRAIN_PATCHSIZE + 1) as u16;

        for y in (0..TERRAIN_PATCHSIZE).step_by(step) {
            for x in (0..TERRAIN_PATCHSIZE).step_by(step) {
                let current = (y as u16 * verts_per_row) + x as u16;
                let next_x = current + step as u16;
                let next_y = ((y + step) as u16 * verts_per_row) + x as u16;
                let next_xy = next_y + step as u16;

                // Triangle 1: CCW (Current, Next_X, Next_XY)
                indices.push(current);
                indices.push(next_x);
                indices.push(next_xy);

                // Triangle 2: CCW (Current, Next_XY, Next_Y)
                indices.push(current);
                indices.push(next_xy);
                indices.push(next_y);
            }
        }

        indices
    }

    pub fn allocate_buffers(
        device: &wgpu::Device,
        vertices: &[TerrainVertex],
        indices: &[u16],
    ) -> (wgpu::Buffer, wgpu::Buffer) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Terrain Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Terrain Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        (vertex_buffer, index_buffer)
    }
}
