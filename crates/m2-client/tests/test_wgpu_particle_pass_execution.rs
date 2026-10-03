use glam::Vec3;

// We need to use m2_client's modules directly because wgpu_particle_system_renderer 
// inside the src file expects them to be in crate root or m2_client root
use m2_client::wgpu_particle_system_renderer::{WgpuParticleSystemRenderer, ParticleProperty};
use m2_client::mse_script_parser::MseEmitterProperty;
use m2_client::particle_instance::ParticleInstance;

#[path = "../src/wgpu_particle_pass_execution.rs"]
mod wgpu_particle_pass_execution {
    use glam::Vec3;
    use wgpu::util::DeviceExt;

    use m2_client::wgpu_particle_billboard_renderer::ParticleVertex;
    use m2_client::wgpu_particle_system_renderer::WgpuParticleSystemRenderer;

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

        pub fn execute_draw<'a>(
            render_pass: &mut wgpu::RenderPass<'a>,
            pipeline: &'a wgpu::RenderPipeline,
            vertex_buffer: &'a wgpu::Buffer,
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
}
use wgpu_particle_pass_execution::WgpuParticlePassExecution;

async fn get_device_queue() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: None,
            apply_limit_buckets: false,
        })
        .await
        .unwrap();

    adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await
        .unwrap()
}

#[tokio::test]
async fn test_empty_system_collection() {
    let systems: Vec<WgpuParticleSystemRenderer> = vec![];
    let vertices = WgpuParticlePassExecution::collect_particle_vertices(
        &systems,
        Vec3::Y,
        Vec3::X,
        Vec3::Z,
    );
    assert_eq!(vertices.len(), 0);
}

#[tokio::test]
async fn test_active_system_vertices_collection() {
    let mut sys = WgpuParticleSystemRenderer::new(
        ParticleProperty::default(),
        MseEmitterProperty::default(),
    );
    sys.active = true;
    
    // Add dummy active particle
    let mut p = ParticleInstance::new(1.0);
    p.age = 0.1; 
    sys.particles.push(p);

    let vertices = WgpuParticlePassExecution::collect_particle_vertices(
        &[sys],
        Vec3::Y,
        Vec3::X,
        Vec3::Z,
    );
    // Should be 4 vertices for one quad
    assert_eq!(vertices.len(), 4);
}

#[tokio::test]
async fn test_inactive_system_skipping() {
    let mut sys = WgpuParticleSystemRenderer::new(
        ParticleProperty::default(),
        MseEmitterProperty::default(),
    );
    sys.active = false;
    
    // Add dummy particle
    let mut p = ParticleInstance::new(1.0);
    p.age = 0.1;
    sys.particles.push(p);

    let vertices = WgpuParticlePassExecution::collect_particle_vertices(
        &[sys],
        Vec3::Y,
        Vec3::X,
        Vec3::Z,
    );
    // Sys is inactive, so no vertices should be generated
    assert_eq!(vertices.len(), 0);
}

#[tokio::test]
async fn test_buffer_update_empty_vertices() {
    let (device, queue) = get_device_queue().await;
    
    let vertices = vec![];
    let (buf, cap) = WgpuParticlePassExecution::update_particle_vertex_buffer(
        &device,
        &queue,
        &vertices,
        None,
        0,
    );
    
    assert!(buf.is_none());
    assert_eq!(cap, 0);
}

#[tokio::test]
async fn test_buffer_update_new_buffer_allocation() {
    let (device, queue) = get_device_queue().await;
    
    let mut sys = WgpuParticleSystemRenderer::new(
        ParticleProperty::default(),
        MseEmitterProperty::default(),
    );
    sys.active = true;
    let mut p = ParticleInstance::new(1.0);
    p.age = 0.1;
    sys.particles.push(p);

    let vertices = WgpuParticlePassExecution::collect_particle_vertices(
        &[sys],
        Vec3::Y,
        Vec3::X,
        Vec3::Z,
    );
    
    let (buf, cap) = WgpuParticlePassExecution::update_particle_vertex_buffer(
        &device,
        &queue,
        &vertices,
        None,
        0,
    );
    
    assert!(buf.is_some());
    assert_eq!(cap, 4);
}

#[tokio::test]
async fn test_buffer_update_reusing_existing_buffer() {
    let (device, queue) = get_device_queue().await;
    
    let mut sys = WgpuParticleSystemRenderer::new(
        ParticleProperty::default(),
        MseEmitterProperty::default(),
    );
    sys.active = true;
    let mut p = ParticleInstance::new(1.0);
    p.age = 0.1;
    sys.particles.push(p);

    let vertices = WgpuParticlePassExecution::collect_particle_vertices(
        &[sys],
        Vec3::Y,
        Vec3::X,
        Vec3::Z,
    );
    
    // Initial allocation
    let (buf1, cap1) = WgpuParticlePassExecution::update_particle_vertex_buffer(
        &device,
        &queue,
        &vertices,
        None,
        0,
    );
    assert_eq!(cap1, 4);
    assert!(buf1.is_some());

    // Reuse the existing buffer with same number of vertices
    let (buf2, cap2) = WgpuParticlePassExecution::update_particle_vertex_buffer(
        &device,
        &queue,
        &vertices,
        buf1,
        cap1,
    );
    assert_eq!(cap2, 4);
    assert!(buf2.is_some());
}
