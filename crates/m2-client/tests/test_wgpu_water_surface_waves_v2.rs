#[path = "../src/wgpu_water_surface_waves_v2.rs"]
mod wgpu_water_surface_waves_v2;

use wgpu_water_surface_waves_v2::{WaterSurfaceUniformsV2, WaterSurfaceVertexV2, WgpuWaterSurfaceWavesV2Renderer};
use std::mem;

#[test]
fn test_water_surface_uniforms_v2_layout() {
    assert_eq!(mem::size_of::<WaterSurfaceUniformsV2>(), 112);
    assert_eq!(mem::align_of::<WaterSurfaceUniformsV2>(), 16);
    
    // Create an instance to ensure zeroability works
    let uniforms = WaterSurfaceUniformsV2 {
        view_proj: [[0.0; 4]; 4],
        time: 0.0,
        water_depth_scale: 1.0,
        _pad0: [0.0; 2],
        dir1: [1.0, 0.0],
        dir2: [0.0, 1.0],
        sun_dir: [0.0, -1.0, 1.0],
        _pad1: 0.0,
    };
    
    assert_eq!(uniforms.time, 0.0);
}

#[test]
fn test_water_surface_vertex_v2_layout() {
    assert_eq!(mem::size_of::<WaterSurfaceVertexV2>(), 16);
    
    let desc = WaterSurfaceVertexV2::desc();
    assert_eq!(desc.array_stride, 16);
    assert_eq!(desc.step_mode, wgpu::VertexStepMode::Vertex);
    assert_eq!(desc.attributes.len(), 2);
    
    assert_eq!(desc.attributes[0].offset, 0);
    assert_eq!(desc.attributes[0].format, wgpu::VertexFormat::Float32x3);
    
    assert_eq!(desc.attributes[1].offset, 12);
    assert_eq!(desc.attributes[1].format, wgpu::VertexFormat::Uint32);
}

#[test]
fn test_wgpu_water_surface_waves_v2_headless() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()));
    
    if let Ok(adapter) = adapter {
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();
        
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Test Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        
        let renderer = WgpuWaterSurfaceWavesV2Renderer::new(
            &device,
            wgpu::TextureFormat::Rgba8Unorm,
            &bind_group_layout,
        );
        
        let uniforms = WaterSurfaceUniformsV2 {
            view_proj: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
            time: 1.0,
            water_depth_scale: 1.0,
            _pad0: [0.0; 2],
            dir1: [1.0, 0.0],
            dir2: [0.0, 1.0],
            sun_dir: [0.0, 1.0, 0.0],
            _pad1: 0.0,
        };
        
        renderer.update_uniforms(&queue, &uniforms);
        
        device.poll(wgpu::PollType::Wait { submission_index: None, timeout: None }).unwrap();
    }
}
