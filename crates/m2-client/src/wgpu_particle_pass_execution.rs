#![allow(dead_code)]

use glam::Vec3;
use wgpu::util::DeviceExt;

use crate::wgpu_particle_billboard_renderer::ParticleVertex;
use crate::wgpu_particle_system_renderer::WgpuParticleSystemRenderer;

#[derive(Debug)]
#[repr(C)]
pub struct WgpuParticlePassExecution {
    pub pass_active: bool,
}

impl WgpuParticlePassExecution {
    pub fn new() -> Self {
        Self { pass_active: true }
    }

    pub fn collect_particle_vertices(
        systems: &[WgpuParticleSystemRenderer],
        camera_up: Vec3,
        camera_cross: Vec3,
        camera_view: Vec3,
    ) -> Vec<ParticleVertex> {
        let mut all_vertices = Vec::new();
        for sys in systems {
            if sys.active {
                let vertices = sys.build_vertex_buffer(camera_up, camera_cross, camera_view);
                all_vertices.extend(vertices);
            }
        }
        all_vertices
    }

    pub fn update_particle_vertex_buffer(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertices: &[ParticleVertex],
        existing_buffer: Option<wgpu::Buffer>,
        existing_buffer_capacity: u32,
    ) -> (Option<wgpu::Buffer>, u32) {
        if vertices.is_empty() {
            return (existing_buffer, existing_buffer_capacity);
        }

        let vertex_count = vertices.len() as u32;
        let size_in_bytes = vertices.len() * std::mem::size_of::<ParticleVertex>();
        
        let data_slice = unsafe {
            std::slice::from_raw_parts(vertices.as_ptr() as *const u8, size_in_bytes)
        };

        if let Some(buf) = &existing_buffer {
            if vertex_count <= existing_buffer_capacity {
                queue.write_buffer(buf, 0, data_slice);
                return (existing_buffer, existing_buffer_capacity);
            }
        }

        let new_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("MagicEffects Dynamic Vertex Buffer"),
            contents: data_slice,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        (Some(new_buffer), vertex_count)
    }

    pub fn execute_draw(
        render_pass: &mut wgpu::RenderPass,
        pipeline: &wgpu::RenderPipeline,
        vertex_buffer: &wgpu::Buffer,
        vertex_count: u32,
    ) {
        if vertex_count == 0 {
            return;
        }

        render_pass.set_pipeline(pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..vertex_count, 0..1);
    }
}
