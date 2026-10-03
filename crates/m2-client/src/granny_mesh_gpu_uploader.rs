use wgpu::util::DeviceExt;

/// Granny PNT332 Vertex structure mirroring the C++ definition.
/// Rigid mesh vertex: Position (3x f32), Normal (3x f32), UV (2x f32).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrannyPnt332Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

/// Granny PWNT3432 Vertex structure mirroring the C++ definition.
/// Deformable mesh vertex: Position (3x f32), Weights (4x u8), Indices (4x u8), Normal (3x f32), UV (2x f32).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GrannyPwnt3432Vertex {
    pub position: [f32; 3],
    pub bone_weights: [u8; 4],
    pub bone_indices: [u8; 4],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

use crate::wgpu_granny_actor_renderer::GrannyActorVertex;

/// Output struct holding WGPU vertex and index buffers for a given mesh.
pub struct GpuMeshBuffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub vertex_count: u32,
    pub index_count: u32,
}

pub struct GrannyMeshGpuUploader;

impl GrannyMeshGpuUploader {
    /// Converts a slice of rigid `GrannyPnt332Vertex` into `GrannyActorVertex`.
    /// Sets the first bone weight to 1.0 (index 0) and the rest to 0.0.
    pub fn convert_pnt332_to_actor(vertices: &[GrannyPnt332Vertex]) -> Vec<GrannyActorVertex> {
        vertices
            .iter()
            .map(|v| GrannyActorVertex {
                position: v.position,
                normal: v.normal,
                tex_coord: v.uv,
                bone_indices: [0, 0, 0, 0],
                bone_weights: [1.0, 0.0, 0.0, 0.0],
            })
            .collect()
    }

    /// Converts a slice of deformable `GrannyPwnt3432Vertex` into `GrannyActorVertex`.
    /// Maps bone indices (u8 to u32) and bone weights (u8 out of 255 to f32 0.0-1.0).
    pub fn convert_pwnt3432_to_actor(vertices: &[GrannyPwnt3432Vertex]) -> Vec<GrannyActorVertex> {
        vertices
            .iter()
            .map(|v| {
                let bw = v.bone_weights;
                let inv_255 = 1.0 / 255.0;
                GrannyActorVertex {
                    position: v.position,
                    normal: v.normal,
                    tex_coord: v.uv,
                    bone_indices: [
                        v.bone_indices[0] as u32,
                        v.bone_indices[1] as u32,
                        v.bone_indices[2] as u32,
                        v.bone_indices[3] as u32,
                    ],
                    bone_weights: [
                        bw[0] as f32 * inv_255,
                        bw[1] as f32 * inv_255,
                        bw[2] as f32 * inv_255,
                        bw[3] as f32 * inv_255,
                    ],
                }
            })
            .collect()
    }

    /// Allocates and uploads `GrannyActorVertex` slice and indices slice into WGPU buffers.
    pub fn upload_mesh(
        device: &wgpu::Device,
        vertices: &[GrannyActorVertex],
        indices: &[u16],
        label: Option<&str>,
    ) -> Result<GpuMeshBuffers, &'static str> {
        if vertices.is_empty() || indices.is_empty() {
            return Err("Cannot upload empty mesh");
        }

        let vertex_label = label.map(|l| format!("{}_vertex_buffer", l));
        let index_label = label.map(|l| format!("{}_index_buffer", l));

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: vertex_label.as_deref(),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: index_label.as_deref(),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });

        Ok(GpuMeshBuffers {
            vertex_buffer,
            index_buffer,
            vertex_count: vertices.len() as u32,
            index_count: indices.len() as u32,
        })
    }
}
