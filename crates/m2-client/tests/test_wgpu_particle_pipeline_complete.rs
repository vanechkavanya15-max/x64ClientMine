#[path = "../src/wgpu_particle_pipeline_complete.rs"]
mod wgpu_particle_pipeline_complete;

use wgpu_particle_pipeline_complete::*;
use std::mem;

fn setup_device() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::default();
    
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        compatible_surface: None,
        force_fallback_adapter: false,
        apply_limit_buckets: false,
    }))
    .expect("Failed to find wgpu adapter");
    
    pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults(),
            memory_hints: wgpu::MemoryHints::Performance,
        }
    ))
    .expect("Failed to create wgpu device")
}

#[test]
fn test_vertex_layout_sizes() {
    assert_eq!(mem::size_of::<TPosition>(), 12, "TPosition should be 12 bytes");
    assert_eq!(mem::size_of::<TTextureCoordinate>(), 8, "TTextureCoordinate should be 8 bytes");
    assert_eq!(mem::size_of::<TDiffuse>(), 4, "TDiffuse should be 4 bytes");
    assert_eq!(mem::size_of::<TPTVertex>(), 20, "TPTVertex should be 20 bytes (12 + 8)");
    assert_eq!(mem::size_of::<TPDTVertex>(), 24, "TPDTVertex should be 24 bytes (12 + 4 + 8)");
}

#[test]
fn test_vertex_layout_offsets() {
    let dummy = TPDTVertex {
        position: TPosition { x: 1.0, y: 2.0, z: 3.0 },
        diffuse: 0xFFFFFFFF,
        tex_coord: TTextureCoordinate { u: 0.5, v: 0.5 },
    };
    
    let base_ptr = &dummy as *const _ as usize;
    let pos_ptr = &dummy.position as *const _ as usize;
    let diff_ptr = &dummy.diffuse as *const _ as usize;
    let tc_ptr = &dummy.tex_coord as *const _ as usize;
    
    assert_eq!(pos_ptr - base_ptr, 0, "Position offset must be 0");
    assert_eq!(diff_ptr - base_ptr, 12, "Diffuse offset must be 12");
    assert_eq!(tc_ptr - base_ptr, 16, "TextureCoord offset must be 16");
}

#[test]
fn test_pipeline_creation() {
    let (device, _) = setup_device();
    let format = wgpu::TextureFormat::Bgra8UnormSrgb;
    
    let pipeline = ParticlePipeline::new(&device, format);
    
    // Test if pipelines are successfully created and accessible
    let _additive = pipeline.additive_pipeline;
    let _alpha = pipeline.alpha_pipeline;
    let _cam_layout = pipeline.camera_bind_group_layout;
    let _tex_layout = pipeline.texture_bind_group_layout;
}

#[test]
fn test_vertex_descriptions() {
    let tpt_desc = TPTVertex::desc();
    assert_eq!(tpt_desc.array_stride, 20);
    assert_eq!(tpt_desc.attributes.len(), 2);
    assert_eq!(tpt_desc.attributes[0].format, wgpu::VertexFormat::Float32x3);
    assert_eq!(tpt_desc.attributes[1].format, wgpu::VertexFormat::Float32x2);
    
    let tpdt_desc = TPDTVertex::desc();
    assert_eq!(tpdt_desc.array_stride, 24);
    assert_eq!(tpdt_desc.attributes.len(), 3);
    assert_eq!(tpdt_desc.attributes[0].format, wgpu::VertexFormat::Float32x3);
    assert_eq!(tpdt_desc.attributes[1].format, wgpu::VertexFormat::Uint32);
    assert_eq!(tpdt_desc.attributes[2].format, wgpu::VertexFormat::Float32x2);
}

#[test]
fn test_bytemuck_pod_cast() {
    let vertices = vec![
        TPDTVertex {
            position: TPosition { x: 0.0, y: 0.0, z: 0.0 },
            diffuse: 0xFFFFFFFF,
            tex_coord: TTextureCoordinate { u: 0.0, v: 0.0 },
        },
        TPDTVertex {
            position: TPosition { x: 1.0, y: 1.0, z: 1.0 },
            diffuse: 0x80808080,
            tex_coord: TTextureCoordinate { u: 1.0, v: 1.0 },
        },
    ];
    
    let bytes: &[u8] = bytemuck::cast_slice(&vertices);
    assert_eq!(bytes.len(), 48, "Two TPDTVertices should be 48 bytes");
    
    // Check first diffuse color in bytes (little endian u32)
    assert_eq!(bytes[12], 0xFF);
    assert_eq!(bytes[13], 0xFF);
    assert_eq!(bytes[14], 0xFF);
    assert_eq!(bytes[15], 0xFF);
    
    // Check second diffuse color
    assert_eq!(bytes[36], 0x80);
    assert_eq!(bytes[37], 0x80);
    assert_eq!(bytes[38], 0x80);
    assert_eq!(bytes[39], 0x80);
}
